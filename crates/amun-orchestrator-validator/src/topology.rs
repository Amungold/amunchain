use crate::config_gen::ClusterEntry;
use crate::inventory::ValidatorInventory;

pub fn build_cluster(inv: &ValidatorInventory) -> Vec<ClusterEntry> {
    inv.validators
        .iter()
        .map(|v| ClusterEntry {
            validator_id: v.validator_id,
            address: v.address.to_string(),
            certificate_path: v.certificate_path.display().to_string(),
        })
        .collect()
}
