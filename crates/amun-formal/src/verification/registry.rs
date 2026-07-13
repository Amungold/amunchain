use super::invariant::Invariant;
use super::category::VerificationCategory;
use std::collections::BTreeMap;

pub struct InvariantRegistry {
    items: Vec<Box<dyn Invariant>>,
}

impl InvariantRegistry {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn register(&mut self, inv: Box<dyn Invariant>) {
        self.items.push(inv);
    }

    /// تجميع حسب الفئة، وترتيب داخلي حسب الأولوية ثم الاسم
    pub fn grouped_by_category(&self) -> BTreeMap<VerificationCategory, Vec<&dyn Invariant>> {
        let mut map: BTreeMap<VerificationCategory, Vec<&dyn Invariant>> = BTreeMap::new();
        for inv in &self.items {
            map.entry(inv.category()).or_default().push(inv.as_ref());
        }
        for list in map.values_mut() {
            list.sort_by(|a, b| {
                a.priority()
                    .cmp(&b.priority())
                    .then_with(|| a.name().cmp(b.name()))
            });
        }
        map
    }
}
