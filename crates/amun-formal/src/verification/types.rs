use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupplyComponent {
    pub id: &'static str,
    pub amount: u64,        // u64 للتوافق مع المشروع الحالي
}

/// صورة كاملة للعرض، تُستخدم في التحقق
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupplyBreakdown {
    pub version: u32,
    pub timestamp: u64,
    pub components: BTreeMap<&'static str, u64>,
    pub total_supply: u64,
}

impl SupplyBreakdown {
    pub fn computed_total(&self) -> u64 {
        self.components.values().sum()
    }

    pub fn component_list(&self) -> Vec<SupplyComponent> {
        self.components
            .iter()
            .map(|(&id, &amount)| SupplyComponent { id, amount })
            .collect()
    }
}
