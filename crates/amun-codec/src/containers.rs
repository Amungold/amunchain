// Canonical container encoding: sequences, sets, and maps.

use crate::encode::CanonicalEncode;
use crate::writer::{CanonicalWriter, WriteResult};
use amun_failure::{module_ids, operation_ids, ConstitutionalFault, FailureContext};
use heapless::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DuplicatePolicy {
    FirstWins = 0,
    LastWins = 1,
    Reject = 2,
}

pub const CONSTITUTIONAL_DUPLICATE_POLICY: DuplicatePolicy = DuplicatePolicy::Reject;

// Manual sort for heapless Vec since heapless 0.8 does not provide sort().
fn sort_heapless_vec<T: Ord, const N: usize>(vec: &mut Vec<T, N>) {
    // Simple insertion sort — stable, deterministic, no alloc.
    let len = vec.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && vec[j - 1] > vec[j] {
            vec.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn sort_heapless_vec_by<T, F, const N: usize>(vec: &mut Vec<T, N>, mut compare: F)
where
    F: FnMut(&T, &T) -> core::cmp::Ordering,
{
    let len = vec.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && compare(&vec[j - 1], &vec[j]) == core::cmp::Ordering::Greater {
            vec.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn encode_sequence<T: CanonicalEncode>(
    items: &[T],
    writer: &mut impl CanonicalWriter,
) -> WriteResult {
    let len = items.len();
    if len > u32::MAX as usize {
        return Err(FailureContext::new(
            ConstitutionalFault::CapacityExceeded,
            module_ids::AMUN_CODEC,
            operation_ids::ENCODE_SEQUENCE,
        ));
    }
    (len as u32).encode_to_writer(writer)?;
    for item in items {
        item.encode_to_writer(writer)?;
    }
    Ok(())
}

pub fn encode_set<T: CanonicalEncode + Ord>(
    items: &[T],
    writer: &mut impl CanonicalWriter,
) -> WriteResult {
    let mut sorted: Vec<&T, 256> = Vec::new();
    for item in items {
        sorted.push(item).map_err(|_| {
            FailureContext::new(
                ConstitutionalFault::CapacityExceeded,
                module_ids::AMUN_CODEC,
                operation_ids::ENCODE_SET,
            )
        })?;
    }
    sort_heapless_vec(&mut sorted);

    let len = sorted.len();
    if len > u32::MAX as usize {
        return Err(FailureContext::new(
            ConstitutionalFault::CapacityExceeded,
            module_ids::AMUN_CODEC,
            operation_ids::ENCODE_SET,
        ));
    }
    (len as u32).encode_to_writer(writer)?;
    for item in sorted {
        item.encode_to_writer(writer)?;
    }
    Ok(())
}

pub fn encode_map<K: CanonicalEncode + Ord, V: CanonicalEncode>(
    entries: &[(K, V)],
    writer: &mut impl CanonicalWriter,
) -> WriteResult {
    let mut sorted: Vec<&(K, V), 256> = Vec::new();
    for entry in entries {
        sorted.push(entry).map_err(|_| {
            FailureContext::new(
                ConstitutionalFault::CapacityExceeded,
                module_ids::AMUN_CODEC,
                operation_ids::ENCODE_MAP,
            )
        })?;
    }
    sort_heapless_vec_by(&mut sorted, |a, b| a.0.cmp(&b.0));

    let len = sorted.len();
    if len > u32::MAX as usize {
        return Err(FailureContext::new(
            ConstitutionalFault::CapacityExceeded,
            module_ids::AMUN_CODEC,
            operation_ids::ENCODE_MAP,
        ));
    }
    (len as u32).encode_to_writer(writer)?;
    for (k, v) in sorted {
        k.encode_to_writer(writer)?;
        v.encode_to_writer(writer)?;
    }
    Ok(())
}
