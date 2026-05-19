use amun_crypto::Ed25519Signer;

pub struct SignedSnapshot {
    pub epoch: u64,
    pub state_root: [u8; 32],
    pub signatures: Vec<([u8; 32], [u8; 64])>,
    pub required_signers: u8,
}

impl SignedSnapshot {
    pub fn new(epoch: u64, root: [u8; 32], required_signers: u8) -> Self {
        Self {
            epoch,
            state_root: root,
            signatures: Vec::new(),
            required_signers,
        }
    }

    pub fn add_signature(&mut self, signer: &Ed25519Signer, chain_id: u64) -> Result<(), &'static str> {
        let pk = signer.public_bytes();
        
        if self.signatures.iter().any(|(existing_pk, _)| *existing_pk == pk) {
            return Err("duplicate signer");
        }
        
        let sig = signer.sign(&self.state_root, b"AMUN_SNAPSHOT_V4", chain_id)
            .map_err(|_| "signing failed")?;
        self.signatures.push((pk, sig));
        Ok(())
    }

    pub fn verify(&self, chain_id: u64) -> bool {
        let valid_count = self.signatures.iter()
            .filter(|(pk, sig)| {
                Ed25519Signer::verify(pk, &self.state_root, sig, b"AMUN_SNAPSHOT_V4", chain_id).is_ok()
            })
            .count();
        valid_count >= self.required_signers as usize
    }
}
