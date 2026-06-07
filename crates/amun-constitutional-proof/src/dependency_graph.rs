use std::collections::{HashMap, HashSet, VecDeque};

use crate::{ObligationId, RegistryError};

#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    nodes: HashSet<ObligationId>,
    edges: HashMap<ObligationId, Vec<ObligationId>>,
    reverse_edges: HashMap<ObligationId, Vec<ObligationId>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, id: ObligationId) {
        self.nodes.insert(id.clone());
        self.edges.entry(id.clone()).or_default();
        self.reverse_edges.entry(id).or_default();
    }

    pub fn add_edge(&mut self, from: ObligationId, to: ObligationId) {
        self.nodes.insert(from.clone());
        self.nodes.insert(to.clone());
        self.edges.entry(from.clone()).or_default().push(to.clone());
        self.reverse_edges.entry(to).or_default().push(from);
    }

    pub fn has_cycles(&self) -> bool {
        self.find_cycle().is_some()
    }

    pub fn topological_sort(&self) -> Result<Vec<ObligationId>, RegistryError> {
        let mut in_degree: HashMap<ObligationId, usize> = HashMap::new();
        for node in &self.nodes {
            in_degree.insert(node.clone(), 0);
        }
        for froms in self.reverse_edges.values() {
            for from in froms {
                *in_degree.get_mut(from).unwrap() += 1;
            }
        }

        let mut queue: VecDeque<ObligationId> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(node, _)| node.clone())
            .collect();

        let mut sorted = Vec::new();
        while let Some(node) = queue.pop_front() {
            sorted.push(node.clone());
            if let Some(dependents) = self.reverse_edges.get(&node) {
                for dependent in dependents {
                    if let Some(deg) = in_degree.get_mut(dependent) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(dependent.clone());
                        }
                    }
                }
            }
        }

        if sorted.len() != self.nodes.len() {
            if let Some(cycle_node) = self.find_cycle() {
                return Err(RegistryError::CircularDependency(cycle_node));
            }
            return Err(RegistryError::CircularDependency(
                ObligationId::new(crate::ObligationNamespace::Safety, 0),
            ));
        }

        Ok(sorted)
    }

    pub fn validate_derived_terminate_in_primary(
        &self,
        kinds: &HashMap<ObligationId, crate::ObligationKind>,
    ) -> Result<(), RegistryError> {
        for node in &self.nodes {
            let kind = kinds.get(node).copied().unwrap_or(crate::ObligationKind::Primary);
            if kind == crate::ObligationKind::Derived
                && !self.has_primary_ancestor(node, kinds)
            {
                return Err(RegistryError::DerivedNotTerminatingInPrimary(node.clone()));
            }
        }
        Ok(())
    }

    pub fn all_dependencies_exist(&self) -> Result<(), RegistryError> {
        for (from, tos) in &self.edges {
            for to in tos {
                if !self.nodes.contains(to) {
                    return Err(RegistryError::MissingDependency(from.clone(), to.clone()));
                }
            }
        }
        Ok(())
    }

    fn find_cycle(&self) -> Option<ObligationId> {
        let mut white = self.nodes.clone();
        let mut gray = HashSet::new();
        let mut black = HashSet::new();

        while let Some(start) = white.iter().next().cloned() {
            if self.dfs_visit(&start, &mut white, &mut gray, &mut black) {
                return Some(start);
            }
        }
        None
    }

    fn dfs_visit(
        &self,
        node: &ObligationId,
        white: &mut HashSet<ObligationId>,
        gray: &mut HashSet<ObligationId>,
        black: &mut HashSet<ObligationId>,
    ) -> bool {
        white.remove(node);
        gray.insert(node.clone());

        if let Some(neighbors) = self.edges.get(node) {
            for neighbor in neighbors {
                if black.contains(neighbor) {
                    continue;
                }
                if gray.contains(neighbor) {
                    return true;
                }
                if self.dfs_visit(neighbor, white, gray, black) {
                    return true;
                }
            }
        }

        gray.remove(node);
        black.insert(node.clone());
        false
    }

    fn has_primary_ancestor(
        &self,
        node: &ObligationId,
        kinds: &HashMap<ObligationId, crate::ObligationKind>,
    ) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![node.clone()];

        while let Some(current) = stack.pop() {
            if !visited.insert(current.clone()) {
                continue;
            }
            let kind = kinds.get(&current).copied().unwrap_or(crate::ObligationKind::Primary);
            if kind == crate::ObligationKind::Primary {
                return true;
            }
            if let Some(deps) = self.edges.get(&current) {
                for dep in deps {
                    stack.push(dep.clone());
                }
            }
        }
        false
    }
}
