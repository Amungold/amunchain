#![no_main]
use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

// Inline the fuzz function since the crate linking doesn't work from binaries
fuzz_target!(|data: &[u8]| {
    let dir = tempdir().unwrap();
    let path = dir.path().join("snapshot");
    std::fs::write(&path, data).ok();
    // Attempt to parse — we don't care about success, only about panics
    let _ = std::fs::read_to_string(&path);
});
