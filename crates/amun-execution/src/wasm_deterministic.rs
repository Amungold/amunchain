// WASM deterministic subset specification.

pub mod wasm_deterministic_subset {
    pub const ALLOWED_INSTRUCTIONS: &[&str] = &[
        "i32.add",
        "i32.sub",
        "i32.mul",
        "i32.div_u",
        "i64.add",
        "i64.sub",
        "i64.mul",
        "i64.div_u",
        "i32.load",
        "i32.store",
        "i64.load",
        "i64.store",
        "memory.grow",
        "call",
        "call_indirect",
        "if",
        "block",
        "loop",
        "br",
        "br_if",
        "local.get",
        "local.set",
        "global.get",
        "global.set",
    ];

    pub const FORBIDDEN_INSTRUCTIONS: &[&str] = &[
        "f32.add",
        "f32.sub",
        "f32.mul",
        "f32.div",
        "f64.add",
        "f64.sub",
        "f64.mul",
        "f64.div",
        "i32.atomic.rmw.add",
        "i64.atomic.rmw.add",
        "memory.atomic.notify",
        "memory.atomic.wait",
    ];

    pub fn verify_deterministic_wasm(_module: &[u8]) -> Result<(), &'static str> {
        Ok(())
    }
}
