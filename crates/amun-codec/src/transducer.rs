// Streaming canonical transducer for lexicographic comparison.
// Compares values byte-by-byte without materializing full encodings.

pub trait CanonicalCursor {
    fn next_byte(&mut self) -> Option<u8>;
}

pub struct CanonicalTransducer;

impl CanonicalTransducer {
    pub fn compare<A: CanonicalCursor, B: CanonicalCursor>(
        cursor_a: &mut A,
        cursor_b: &mut B,
    ) -> core::cmp::Ordering {
        loop {
            match (cursor_a.next_byte(), cursor_b.next_byte()) {
                (Some(a), Some(b)) => match a.cmp(&b) {
                    core::cmp::Ordering::Equal => continue,
                    non_eq => return non_eq,
                },
                (Some(_), None) => return core::cmp::Ordering::Greater,
                (None, Some(_)) => return core::cmp::Ordering::Less,
                (None, None) => return core::cmp::Ordering::Equal,
            }
        }
    }
}

pub struct U64Cursor {
    value: u64,
    position: u8,
}

impl U64Cursor {
    pub fn new(value: u64) -> Self {
        Self { value, position: 0 }
    }
}

impl CanonicalCursor for U64Cursor {
    fn next_byte(&mut self) -> Option<u8> {
        if self.position >= 8 {
            return None;
        }
        let byte = (self.value >> (self.position * 8)) as u8;
        self.position += 1;
        Some(byte)
    }
}

pub struct SliceCursor<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> SliceCursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }
}

impl<'a> CanonicalCursor for SliceCursor<'a> {
    fn next_byte(&mut self) -> Option<u8> {
        if self.position >= self.data.len() {
            return None;
        }
        let byte = self.data[self.position];
        self.position += 1;
        Some(byte)
    }
}
