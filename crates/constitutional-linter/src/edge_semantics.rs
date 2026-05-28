use serde::Deserialize;
use std::collections::{HashMap, HashSet};

/// Edge type in the dependency graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeType {
    /// Required runtime dependency (always linked)
    Runtime,
    /// Optional dependency (feature-gated)
    Optional,
    /// Dev-only dependency (tests/benches only)
    DevOnly,
    /// Build dependency (build.rs only)
    BuildOnly,
    /// Feature-gated behind a specific feature flag
    FeatureGated(String),
    /// Proc-macro dependency (compile-time only)
    ProcMacro,
}

impl EdgeType {
    pub fn is_constitutional(&self) -> bool {
        match self {
            EdgeType::Runtime => true,
            EdgeType::Optional => false,  // Optional deps are NOT constitutional
            EdgeType::DevOnly => false,
            EdgeType::BuildOnly => false,
            EdgeType::FeatureGated(_) => false,
            EdgeType::ProcMacro => false,
        }
    }

    pub fn name(&self) -> String {
        match self {
            EdgeType::Runtime => "runtime".to_string(),
            EdgeType::Optional => "optional".to_string(),
            EdgeType::DevOnly => "dev".to_string(),
            EdgeType::BuildOnly => "build".to_string(),
            EdgeType::FeatureGated(f) => format!("feature:{}", f),
            EdgeType::ProcMacro => "proc-macro".to_string(),
        }
    }
}

/// Edge from source to target with type information
#[derive(Debug, Clone)]
pub struct TypedEdge {
    pub source: String,
    pub target: String,
    pub edge_type: EdgeType,
}

/// Classify dependency edges from cargo metadata
pub fn classify_edges(
    packages: &[super::Package],
    _resolve: &Option<super::Resolve>,
) -> HashMap<String, Vec<TypedEdge>> {
    let mut typed_edges: HashMap<String, Vec<TypedEdge>> = HashMap::new();

    for pkg in packages {
        let edges = typed_edges.entry(pkg.name.clone()).or_default();
        for dep in &pkg.dependencies {
            let edge_type = if dep.optional {
                EdgeType::Optional
            } else {
                EdgeType::Runtime
            };
            edges.push(TypedEdge {
                source: pkg.name.clone(),
                target: dep.name.clone(),
                edge_type,
            });
        }
    }

    typed_edges
}

/// Filter edges to only constitutional (runtime, non-optional) edges
pub fn constitutional_edges_only(
    typed_edges: &HashMap<String, Vec<TypedEdge>>,
) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    for (source, edges) in typed_edges {
        for edge in edges {
            if edge.edge_type.is_constitutional() {
                graph.entry(source.clone())
                    .or_default()
                    .insert(edge.target.clone());
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_classification() {
        let runtime = EdgeType::Runtime;
        let optional = EdgeType::Optional;
        assert!(runtime.is_constitutional());
        assert!(!optional.is_constitutional());
    }

    #[test]
    fn test_edge_names() {
        assert_eq!(EdgeType::Runtime.name(), "runtime");
        assert_eq!(EdgeType::Optional.name(), "optional");
        assert_eq!(EdgeType::FeatureGated("async".into()).name(), "feature:async");
    }
}
