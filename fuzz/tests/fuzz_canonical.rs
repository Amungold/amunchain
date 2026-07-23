use amun_canonical_codec::{CanonicalReader, CanonicalWriter};

#[test]
fn fuzz_roundtrip_random_sizes() {
    for size in [0, 1, 16, 255, 1024, 65536] {
        let data = vec![0xAAu8; size];
        let mut w = CanonicalWriter::new();
        w.write_bytes(&data);
        let bytes = w.into_bytes();
        let mut r = CanonicalReader::new(&bytes);
        assert!(r.read_bytes().is_some());
    }
}

#[test]
fn fuzz_empty_input() {
    let mut r = CanonicalReader::new(&[]);
    assert!(r.read_u32().is_none());
    assert!(r.read_bool().is_none());
}

#[test]
fn fuzz_truncated_length() {
    let mut w = CanonicalWriter::new();
    w.write_u32(100);
    let bytes = w.into_bytes();
    let mut r = CanonicalReader::new(&bytes);
    let _ = r.read_bytes();
}

#[test]
fn fuzz_multiple_reads() {
    let mut w = CanonicalWriter::new();
    w.write_u32(42);
    w.write_u64(12345);
    w.write_bool(true);
    let bytes = w.into_bytes();
    let mut r = CanonicalReader::new(&bytes);
    assert_eq!(r.read_u32(), Some(42));
    assert_eq!(r.read_u64(), Some(12345));
    assert_eq!(r.read_bool(), Some(true));
}
