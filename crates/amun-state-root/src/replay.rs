use amun_kernel::canonical::{CanonicalEncode, encode_hash_slice};

#[derive(Clone, PartialEq)]
pub struct ReplayEquivalenceProof {
    pub live_root:     [u8; 32],
    pub replayed_root: [u8; 32],
    pub identical:     bool,
}

impl CanonicalEncode for ReplayEquivalenceProof {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.live_root);
        out.extend_from_slice(&self.replayed_root);
        (self.identical as u8).encode_canonical(out);
    }
}

#[derive(Clone)]
pub struct ReplayTranscript {
    pub tx_hashes:           Vec<[u8; 32]>,
    pub pre_state_roots:     Vec<[u8; 32]>,
    pub post_state_roots:    Vec<[u8; 32]>,
    pub receipts:            Vec<[u8; 32]>,
    pub emitted_events:      Vec<[u8; 32]>,
    pub scheduler_trace:     Vec<[u8; 32]>,
    pub consensus_trace:     Vec<[u8; 32]>,
}

impl CanonicalEncode for ReplayTranscript {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        encode_hash_slice(&self.tx_hashes, out);
        encode_hash_slice(&self.pre_state_roots, out);
        encode_hash_slice(&self.post_state_roots, out);
        encode_hash_slice(&self.receipts, out);
        encode_hash_slice(&self.emitted_events, out);
        encode_hash_slice(&self.scheduler_trace, out);
        encode_hash_slice(&self.consensus_trace, out);
    }
}

#[derive(Clone)]
pub struct ReplayCertificate {
    pub transcript:   ReplayTranscript,
    pub proof:        ReplayEquivalenceProof,
}

impl CanonicalEncode for ReplayCertificate {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        self.transcript.encode_canonical(out);
        self.proof.encode_canonical(out);
    }
}
