use amun_quorum_certificate::QuorumCertificate;
use amun_chain_position::ChainPosition;
use std::collections::{BTreeMap, HashSet};

/// Maximum ancestry traversal depth to prevent infinite loops
pub const MAX_DAG_DEPTH: usize = 50_000;

#[derive(Debug, Clone)]
pub struct BlockNode {
    pub block_hash: [u8; 32],
    pub parent_hash: Option<[u8; 32]>,
    pub position: ChainPosition,
    pub round: u64,
    pub justify_qc: Option<QuorumCertificate>,
    pub state_root: [u8; 32],
    pub committed: bool,
}

impl BlockNode {
    pub fn new(
        block_hash: [u8; 32],
        parent_hash: Option<[u8; 32]>,
        position: ChainPosition,
        round: u64,
        justify_qc: Option<QuorumCertificate>,
        state_root: [u8; 32],
    ) -> Self {
        Self { block_hash, parent_hash, position, round, justify_qc, state_root, committed: false }
    }

    /// Check if this block is a descendant of `ancestor`
    /// Returns false for self (a block is not a descendant of itself)
    /// Depth-bounded with cycle protection
    pub fn is_descendant_of(&self, ancestor: &[u8; 32], dag: &BlockDAG) -> bool {
        let mut current = self.parent_hash;
        let mut visited = HashSet::new();
        visited.insert(self.block_hash);
        let mut depth = 0;

        while let Some(parent_hash) = current {
            if depth >= MAX_DAG_DEPTH {
                return false; // Depth limit exceeded - safety bound
            }
            if !visited.insert(parent_hash) {
                return false; // Cycle detected
            }
            if &parent_hash == ancestor {
                return true; // Found ancestor in parent chain
            }
            current = dag.get_block(&parent_hash).and_then(|b| b.parent_hash);
            depth += 1;
        }
        false
    }

    pub fn ancestor_chain(&self, dag: &BlockDAG, depth: usize) -> Vec<[u8; 32]> {
        let mut chain = Vec::new();
        let mut current = Some(self.block_hash);
        let mut visited = HashSet::new();
        let effective_depth = depth.min(MAX_DAG_DEPTH);

        while let Some(hash) = current {
            if !visited.insert(hash) || chain.len() >= effective_depth {
                break;
            }
            chain.push(hash);
            current = dag.get_block(&hash).and_then(|b| b.parent_hash);
        }
        chain
    }
}

#[derive(Debug, Clone)]
pub struct BlockDAG {
    pub blocks: BTreeMap<[u8; 32], BlockNode>,
    pub children_index: BTreeMap<[u8; 32], HashSet<[u8; 32]>>,
    pub height_index: BTreeMap<u64, HashSet<[u8; 32]>>,
    pub round_index: BTreeMap<u64, HashSet<[u8; 32]>>,
    pub last_committed: Option<[u8; 32]>,
    pub finalized_height: u64,
    pub canonical_spine: Vec<[u8; 32]>,
    pub genesis_hash: [u8; 32],
}

impl BlockDAG {
    pub fn new(genesis_hash: [u8; 32]) -> Self {
        let genesis = BlockNode {
            block_hash: genesis_hash, parent_hash: None,
            position: ChainPosition::new(0, 0), round: 0,
            justify_qc: None, state_root: genesis_hash, committed: true,
        };
        let mut blocks = BTreeMap::new();
        blocks.insert(genesis_hash, genesis);
        let mut height_index = BTreeMap::new();
        height_index.insert(0, { let mut s = HashSet::new(); s.insert(genesis_hash); s });
        let mut round_index = BTreeMap::new();
        round_index.insert(0, { let mut s = HashSet::new(); s.insert(genesis_hash); s });

        Self {
            blocks, children_index: BTreeMap::new(), height_index, round_index,
            last_committed: Some(genesis_hash), finalized_height: 0,
            canonical_spine: vec![genesis_hash], genesis_hash,
        }
    }

    pub fn add_block(&mut self, block: BlockNode) -> Result<(), &'static str> {
        let hash = block.block_hash;
        if self.blocks.contains_key(&hash) {
            return Err("Block already exists");
        }
        if let Some(parent_hash) = block.parent_hash {
            if !self.blocks.contains_key(&parent_hash) {
                return Err("Parent block not found");
            }
            self.children_index
                .entry(parent_hash)
                .or_insert_with(HashSet::new)
                .insert(hash);
        }
        let height = block.position.sequence;
        let round = block.round;
        self.height_index.entry(height).or_insert_with(HashSet::new).insert(hash);
        self.round_index.entry(round).or_insert_with(HashSet::new).insert(hash);
        self.blocks.insert(hash, block);
        Ok(())
    }

    pub fn get_block(&self, hash: &[u8; 32]) -> Option<&BlockNode> {
        self.blocks.get(hash)
    }

    pub fn get_children(&self, parent_hash: &[u8; 32]) -> Vec<&BlockNode> {
        self.children_index
            .get(parent_hash)
            .map(|children| children.iter().filter_map(|h| self.blocks.get(h)).collect())
            .unwrap_or_default()
    }

    pub fn blocks_at_height(&self, height: u64) -> Vec<&BlockNode> {
        self.height_index
            .get(&height)
            .map(|hashes| hashes.iter().filter_map(|h| self.blocks.get(h)).collect())
            .unwrap_or_default()
    }

    pub fn blocks_at_round(&self, round: u64) -> Vec<&BlockNode> {
        self.round_index
            .get(&round)
            .map(|hashes| hashes.iter().filter_map(|h| self.blocks.get(h)).collect())
            .unwrap_or_default()
    }

    pub fn commit_block(&mut self, hash: &[u8; 32]) {
        if let Some(block) = self.blocks.get_mut(hash) {
            block.committed = true;
            self.last_committed = Some(*hash);
        }
    }

    /// Update canonical spine with invariant assertions
    pub fn update_canonical_spine(&mut self, finalized_block: [u8; 32]) {
        let mut spine = vec![finalized_block];
        let mut current = Some(finalized_block);
        let mut visited = HashSet::new();
        visited.insert(finalized_block);
        let mut depth = 0;

        while let Some(hash) = current {
            if depth >= MAX_DAG_DEPTH { break; }
            if let Some(block) = self.blocks.get(&hash) {
                if let Some(parent) = block.parent_hash {
                    if !visited.insert(parent) { break; }
                    spine.push(parent);
                    current = Some(parent);
                } else {
                    break;
                }
            } else {
                break;
            }
            depth += 1;
        }
        spine.reverse();
        self.canonical_spine = spine;

        // Invariant assertions
        debug_assert!(
            self.blocks.contains_key(&self.genesis_hash),
            "DAG invariant: genesis must always exist"
        );
        debug_assert!(
            self.canonical_spine.first() == Some(&self.genesis_hash),
            "DAG invariant: canonical spine must start at genesis"
        );
    }

    pub fn finalize_and_prune(&mut self, height: u64, finalized_block: [u8; 32]) {
        debug_assert!(height > self.finalized_height, "Finalized height must advance");
        self.finalized_height = height;
        self.update_canonical_spine(finalized_block);
        self.prune_safe();

        // Post-pruning invariant
        debug_assert!(
            self.blocks.contains_key(&self.genesis_hash),
            "DAG invariant: genesis must survive pruning"
        );
    }

    fn prune_safe(&mut self) {
        let spine_set: HashSet<[u8; 32]> = self.canonical_spine.iter().cloned().collect();

        // Collect blocks to remove
        let to_remove: Vec<[u8; 32]> = self.blocks
            .iter()
            .filter(|(_, b)| {
                b.position.sequence < self.finalized_height
                    && b.position.sequence > 0
                    && !spine_set.contains(&b.block_hash)
            })
            .map(|(h, _)| *h)
            .collect();

        // O(1) lookup set
        let remove_set: HashSet<[u8; 32]> = to_remove.iter().cloned().collect();

        for hash in &to_remove {
            self.blocks.remove(hash);
        }

        // Clean indexes with O(1) contains
        self.height_index.retain(|h, _| *h >= self.finalized_height || *h == 0);
        self.round_index.retain(|_, hashes| {
            hashes.retain(|h| !remove_set.contains(h));
            !hashes.is_empty()
        });
        self.children_index.retain(|_, children| {
            children.retain(|h| !remove_set.contains(h));
            !children.is_empty()
        });
        // Clean orphaned parent entries
        self.children_index.retain(|parent, _| self.blocks.contains_key(parent));
    }

    pub fn highest_committed_height(&self) -> u64 {
        self.last_committed_block()
            .map(|b| b.position.sequence)
            .unwrap_or(0)
    }

    pub fn last_committed_block(&self) -> Option<&BlockNode> {
        self.last_committed.and_then(|h| self.blocks.get(&h))
    }

    pub fn is_on_canonical_spine(&self, hash: &[u8; 32]) -> bool {
        self.canonical_spine.contains(hash)
    }
}
