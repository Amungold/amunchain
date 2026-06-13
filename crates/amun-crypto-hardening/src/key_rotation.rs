use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyRotationCertificate {
    pub sequence: u64,
    pub old_public_key: [u8; 32],
    pub new_public_key: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub rotation_signature: [u8; 64],
    pub issued_at: u64,
}

impl KeyRotationCertificate {
    pub fn new(
        sequence: u64,
        old_public_key: [u8; 32],
        new_public_key: [u8; 32],
        rotation_signature: [u8; 64],
        issued_at: u64,
    ) -> Self {
        Self {
            sequence,
            old_public_key,
            new_public_key,
            rotation_signature,
            issued_at,
        }
    }

    pub fn verify(&self) -> bool {
        let mut message = Vec::new();
        message.extend_from_slice(&self.sequence.to_le_bytes());
        message.extend_from_slice(&self.old_public_key);
        message.extend_from_slice(&self.new_public_key);

        use crate::production_keys::ValidatorPublicKey;
        let old_key = ValidatorPublicKey::from_bytes(self.old_public_key);
        old_key.verify(&message, &self.rotation_signature)
    }

    pub fn rotation_id(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_KEY_ROTATION_V1");
        hasher.update(&self.sequence.to_le_bytes());
        hasher.update(&self.old_public_key);
        hasher.update(&self.new_public_key);
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyRotationChain {
    pub rotations: Vec<KeyRotationCertificate>,
    pub genesis_key: [u8; 32],
}

impl KeyRotationChain {
    pub fn new(genesis_key: [u8; 32]) -> Self {
        Self {
            rotations: Vec::new(),
            genesis_key,
        }
    }

    pub fn add_rotation(&mut self, cert: KeyRotationCertificate) -> Result<(), String> {
        if !cert.verify() {
            return Err("Rotation certificate verification failed".into());
        }
        let expected_seq = self.rotations.len() as u64 + 1;
        if cert.sequence != expected_seq {
            return Err(format!(
                "Expected sequence {}, got {}",
                expected_seq, cert.sequence
            ));
        }
        let current_key = self.current_key();
        if cert.old_public_key != current_key {
            return Err("Old public key does not match current active key".into());
        }
        self.rotations.push(cert);
        Ok(())
    }

    pub fn current_key(&self) -> [u8; 32] {
        self.rotations
            .last()
            .map(|r| r.new_public_key)
            .unwrap_or(self.genesis_key)
    }

    pub fn rotation_count(&self) -> usize {
        self.rotations.len()
    }

    pub fn verify_chain(&self) -> bool {
        let mut expected_key = self.genesis_key;
        for cert in &self.rotations {
            if cert.old_public_key != expected_key {
                return false;
            }
            if !cert.verify() {
                return false;
            }
            expected_key = cert.new_public_key;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::production_keys::ValidatorKeypair;

    #[test]
    fn n58_single_key_rotation() {
        let old_kp = ValidatorKeypair::generate();
        let new_kp = ValidatorKeypair::generate();

        let mut message = Vec::new();
        message.extend_from_slice(&1u64.to_le_bytes());
        message.extend_from_slice(&old_kp.public_key);
        message.extend_from_slice(&new_kp.public_key);
        let sig = old_kp.sign(&message);

        let cert = KeyRotationCertificate::new(1, old_kp.public_key, new_kp.public_key, sig, 1000);
        assert!(cert.verify());
    }

    #[test]
    fn n58_rotation_chain() {
        let genesis_kp = ValidatorKeypair::generate();
        let mut chain = KeyRotationChain::new(genesis_kp.public_key);

        let mut current_kp = genesis_kp;
        for i in 1..=3 {
            let new_kp = ValidatorKeypair::generate();
            let mut message = Vec::new();
            #[allow(clippy::unnecessary_cast)]
            message.extend_from_slice(&(i as u64).to_le_bytes());
            message.extend_from_slice(&current_kp.public_key);
            message.extend_from_slice(&new_kp.public_key);
            let sig = current_kp.sign(&message);

            let cert = KeyRotationCertificate::new(
                i,
                current_kp.public_key,
                new_kp.public_key,
                sig,
                1000 + i,
            );
            chain.add_rotation(cert).unwrap();
            current_kp = new_kp;
        }

        assert_eq!(chain.rotation_count(), 3);
        assert!(chain.verify_chain());
    }

    #[test]
    fn n58_reject_wrong_sequence() {
        let kp1 = ValidatorKeypair::generate();
        let kp2 = ValidatorKeypair::generate();
        let mut chain = KeyRotationChain::new(kp1.public_key);

        let mut message = Vec::new();
        message.extend_from_slice(&5u64.to_le_bytes());
        message.extend_from_slice(&kp1.public_key);
        message.extend_from_slice(&kp2.public_key);
        let sig = kp1.sign(&message);

        let cert = KeyRotationCertificate::new(5, kp1.public_key, kp2.public_key, sig, 1000);
        assert!(chain.add_rotation(cert).is_err());
    }
}
