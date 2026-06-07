#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    amun_fuzz::resource_fuzz::fuzz_resource_registry(data);
});
