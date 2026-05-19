#[derive(Debug, Clone)]
pub struct ByzantineScenario {
    pub name: String,
    pub description: String,
    pub byzantine_validators: usize,
    pub total_validators: usize,
    pub expect_equivocation_detection: bool,
    pub expect_safety_violation: bool,
}

impl ByzantineScenario {
    pub fn equivocation_attack() -> Self {
        Self {
            name: "equivocation_attack".to_string(),
            description: "One validator signs conflicting votes at same round".to_string(),
            byzantine_validators: 1,
            total_validators: 4,
            expect_equivocation_detection: true,
            expect_safety_violation: true,
        }
    }

    pub fn triple_equivocation() -> Self {
        Self {
            name: "triple_equivocation".to_string(),
            description: "Three validators equivocate simultaneously".to_string(),
            byzantine_validators: 3,
            total_validators: 4,
            expect_equivocation_detection: true,
            expect_safety_violation: true,
        }
    }

    pub fn honest() -> Self {
        Self {
            name: "honest".to_string(),
            description: "All validators follow protocol".to_string(),
            byzantine_validators: 0,
            total_validators: 4,
            expect_equivocation_detection: false,
            expect_safety_violation: false,
        }
    }
}
