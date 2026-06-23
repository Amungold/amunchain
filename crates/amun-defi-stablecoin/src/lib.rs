use amun_resource_core::{RegistryError, ResourceId, ResourceRegistry};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub struct StablecoinPosition {
    pub position_id: ResourceId,
    pub owner: [u8; 32],
    pub collateral: u64,
    pub minted: u64,
    pub active: bool,
}

pub struct StablecoinEngine {
    pub positions: BTreeMap<[u8; 32], StablecoinPosition>,
    pub total_supply: u64,
}

impl Default for StablecoinEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl StablecoinEngine {
    pub fn new() -> Self {
        Self {
            positions: BTreeMap::new(),
            total_supply: 0,
        }
    }

    pub fn mint(
        &mut self,
        _registry: &mut ResourceRegistry,
        owner: [u8; 32],
        collateral_amount: u64,
        mint_amount: u64,
    ) -> Result<ResourceId, RegistryError> {
        if mint_amount > collateral_amount * 2 / 3 {
            return Err(RegistryError::NotActive(ResourceId([0u8; 32])));
        }
        let position_id = ResourceId(generate_id(&owner, collateral_amount));
        let position = StablecoinPosition {
            position_id,
            owner,
            collateral: collateral_amount,
            minted: mint_amount,
            active: true,
        };
        self.total_supply += mint_amount;
        self.positions.insert(position_id.0, position);
        Ok(position_id)
    }

    pub fn burn(
        &mut self,
        position_id: &ResourceId,
        burn_amount: u64,
    ) -> Result<u64, &'static str> {
        if let Some(position) = self.positions.get_mut(&position_id.0) {
            if !position.active {
                return Err("Position not active");
            }
            if burn_amount > position.minted {
                return Err("Insufficient minted amount");
            }
            position.minted -= burn_amount;
            self.total_supply -= burn_amount;
            if position.minted == 0 {
                position.active = false;
            }
            Ok(burn_amount)
        } else {
            Err("Position not found")
        }
    }

    pub fn compute_stablecoin_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_STABLECOIN_V1");
        hasher.update(self.total_supply.to_le_bytes());
        for (id, position) in &self.positions {
            hasher.update(id);
            hasher.update(position.collateral.to_le_bytes());
            hasher.update(position.minted.to_le_bytes());
            hasher.update([position.active as u8]);
        }
        hasher.finalize().into()
    }
}

fn generate_id(owner: &[u8; 32], collateral: u64) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(owner);
    hasher.update(collateral.to_le_bytes());
    hasher.finalize().into()
}
