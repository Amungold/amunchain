use amun_resource_core::{
    ResourceId, ResourceRegistry, ResourceState, RegistryError,
};
use amun_nft_core::{NftEvent, NftEvidence};

/// Constitutional Evidence Kernel (CEK) for NFT operations.
pub struct NftEvidenceKernel;

/// Context for mint verification to reduce argument count.
pub struct MintVerificationContext<'a> {
    pub registry: &'a ResourceRegistry,
    pub collection_id: &'a ResourceId,
    pub token_id: &'a ResourceId,
    pub owner: &'a [u8; 32],
    pub metadata_hash: &'a [u8; 32],
    pub actual_metadata_hash: &'a [u8; 32],
    pub timestamp: u64,
    pub last_event_time: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CekError {
    Law1InvalidOwnership,
    Law2DuplicateToken,
    Law3InvalidMetadataHash,
    Law4ReplayDetected,
    Law5EvidenceGenerationFailed,
    Registry(RegistryError),
}

impl NftEvidenceKernel {
    /// Law 1: Valid Ownership — only the owner can transfer or burn
    pub fn verify_ownership(
        registry: &ResourceRegistry,
        token_id: &ResourceId,
        claimed_owner: &[u8; 32],
    ) -> Result<(), CekError> {
        let token = registry.get(token_id)
            .ok_or(CekError::Registry(RegistryError::NotFound(*token_id)))?;
        if token.owner != *claimed_owner {
            return Err(CekError::Law1InvalidOwnership);
        }
        Ok(())
    }

    /// Law 2: Non-Duplicate Token — token ID must not already exist (for mint)
    pub fn verify_non_duplicate(
        registry: &ResourceRegistry,
        token_id: &ResourceId,
    ) -> Result<(), CekError> {
        if registry.contains(token_id) {
            return Err(CekError::Law2DuplicateToken);
        }
        Ok(())
    }

    /// Law 3: Valid Metadata Hash — for mint operations
    pub fn verify_metadata_hash(
        claimed_hash: &[u8; 32],
        actual_hash: &[u8; 32],
    ) -> Result<(), CekError> {
        if claimed_hash != actual_hash {
            return Err(CekError::Law3InvalidMetadataHash);
        }
        Ok(())
    }

    /// Law 4: Replay Protection — check nonce or timestamp ordering
    pub fn verify_replay_protection(
        last_event_time: u64,
        current_time: u64,
    ) -> Result<(), CekError> {
        if current_time <= last_event_time {
            return Err(CekError::Law4ReplayDetected);
        }
        Ok(())
    }

    /// Law 5: Evidence Generation — produce evidence record
    pub fn generate_evidence(
        event: NftEvent,
        timestamp: u64,
        block_height: u64,
    ) -> Result<NftEvidence, CekError> {
        Ok(NftEvidence::new(event, timestamp, block_height))
    }

    /// Full CEK check before mint
    pub fn verify_mint(ctx: MintVerificationContext) -> Result<(), CekError> {
        // Law 2: token must not exist
        Self::verify_non_duplicate(ctx.registry, ctx.token_id)?;

        // Law 1: collection must exist and be active
        let collection = ctx.registry.get(ctx.collection_id)
            .ok_or(CekError::Registry(RegistryError::NotFound(*ctx.collection_id)))?;
        if !matches!(collection.state, ResourceState::Active) {
            return Err(CekError::Law1InvalidOwnership);
        }

        // Law 3: metadata hash must match
        Self::verify_metadata_hash(ctx.metadata_hash, ctx.actual_metadata_hash)?;

        // Law 4: replay protection
        Self::verify_replay_protection(ctx.last_event_time, ctx.timestamp)?;

        Ok(())
    }

    /// Full CEK check before transfer
    pub fn verify_transfer(
        registry: &ResourceRegistry,
        token_id: &ResourceId,
        from: &[u8; 32],
        timestamp: u64,
        last_event_time: u64,
    ) -> Result<(), CekError> {
        // Law 1: only owner can transfer
        Self::verify_ownership(registry, token_id, from)?;

        // Law 4: replay protection
        Self::verify_replay_protection(last_event_time, timestamp)?;

        Ok(())
    }

    /// Full CEK check before burn
    pub fn verify_burn(
        registry: &ResourceRegistry,
        token_id: &ResourceId,
        owner: &[u8; 32],
        timestamp: u64,
        last_event_time: u64,
    ) -> Result<(), CekError> {
        // Law 1: only owner can burn
        Self::verify_ownership(registry, token_id, owner)?;

        // Law 4: replay protection
        Self::verify_replay_protection(last_event_time, timestamp)?;

        Ok(())
    }
}

impl From<RegistryError> for CekError {
    fn from(err: RegistryError) -> Self {
        CekError::Registry(err)
    }
}

/// Accumulates NFT evidence hashes for inclusion in BlockEvidenceRoot.
pub fn accumulate_nft_evidence_root(evidences: &[NftEvidence]) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    for ev in evidences {
        hasher.update(ev.evidence_hash);
    }
    hasher.finalize().into()
}
