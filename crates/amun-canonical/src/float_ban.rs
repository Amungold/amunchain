pub struct FloatBan;

impl FloatBan {
    pub fn verify_no_floats(code: &str) -> Result<(), Vec<&'static str>> {
        let mut violations = Vec::new();
        for keyword in &["f32", "f64", "float", "double"] {
            if code.contains(keyword) {
                violations.push(*keyword);
            }
        }
        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    pub fn verify_type<T: 'static>() -> Result<(), &'static str> {
        let type_name = core::any::type_name::<T>();
        if type_name.contains("f32") || type_name.contains("f64") {
            Err("floating point type forbidden in consensus path")
        } else {
            Ok(())
        }
    }
}
