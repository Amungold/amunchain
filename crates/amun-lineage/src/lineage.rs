use amun_canonical_codec::CanonicalHasher;
use std::collections::{HashMap, HashSet};

pub const LINEAGE_DOMAIN: &[u8] = b"AMUN_LINEAGE_V1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CivilizationId(pub [u8; 32]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineageId(pub [u8; 32]);

impl LineageId {
    pub fn new_single(parent: &CivilizationId, child: &CivilizationId, epoch: u64) -> Self {
        let mut h = CanonicalHasher::with_domain(LINEAGE_DOMAIN);
        h.update(&parent.0);
        h.update(&child.0);
        h.update(&epoch.to_le_bytes());
        LineageId(h.finalize())
    }

    pub fn new_merger(parents: &[CivilizationId], child: &CivilizationId, epoch: u64) -> Self {
        let mut sorted: Vec<&CivilizationId> = parents.iter().collect();
        // Deterministic cross-platform byte ordering (not key-based)
        sorted.sort_by(|a, b| a.0.as_slice().cmp(b.0.as_slice()));
        let mut h = CanonicalHasher::with_domain(LINEAGE_DOMAIN);
        h.update(&(parents.len() as u64).to_le_bytes());
        for p in &sorted {
            h.update(&p.0);
        }
        h.update(&child.0);
        h.update(&epoch.to_le_bytes());
        LineageId(h.finalize())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvolutionMode {
    Superseding,
    Parallel,
    Experimental,
    Merger { parents: Vec<CivilizationId> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineageNode {
    pub civilization_id: CivilizationId,
    pub constitution_hash: [u8; 32],
    pub parents: HashSet<CivilizationId>,
    pub descendants: HashSet<CivilizationId>,
    pub origin_epoch: u64,
    pub status: CivilizationStatus,
    pub evolution_mode: Option<EvolutionMode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CivilizationStatus {
    Active,
    Superseded,
    Extinct,
    Frozen,
}

#[derive(Debug, Clone)]
pub struct LineageGraph {
    pub nodes: HashMap<CivilizationId, LineageNode>,
    pub root: Option<CivilizationId>,
    pub heads: HashSet<CivilizationId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationError {
    CycleDetected,
    SelfReference,
    AlreadyExists,
    ParentNotFound,
}

impl Default for LineageGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl LineageGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root: None,
            heads: HashSet::new(),
        }
    }

    pub fn register_descendant(
        &mut self,
        parent_id: CivilizationId,
        child_id: CivilizationId,
        child_constitution_hash: [u8; 32],
        epoch: u64,
        mode: EvolutionMode,
    ) -> Result<LineageId, RegistrationError> {
        if child_id == parent_id {
            return Err(RegistrationError::SelfReference);
        }
        if self.nodes.contains_key(&child_id) {
            return Err(RegistrationError::AlreadyExists);
        }
        if !self.nodes.contains_key(&parent_id) {
            return Err(RegistrationError::ParentNotFound);
        }

        // Cycle prevention (non-reflexive ancestor check)
        if self.is_ancestor_nonreflexive(&child_id, &parent_id) {
            return Err(RegistrationError::CycleDetected);
        }

        let lineage_id = match &mode {
            EvolutionMode::Merger { parents } => {
                for pid in parents {
                    if self.is_ancestor_nonreflexive(&child_id, pid) {
                        return Err(RegistrationError::CycleDetected);
                    }
                }
                LineageId::new_merger(parents, &child_id, epoch)
            }
            _ => LineageId::new_single(&parent_id, &child_id, epoch),
        };

        match &mode {
            EvolutionMode::Superseding => {
                self.heads.remove(&parent_id);
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.descendants.insert(child_id);
                    parent.status = CivilizationStatus::Superseded;
                }
            }
            EvolutionMode::Parallel | EvolutionMode::Experimental => {
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.descendants.insert(child_id);
                }
            }
            EvolutionMode::Merger { parents } => {
                for pid in parents {
                    self.heads.remove(pid);
                    if let Some(p) = self.nodes.get_mut(pid) {
                        p.descendants.insert(child_id);
                        p.status = CivilizationStatus::Superseded;
                    }
                }
            }
        }

        let mut child_parents = HashSet::new();
        match &mode {
            EvolutionMode::Merger { parents } => {
                for pid in parents {
                    child_parents.insert(*pid);
                }
            }
            _ => {
                child_parents.insert(parent_id);
            }
        }

        let child = LineageNode {
            civilization_id: child_id,
            constitution_hash: child_constitution_hash,
            parents: child_parents,
            descendants: HashSet::new(),
            origin_epoch: epoch,
            status: CivilizationStatus::Active,
            evolution_mode: Some(mode),
        };
        self.nodes.insert(child_id, child);
        self.heads.insert(child_id);

        Ok(lineage_id)
    }

    pub fn register_genesis(
        &mut self,
        id: CivilizationId,
        constitution_hash: [u8; 32],
        epoch: u64,
    ) {
        let node = LineageNode {
            civilization_id: id,
            constitution_hash,
            parents: HashSet::new(),
            descendants: HashSet::new(),
            origin_epoch: epoch,
            status: CivilizationStatus::Active,
            evolution_mode: None,
        };
        self.nodes.insert(id, node);
        self.root = Some(id);
        self.heads.insert(id);
    }

    /// Non-reflexive ancestor check: returns false if ancestor == descendant.
    fn is_ancestor_nonreflexive(
        &self,
        ancestor: &CivilizationId,
        descendant: &CivilizationId,
    ) -> bool {
        if ancestor == descendant {
            return false;
        }
        let mut visited = HashSet::new();
        let mut stack = vec![*descendant];
        while let Some(cid) = stack.pop() {
            if visited.insert(cid) {
                if let Some(node) = self.nodes.get(&cid) {
                    for parent in &node.parents {
                        if parent == ancestor {
                            return true;
                        }
                        stack.push(*parent);
                    }
                }
            }
        }
        false
    }
}
