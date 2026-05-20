use heapless::Vec;

pub struct CanonicalSorter;

impl CanonicalSorter {
    pub fn sort_bytes(pairs: &mut Vec<(&[u8], &[u8]), 256>) {
        pairs.sort_unstable_by(|a, b| a.0.cmp(b.0));
    }

    pub fn sort_validators(validators: &mut Vec<([u8; 32], u64), 128>) {
        validators.sort_unstable_by_key(|a| a.0);
    }
}
