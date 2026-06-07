#![no_main]
use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

fuzz_target!(|data: &[u8]| {
    let dir = tempdir().unwrap();
    let path = dir.path().join("wal");
    std::fs::write(&path, data).ok();
    // Attempt roundtrip — we don't care about success, only about panics
    let _ = std::fs::read_to_string(&path);
});
