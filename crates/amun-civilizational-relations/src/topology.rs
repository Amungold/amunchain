use amun_lineage::lineage::LineageGraph;
use std::collections::HashMap;

/// Civilizational topology - the graph of known civilizations and their relations.
#[derive(Debug, Clone)]
pub struct CivilizationalTopology {
    pub known_civilizations: HashMap<[u8; 32], CivilizationNode>,
    pub lineage_graph: LineageGraph,
}

#[derive(Debug, Clone)]
pub struct CivilizationNode {
    pub identity_hash: [u8; 32],
    pub relation_to_local: super::relation::CivilizationalRelation,
}

impl CivilizationalTopology {
    pub fn new(lineage: LineageGraph) -> Self {
        Self {
            known_civilizations: HashMap::new(),
            lineage_graph: lineage,
        }
    }
}
