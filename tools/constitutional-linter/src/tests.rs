#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::rules::*;

    #[test]
    fn test_no_unsafe_outside_boundary_allows_safe_code() {
        let rules = all_rules();
        let rule = &rules[0];
        let violations = (rule.check)("fn foo() { let x = 1; }", "crates/test/src/lib.rs");
        assert!(violations.is_empty());
    }

    #[test]
    fn test_no_unsafe_detects_unsafe_block() {
        let rules = all_rules();
        let rule = &rules[0];
        let violations = (rule.check)("unsafe { *ptr }", "crates/test/src/lib.rs");
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_no_unsafe_allows_amun_unsafe_crate() {
        let rules = all_rules();
        let rule = &rules[0];
        let violations = (rule.check)("unsafe { *ptr }", "crates/amun-unsafe/src/lib.rs");
        assert!(violations.is_empty());
    }

    #[test]
    fn test_no_floats_detects_f32() {
        let rules = all_rules();
        let rule = &rules[1];
        let violations = (rule.check)("let x: f32 = 1.0;", "crates/test/src/lib.rs");
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_no_floats_allows_tools() {
        let rules = all_rules();
        let rule = &rules[1];
        // Rules check for "/tools/" in path
        let violations = (rule.check)(
            "let x: f32 = 1.0;",
            "/tools/constitutional-linter/src/main.rs",
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_no_heap_detects_std_vec() {
        let rules = all_rules();
        let rule = &rules[2];
        let violations = (rule.check)("use std::vec::Vec;", "crates/test/src/lib.rs");
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_no_heap_allows_heapless() {
        let rules = all_rules();
        let rule = &rules[2];
        let violations = (rule.check)("use heapless::Vec;", "crates/test/src/lib.rs");
        assert!(violations.is_empty());
    }

    #[test]
    fn test_no_heap_allows_tools() {
        let rules = all_rules();
        let rule = &rules[2];
        // Rules check for "/tools/" in path
        let violations = (rule.check)(
            "use std::vec::Vec;",
            "/tools/constitutional-linter/src/main.rs",
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_all_rules_exist() {
        let rules = all_rules();
        assert_eq!(rules.len(), 3);
        assert_eq!(rules[0].id, "CONST-001");
        assert_eq!(rules[1].id, "CONST-002");
        assert_eq!(rules[2].id, "CONST-003");
    }

    #[test]
    fn test_commented_unsafe_ignored() {
        let rules = all_rules();
        let rule = &rules[0];
        let violations = (rule.check)("// unsafe { }", "crates/test/src/lib.rs");
        assert!(violations.is_empty());
    }

    #[test]
    fn test_doc_comment_unsafe_ignored() {
        let rules = all_rules();
        let rule = &rules[0];
        let violations = (rule.check)("/// unsafe { }", "crates/test/src/lib.rs");
        assert!(violations.is_empty());
    }

    #[test]
    fn test_float_in_string_ignored() {
        let rules = all_rules();
        let rule = &rules[1];
        let violations = (rule.check)("let s = \"f32\";", "crates/test/src/lib.rs");
        assert!(violations.is_empty());
    }
}
