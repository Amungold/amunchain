use crate::economic_error::EconomicError;
use crate::economic_snapshot::EconomicSnapshot;
use crate::Hash32;

const DOMAIN_ECON_LEAF: &[u8] = b"AMUN_ECON_LEAF_V1";
const DOMAIN_ECON_NODE: &[u8] = b"AMUN_ECON_NODE_V1";

fn leaf_hash(leaf_index: u16, value: u64) -> Hash32 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN_ECON_LEAF);
    hasher.update(&leaf_index.to_be_bytes());
    hasher.update(&value.to_be_bytes());
    *hasher.finalize().as_bytes()
}

fn parent_hash(left: &Hash32, right: &Hash32) -> Hash32 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN_ECON_NODE);
    hasher.update(left);
    hasher.update(right);
    *hasher.finalize().as_bytes()
}

fn build_merkle_root(mut hashes: Vec<Hash32>) -> Hash32 {
    if hashes.is_empty() {
        return [0u8; 32];
    }
    while hashes.len() > 1 {
        if !hashes.len().is_multiple_of(2) {
            hashes.push(*hashes.last().unwrap());
        }
        let mut next = Vec::with_capacity(hashes.len() / 2);
        for chunk in hashes.chunks(2) {
            next.push(parent_hash(&chunk[0], &chunk[1]));
        }
        hashes = next;
    }
    hashes[0]
}

pub struct EconomicTree;

impl EconomicTree {
    pub fn root(snapshot: &EconomicSnapshot) -> Result<Hash32, EconomicError> {
        let computed_circulating = snapshot
            .total_supply
            .saturating_sub(snapshot.burned_supply)
            .saturating_sub(snapshot.staked_supply)
            .saturating_sub(snapshot.treasury_balance);

        if computed_circulating != snapshot.circulating_supply {
            return Err(EconomicError::InvalidCirculatingSupply {
                computed: computed_circulating,
                stored: snapshot.circulating_supply,
            });
        }

        let leaves = vec![
            leaf_hash(0, snapshot.total_supply),
            leaf_hash(1, snapshot.treasury_balance),
            leaf_hash(2, snapshot.validator_reward_pool),
            leaf_hash(3, snapshot.ecosystem_pool),
            leaf_hash(4, snapshot.burned_supply),
            leaf_hash(5, snapshot.issued_supply),
            leaf_hash(6, snapshot.staked_supply),
            leaf_hash(7, snapshot.circulating_supply),
        ];

        Ok(build_merkle_root(leaves))
    }
}
