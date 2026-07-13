use crate::transcript::EntropyTranscript;
use blake3::Hasher;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntropyRole {
    Consensus,
    Simulation,
    Election,
    Scheduler,
    Replay,
}

pub struct DeterministicEntropy {
    transcript: EntropyTranscript,
    role: EntropyRole,
}

impl DeterministicEntropy {
    pub fn new(seed: [u8; 32]) -> Self {
        Self {
            transcript: EntropyTranscript::new(seed),
            role: EntropyRole::Consensus,
        }
    }

    pub fn with_role(seed: [u8; 32], role: EntropyRole) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_ENTROPY_ROLE_V1");
        hasher.update(&seed);
        hasher.update(&(role as u32).to_le_bytes());
        let mut role_seed = [0u8; 32];
        role_seed.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
        Self {
            transcript: EntropyTranscript::new(role_seed),
            role,
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        let bytes = self.next_bytes::<8>();
        u64::from_le_bytes(bytes)
    }

    pub fn next_bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_ENTROPY_V1");
        hasher.update(&(self.role as u32).to_le_bytes());
        hasher.update(&self.transcript.seed());
        hasher.update(&self.transcript.counter().to_le_bytes());
        self.transcript.increment();

        let hash = hasher.finalize();
        let mut out = [0u8; N];
        let hash_bytes = hash.as_bytes();
        for i in 0..N {
            out[i] = hash_bytes[i % 32];
        }

        let mut record_val = [0u8; 32];
        let copy_len = 32.min(N);
        record_val[..copy_len].copy_from_slice(&out[..copy_len]);
        self.transcript.record(record_val);

        out
    }

    pub fn gen_range(&mut self, min: u64, max: u64) -> u64 {
        if min >= max {
            return min;
        }
        let range = max - min + 1;
        min + (self.next_u64() % range)
    }

    pub fn fork(&self, branch: &[u8]) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(&self.transcript.seed());
        hasher.update(&self.transcript.counter().to_le_bytes());
        hasher.update(branch);
        let mut new_seed = [0u8; 32];
        new_seed.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
        Self {
            transcript: EntropyTranscript::new(new_seed),
            role: self.role,
        }
    }

    pub fn into_transcript(self) -> EntropyTranscript {
        self.transcript
    }
}
