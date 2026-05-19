pub struct HostBoundary;

impl HostBoundary {
    pub const MAX_REQUEST_SIZE: usize = 1_048_576;
    pub const MAX_BATCH_SIZE: usize = 64;

    pub fn validate_ingress(payload: &[u8], max_size: usize) -> Result<&[u8], &'static str> {
        if payload.len() > max_size {
            return Err("payload exceeds maximum size");
        }
        if payload.is_empty() {
            return Err("empty payload rejected");
        }
        Ok(payload)
    }

    pub fn canonical_hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_HOST_BOUNDARY_V4");
        hasher.update(data);
        let h = hasher.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&h.as_bytes()[..32]);
        out
    }
}
