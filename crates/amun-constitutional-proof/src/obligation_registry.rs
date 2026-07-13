use std::collections::BTreeMap;

use crate::{
    DependencyGraph, ObligationId, ObligationKind, ObligationSeverity, ObligationStatus,
    ProofObligation, RegistryError,
};

#[derive(Debug, Clone)]
pub struct ObligationRegistry {
    obligations: BTreeMap<ObligationId, ProofObligation>,
    graph: DependencyGraph,
    frozen: bool,
}

impl ObligationRegistry {
    pub fn new() -> Self {
        Self {
            obligations: BTreeMap::new(),
            graph: DependencyGraph::new(),
            frozen: false,
        }
    }

    pub fn register(&mut self, obligation: ProofObligation) -> Result<(), RegistryError> {
        if self.frozen {
            return Err(RegistryError::RegistryFrozen);
        }
        if self.obligations.contains_key(&obligation.id) {
            return Err(RegistryError::DuplicateId(obligation.id.clone()));
        }
        for dep in &obligation.depends_on {
            if !self.obligations.contains_key(dep) {
                return Err(RegistryError::MissingDependency(
                    obligation.id.clone(),
                    dep.clone(),
                ));
            }
        }

        self.graph.add_node(obligation.id.clone());
        for dep in &obligation.depends_on {
            self.graph.add_edge(obligation.id.clone(), dep.clone());
        }

        self.graph
            .all_dependencies_exist()
            .map_err(|_| RegistryError::CircularDependency(obligation.id.clone()))?;

        let kinds = self.collect_kinds();
        self.graph.validate_derived_terminate_in_primary(&kinds)?;

        if self.graph.has_cycles() {
            self.graph = self.build_graph_from_registry();
            return Err(RegistryError::CircularDependency(obligation.id.clone()));
        }

        self.obligations.insert(obligation.id.clone(), obligation);
        Ok(())
    }

    pub fn freeze(&mut self) -> Result<(), RegistryError> {
        if self.frozen {
            return Err(RegistryError::RegistryFrozen);
        }
        for obl in self.obligations.values_mut() {
            obl.status = ObligationStatus::Frozen;
        }
        self.frozen = true;
        Ok(())
    }

    pub fn get(&self, id: &ObligationId) -> Option<&ProofObligation> {
        self.obligations.get(id)
    }

    pub fn all_obligations(&self) -> impl Iterator<Item = &ProofObligation> {
        self.obligations.values()
    }

    pub fn by_namespace(&self, ns: ObligationSeverity) -> Vec<&ProofObligation> {
        self.obligations
            .values()
            .filter(|o| o.severity == ns)
            .collect()
    }

    pub fn by_severity(&self, severity: ObligationSeverity) -> Vec<&ProofObligation> {
        self.obligations
            .values()
            .filter(|o| o.severity == severity)
            .collect()
    }

    pub fn by_phase(&self, phase: &str) -> Vec<&ProofObligation> {
        self.obligations
            .values()
            .filter(|o| o.phase == phase)
            .collect()
    }

    pub fn total(&self) -> usize {
        self.obligations.len()
    }

    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    fn collect_kinds(&self) -> std::collections::HashMap<ObligationId, ObligationKind> {
        self.obligations
            .iter()
            .map(|(id, obl)| (id.clone(), obl.kind))
            .collect()
    }

    fn build_graph_from_registry(&self) -> DependencyGraph {
        let mut g = DependencyGraph::new();
        for obl in self.obligations.values() {
            g.add_node(obl.id.clone());
            for dep in &obl.depends_on {
                g.add_edge(obl.id.clone(), dep.clone());
            }
        }
        g
    }
}

impl Default for ObligationRegistry {
    fn default() -> Self {
        Self::new()
    }
}
