/// Marker trait for types with a stable, deterministic memory layout.
///
/// # Safety
///
/// Implementors must guarantee:
/// 1. No internal pointers or references.
/// 2. size_of and align_of are identical on all supported targets.
/// 3. The type is Copy or can be safely memcpy'd.
/// 4. Bit representation is identical on x86_64, aarch64, and wasm32.
pub unsafe trait StableLayout: Sized + Copy {}

unsafe impl StableLayout for u8 {}
unsafe impl StableLayout for u16 {}
unsafe impl StableLayout for u32 {}
unsafe impl StableLayout for u64 {}
unsafe impl StableLayout for u128 {}
unsafe impl StableLayout for i8 {}
unsafe impl StableLayout for i16 {}
unsafe impl StableLayout for i32 {}
unsafe impl StableLayout for i64 {}
unsafe impl StableLayout for [u8; 32] {}
unsafe impl StableLayout for [u8; 48] {}
unsafe impl StableLayout for [u8; 96] {}

pub const fn assert_stable_layout<T: StableLayout>() {}
