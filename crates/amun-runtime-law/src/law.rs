pub struct RuntimeLaw;

impl RuntimeLaw {
    pub const FORBIDDEN_API: &[&str] = &[
        "std::time::SystemTime",
        "std::time::Instant",
        "std::thread::sleep",
        "f32",
        "f64",
        "rand::random",
    ];

    pub fn verify_code(code: &str) -> Result<(), Vec<&'static str>> {
        let mut violations = Vec::new();
        for api in Self::FORBIDDEN_API {
            if code.contains(api) {
                violations.push(*api);
            }
        }
        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }
}
