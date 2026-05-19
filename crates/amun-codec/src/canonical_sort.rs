// Canonical sorting using byte-ordered comparison.

use crate::encode::CanonicalEncode;
use crate::writer::{BufferWriter, CanonicalWriter, WriteResult};

pub trait CanonicalSortKey {
    fn write_sort_key(&self, writer: &mut impl CanonicalWriter) -> WriteResult;
}

impl<T: CanonicalEncode> CanonicalSortKey for T {
    fn write_sort_key(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        self.encode_to_writer(writer)
    }
}

pub fn compare_by_canonical_bytes<T: CanonicalEncode>(a: &T, b: &T) -> core::cmp::Ordering {
    let mut buf_a = [0u8; 128];
    let mut buf_b = [0u8; 128];

    let len_a = {
        let mut writer = BufferWriter::new(&mut buf_a);
        a.encode_to_writer(&mut writer).ok();
        writer.position()
    };

    let len_b = {
        let mut writer = BufferWriter::new(&mut buf_b);
        b.encode_to_writer(&mut writer).ok();
        writer.position()
    };

    let min_len = len_a.min(len_b);
    for i in 0..min_len {
        match buf_a[i].cmp(&buf_b[i]) {
            core::cmp::Ordering::Equal => continue,
            non_eq => return non_eq,
        }
    }

    len_a.cmp(&len_b)
}

// Manual insertion sort for no_std compatibility.
pub fn canonical_sort<T: CanonicalEncode>(items: &mut [T]) {
    let len = items.len();
    for i in 1..len {
        let mut j = i;
        while j > 0
            && compare_by_canonical_bytes(&items[j - 1], &items[j]) == core::cmp::Ordering::Greater
        {
            items.swap(j - 1, j);
            j -= 1;
        }
    }
}
