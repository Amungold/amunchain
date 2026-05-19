#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleSeverity {
    Fatal,
    Warning,
}

pub struct ConstitutionalRule {
    pub id: &'static str,
    pub severity: RuleSeverity,
    pub check: fn(&str, &str) -> Vec<Violation>,
}

#[derive(Debug)]
pub struct Violation {
    pub file: String,
    pub line: usize,
    pub message: String,
}

fn no_unsafe_outside_boundary(content: &str, path: &str) -> Vec<Violation> {
    let mut violations = Vec::new();
    if path.contains("amun-unsafe") {
        return violations;
    }
    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("///") || trimmed.starts_with("*") {
            continue;
        }
        if trimmed.contains("unsafe ")
            || trimmed.contains("unsafe{")
            || trimmed.contains("unsafe {")
        {
            violations.push(Violation {
                file: path.to_string(),
                line: i + 1,
                message: format!("unsafe outside amun-unsafe: {}", trimmed),
            });
        }
    }
    violations
}

fn no_floats_in_consensus(content: &str, path: &str) -> Vec<Violation> {
    let mut violations = Vec::new();
    if path.contains("/tools/") || path.contains("/tests/") {
        return violations;
    }
    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("///") {
            continue;
        }
        if trimmed.contains(": f32") || trimmed.contains(": f64") {
            violations.push(Violation {
                file: path.to_string(),
                line: i + 1,
                message: format!("float type: {}", trimmed),
            });
        }
    }
    violations
}

fn no_heap_in_kernel(content: &str, path: &str) -> Vec<Violation> {
    let mut violations = Vec::new();
    if path.contains("/tools/") || path.contains("/tests/") {
        return violations;
    }
    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("///") {
            continue;
        }
        if trimmed.contains("heapless") {
            continue;
        }
        if trimmed.contains("std::vec::Vec")
            || trimmed.contains("std::collections")
            || trimmed.contains("std::boxed::Box")
        {
            violations.push(Violation {
                file: path.to_string(),
                line: i + 1,
                message: format!("heap: {}", trimmed),
            });
        }
    }
    violations
}

pub fn all_rules() -> Vec<ConstitutionalRule> {
    vec![
        ConstitutionalRule {
            id: "CONST-001",
            severity: RuleSeverity::Fatal,
            check: no_unsafe_outside_boundary,
        },
        ConstitutionalRule {
            id: "CONST-002",
            severity: RuleSeverity::Warning,
            check: no_floats_in_consensus,
        },
        ConstitutionalRule {
            id: "CONST-003",
            severity: RuleSeverity::Fatal,
            check: no_heap_in_kernel,
        },
    ]
}
