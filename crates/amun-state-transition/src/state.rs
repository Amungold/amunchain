use crate::write_set::{WriteSet, StateOperation};
use crate::receipt::ExecutionReceipt;
use amun_canonical::{CanonicalEncoder, SchemaVersion};
use amun_deterministic_allocator::DeterministicMap;
use amun_chain_position::ChainPosition;
use blake3::Hasher;

pub struct StateMachine {
    pub state: DeterministicMap<[u8; 32], Vec<u8>>,
    pub current_root: [u8; 32],
    pub genesis_root: [u8; 32],
    pub execution_version: u64,
    pub epoch_seal_hash: Option<[u8; 32]>,
}

/// Owned overlay. Contains only the delta operations.
pub struct StateOverlay {
    delta: Vec<(StateOperation, bool)>,
}

impl StateOverlay {
    pub fn new() -> Self { Self { delta: Vec::new() } }

    pub fn put(&mut self, key: [u8; 32], value: Vec<u8>) {
        self.delta.retain(|(op, _)| match op {
            StateOperation::Put { key: k, .. } | StateOperation::Delete { key: k } => *k != key,
        });
        self.delta.push((StateOperation::Put { key, value }, false));
    }

    pub fn delete(&mut self, key: [u8; 32]) {
        self.delta.retain(|(op, _)| match op {
            StateOperation::Put { key: k, .. } | StateOperation::Delete { key: k } => *k != key,
        });
        self.delta.push((StateOperation::Delete { key }, false));
    }

    pub fn is_empty(&self) -> bool { self.delta.is_empty() }

    pub fn to_writeset(&self) -> WriteSet {
        WriteSet::from_overlay(&self.delta)
    }

    pub fn commit_to(self, target: &mut DeterministicMap<[u8; 32], Vec<u8>>) {
        for (op, _) in &self.delta {
            match op {
                StateOperation::Put { key, value } => { let _ = target.insert(*key, value.clone()); }
                StateOperation::Delete { key } => { target.remove(key); }
            }
        }
    }
}

/// Stateless transition output.
pub struct TransitionOutput {
    pub receipt: ExecutionReceipt,
    pub overlay: StateOverlay,
    pub new_root: [u8; 32],
}

impl StateMachine {
    pub fn new(genesis_root: [u8; 32], execution_version: u64) -> Self {
        Self {
            state: DeterministicMap::new(), current_root: genesis_root, genesis_root,
            execution_version, epoch_seal_hash: None,
        }
    }

    pub fn set_epoch_seal(&mut self, seal_hash: [u8; 32]) {
        self.epoch_seal_hash = Some(seal_hash);
    }

    /// STATELESS transition function. Takes an explicit state reference,
    /// does NOT mutate self. Can be called on any StateMachine instance.
    pub fn execute_transition(
        state: &DeterministicMap<[u8; 32], Vec<u8>>,
        execution_version: u64,
        epoch_seal_hash: Option<[u8; 32]>,
        position: ChainPosition,
        tx_data: &[u8],
        gas_limit: u64,
    ) -> (ExecutionReceipt, StateOverlay) {
        let from_root = Self::compute_root(state, execution_version, epoch_seal_hash);
        let mut overlay = StateOverlay::new();
        let mut gas_used: u64 = 0;
        let base_gas = 21000u64;
        gas_used = gas_used.saturating_add(base_gas);

        if gas_used > gas_limit {
            return (ExecutionReceipt::failed(position, from_root, from_root, gas_limit, 1), overlay);
        }

        let storage_key = {
            let mut h = Hasher::new();
            h.update(&position.hash());
            h.update(tx_data);
            let mut key = [0u8; 32];
            key.copy_from_slice(&h.finalize().as_bytes()[..32]);
            key
        };
        overlay.put(storage_key, tx_data.to_vec());

        let new_root = Self::compute_root_with_overlay(state, execution_version, epoch_seal_hash, &overlay);
        let write_set = overlay.to_writeset();

        (ExecutionReceipt::success(position, from_root, new_root, gas_used, write_set), overlay)
    }

    /// Instance method for backward compatibility.
    pub fn execute(
        &self,
        position: ChainPosition,
        tx_data: &[u8],
        gas_limit: u64,
    ) -> (ExecutionReceipt, StateOverlay) {
        Self::execute_transition(
            &self.state, self.execution_version, self.epoch_seal_hash,
            position, tx_data, gas_limit,
        )
    }

    pub fn compute_root(
        state: &DeterministicMap<[u8; 32], Vec<u8>>,
        execution_version: u64,
        epoch_seal_hash: Option<[u8; 32]>,
    ) -> [u8; 32] {
        let mut enc = CanonicalEncoder::new(SchemaVersion::V4);
        let _ = enc.write_u64(execution_version);
        if let Some(seal_hash) = epoch_seal_hash {
            let _ = enc.write_bytes(&seal_hash);
        }
        for (key, value) in state.iter() {
            let _ = enc.write_bytes(key);
            let _ = enc.write_bytes(value);
        }
        enc.finish()
    }

    pub fn compute_root_with_overlay(
        state: &DeterministicMap<[u8; 32], Vec<u8>>,
        execution_version: u64,
        epoch_seal_hash: Option<[u8; 32]>,
        overlay: &StateOverlay,
    ) -> [u8; 32] {
        let mut enc = CanonicalEncoder::new(SchemaVersion::V4);
        let _ = enc.write_u64(execution_version);
        if let Some(seal_hash) = epoch_seal_hash {
            let _ = enc.write_bytes(&seal_hash);
        }
        let mut all_keys: Vec<[u8; 32]> = state.iter().map(|(k, _)| *k).collect();
        for (op, _) in &overlay.delta {
            match op {
                StateOperation::Put { key, .. } | StateOperation::Delete { key } => {
                    if !all_keys.contains(key) { all_keys.push(*key); }
                }
            }
        }
        all_keys.sort();
        for key in &all_keys {
            let value = Self::resolve_value(state, &overlay.delta, key);
            if !value.is_empty() {
                let _ = enc.write_bytes(key);
                let _ = enc.write_bytes(&value);
            }
        }
        enc.finish()
    }

    fn resolve_value(
        state: &DeterministicMap<[u8; 32], Vec<u8>>,
        delta: &[(StateOperation, bool)],
        key: &[u8; 32],
    ) -> Vec<u8> {
        for (op, _) in delta.iter().rev() {
            match op {
                StateOperation::Put { key: k, value } if k == key => return value.clone(),
                StateOperation::Delete { key: k } if k == key => return Vec::new(),
                _ => {}
            }
        }
        state.get(key).cloned().unwrap_or_default()
    }

    pub fn apply_overlay(&mut self, overlay: StateOverlay, new_root: [u8; 32]) {
        overlay.commit_to(&mut self.state);
        self.current_root = new_root;
    }

    pub fn live_root(&self) -> [u8; 32] { self.current_root }
}
