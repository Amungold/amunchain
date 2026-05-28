//! ExecutionDAG — causally constrained, scheduler-oblivious.
//!
//! The ExecutionDAG represents the PARTIAL ORDER of execution.
//! It does NOT represent truth ordering, validity, or admissibility.
//!
//! INVARIANT: Different schedulers may traverse the DAG differently.
//! The constitutional kernel derives identical truth regardless.
//!
//! The DAG is a SCHEDULING ARTIFACT, not a truth artifact.

use amun_constitutional::prelude::*;
use crate::execution_vertex::ExecutionVertex;
use crate::execution_dependency::ExecutionDependencyType;

/// An edge in the execution DAG — a dependency between two vertices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionEdge {
    /// The vertex that depends on another.
    pub from_vertex: u64,
    /// The vertex that is depended upon.
    pub to_vertex: u64,
    /// The type of execution dependency.
    pub dependency_type: ExecutionDependencyType,
}

/// The distributed execution DAG.
///
/// This DAG represents WHAT must execute before WHAT.
/// It does NOT represent WHY anything is constitutionally valid.
/// It is SCHEDULER-OBLIVIOUS: any valid topological ordering is acceptable.
#[derive(Debug, Clone, Default)]
pub struct ExecutionDAG {
    /// Vertices in the DAG.
    vertices: Vec<ExecutionVertex>,
    /// Edges representing execution dependencies.
    edges: Vec<ExecutionEdge>,
}

impl ExecutionDAG {
    pub fn new() -> Self {
        Self { vertices: Vec::new(), edges: Vec::new() }
    }

    /// Add a vertex to the DAG.
    pub fn add_vertex(&mut self, vertex: ExecutionVertex) {
        self.vertices.push(vertex);
    }

    /// Add an execution dependency edge.
    pub fn add_edge(&mut self, from: u64, to: u64, dep_type: ExecutionDependencyType) {
        self.edges.push(ExecutionEdge { from_vertex: from, to_vertex: to, dependency_type: dep_type });
    }

    /// Get vertices that have no unmet mandatory dependencies.
    /// These are ready for execution regardless of scheduler choice.
    pub fn ready_vertices(&self) -> Vec<&ExecutionVertex> {
        let completed: Vec<u64> = self.vertices.iter()
            .filter(|v| !v.produced_artifacts.is_empty())
            .map(|v| v.vertex_id)
            .collect();

        self.vertices.iter()
            .filter(|v| {
                // Vertex is ready if all mandatory dependencies are completed
                let deps: Vec<&ExecutionEdge> = self.edges.iter()
                    .filter(|e| e.from_vertex == v.vertex_id && e.dependency_type.is_mandatory())
                    .collect();
                deps.iter().all(|e| completed.contains(&e.to_vertex))
            })
            .collect()
    }

    /// Get the partial order: vertices in topological order.
    /// This is ONE valid order — different schedulers may produce different orders.
    /// Constitutional truth MUST be identical regardless of the order chosen.
    pub fn topological_order(&self) -> Vec<&ExecutionVertex> {
        // Kahn's algorithm — produces one valid topological order
        let mut in_degree: Vec<usize> = vec![0; self.vertices.len()];
        for edge in &self.edges {
            if let Some(pos) = self.vertices.iter().position(|v| v.vertex_id == edge.from_vertex) {
                in_degree[pos] += 1;
            }
        }

        let mut queue: Vec<usize> = in_degree.iter()
            .enumerate()
            .filter(|(_, &d)| d == 0)
            .map(|(i, _)| i)
            .collect();

        let mut order: Vec<&ExecutionVertex> = Vec::new();
        while let Some(idx) = queue.pop() {
            order.push(&self.vertices[idx]);
            for edge in &self.edges {
                if edge.to_vertex == self.vertices[idx].vertex_id {
                    if let Some(pos) = self.vertices.iter().position(|v| v.vertex_id == edge.from_vertex) {
                        in_degree[pos] -= 1;
                        if in_degree[pos] == 0 {
                            queue.push(pos);
                        }
                    }
                }
            }
        }
        order
    }

    pub fn vertex_count(&self) -> usize { self.vertices.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_vertex::VertexType;

    #[test]
    fn test_empty_dag() {
        let dag = ExecutionDAG::new();
        assert_eq!(dag.vertex_count(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut dag = ExecutionDAG::new();
        dag.add_vertex(ExecutionVertex::new(1, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32]));
        assert_eq!(dag.vertex_count(), 1);
    }

    #[test]
    fn test_topological_order() {
        let mut dag = ExecutionDAG::new();
        let v1 = ExecutionVertex::new(1, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32])
            .with_artifact([0x01; 32]);
        let v2 = ExecutionVertex::new(2, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32])
            .with_dependency(1);
        let v3 = ExecutionVertex::new(3, VertexType::Verification, 200, [0xAB; 32], [0xBC; 32])
            .with_dependency(2);

        dag.add_vertex(v1);
        dag.add_vertex(v2);
        dag.add_vertex(v3);
        dag.add_edge(2, 1, ExecutionDependencyType::RequiresArtifact);
        dag.add_edge(3, 2, ExecutionDependencyType::RequiresArtifact);

        let order = dag.topological_order();
        assert_eq!(order.len(), 3);
        // v1 should come before v2, v2 before v3
        let pos1 = order.iter().position(|v| v.vertex_id == 1).unwrap();
        let pos2 = order.iter().position(|v| v.vertex_id == 2).unwrap();
        let pos3 = order.iter().position(|v| v.vertex_id == 3).unwrap();
        assert!(pos1 < pos2);
        assert!(pos2 < pos3);
    }

    #[test]
    fn test_dag_is_scheduler_oblivious() {
        // The same DAG can produce different valid topological orders.
        // Constitutional truth must be identical regardless.
        let mut dag = ExecutionDAG::new();
        let v1 = ExecutionVertex::new(1, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32])
            .with_artifact([0x01; 32]);
        let v2 = ExecutionVertex::new(2, VertexType::StateTransition, 200, [0xAB; 32], [0xBC; 32])
            .with_artifact([0x02; 32]);
        // v1 and v2 have no dependencies on each other — they can execute in any order
        dag.add_vertex(v1);
        dag.add_vertex(v2);

        let order = dag.topological_order();
        assert_eq!(order.len(), 2);
        // Both orders are valid — this test just verifies the DAG doesn't crash
    }
}
