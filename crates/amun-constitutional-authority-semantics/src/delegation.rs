use crate::capability::CapabilityWitness;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DelegationChain {
    pub witnesses: Vec<CapabilityWitness>,
}

impl DelegationChain {
    pub fn new(root: CapabilityWitness) -> Self {
        Self {
            witnesses: vec![root],
        }
    }

    pub fn append(&mut self, witness: CapabilityWitness) -> Result<(), String> {
        let prev = self.witnesses.last().ok_or("Empty chain")?;
        if witness.signature.verifying_key_hex != prev.artifact.subject_verifying_key_hex {
            return Err("Delegation must be signed by previous subject".into());
        }
        if witness.artifact.epoch_start < prev.artifact.epoch_start
            || witness.artifact.epoch_end > prev.artifact.epoch_end
        {
            return Err("Epoch exceeds parent".into());
        }
        self.witnesses.push(witness);
        Ok(())
    }

    pub fn verify(&self) -> Result<(), String> {
        for w in &self.witnesses {
            w.verify()?;
        }
        for i in 1..self.witnesses.len() {
            let prev = &self.witnesses[i - 1];
            let curr = &self.witnesses[i];
            if curr.signature.verifying_key_hex != prev.artifact.subject_verifying_key_hex {
                return Err("Broken signature chain".into());
            }
            if curr.artifact.epoch_start < prev.artifact.epoch_start
                || curr.artifact.epoch_end > prev.artifact.epoch_end
            {
                return Err("Epoch containment violated".into());
            }
        }
        Ok(())
    }
}

pub type DelegationProof = DelegationChain;
