use crate::{ObligationKind, ObligationRegistry, ObligationSeverity};
use std::collections::HashMap;

/// Certificate confirming that Article I of the N47 constitution is fully
/// implemented and operational.
///
/// Issued only when all constitutional rules of Article I are satisfied:
/// - All 22 obligations registered
/// - Dependency graph valid and cycle-free
/// - Derived obligations terminate in Primary obligations
/// - Registry is frozen and immutable
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleICertificate {
    pub certificate_id: String,
    pub obligations_registered: usize,
    pub dependency_graph_valid: bool,
    pub cycle_free: bool,
    pub registry_frozen: bool,
    pub total_primary: usize,
    pub total_derived: usize,
    pub critical_count: usize,
    pub major_count: usize,
    pub minor_count: usize,
    pub advisory_count: usize,
    pub issued_at: u64,
}

impl ArticleICertificate {
    /// Try to issue an Article I certificate by inspecting the registry.
    ///
    /// Returns `None` if the registry is not frozen or the graph is invalid.
    pub fn issue(registry: &ObligationRegistry, issued_at: u64) -> Option<Self> {
        if !registry.is_frozen() {
            return None;
        }

        let total = registry.total();
        if total < 22 {
            return None; // constitutional minimum not met
        }

        // Collect statistics
        let mut primaries = 0;
        let mut derived = 0;
        let mut critical = 0;
        let mut major = 0;
        let mut minor = 0;
        let mut advisory = 0;
        let mut kinds = HashMap::new();

        for obl in registry.all_obligations() {
            match obl.kind {
                ObligationKind::Primary => primaries += 1,
                ObligationKind::Derived => derived += 1,
            }
            match obl.severity {
                ObligationSeverity::Critical => critical += 1,
                ObligationSeverity::Major => major += 1,
                ObligationSeverity::Minor => minor += 1,
                ObligationSeverity::Advisory => advisory += 1,
            }
            kinds.insert(obl.id.clone(), obl.kind);
        }

        // Build a temporary graph from the registry and validate
        let mut graph = crate::DependencyGraph::new();
        for obl in registry.all_obligations() {
            graph.add_node(obl.id.clone());
            for dep in &obl.depends_on {
                graph.add_edge(obl.id.clone(), dep.clone());
            }
        }

        let cycle_free = !graph.has_cycles();
        let derived_ok = graph.validate_derived_terminate_in_primary(&kinds).is_ok();
        let graph_valid = cycle_free && derived_ok;

        if !graph_valid {
            return None;
        }

        let cert = Self {
            certificate_id: "N47.1-CERT-001".into(),
            obligations_registered: total,
            dependency_graph_valid: graph_valid,
            cycle_free,
            registry_frozen: true,
            total_primary: primaries,
            total_derived: derived,
            critical_count: critical,
            major_count: major,
            minor_count: minor,
            advisory_count: advisory,
            issued_at,
        };

        Some(cert)
    }
}
