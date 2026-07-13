#![allow(clippy::uninlined_format_args)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::stable_sort_primitive)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::similar_names)]
#![allow(clippy::float_cmp)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::unused_self)]
#![allow(clippy::missing_fields_in_debug)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::new_without_default)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_borrows_for_generic_args)]
pub mod network_messages;
pub mod peer_discovery;
pub mod rejoin_protocol;
pub mod sync_transport;
pub mod validator_identity;

pub use network_messages::*;
pub use peer_discovery::*;
pub use rejoin_protocol::*;
pub use sync_transport::*;
pub use validator_identity::*;

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::{
        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
        ResourceState,
    };

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32];
        h[0] = seed;
        ResourceId(h)
    }

    fn build_registry(count: u8) -> ResourceRegistry {
        let mut reg = ResourceRegistry::new(10000);
        for i in 0..count {
            reg.register_genesis(ResourceMetadata {
                resource_id: make_id(i),
                archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::genesis(make_id(i)),
                contract_id: [1u8; 32],
                owner: [2u8; 32],
            })
            .unwrap();
        }
        reg
    }

    #[test]
    fn n57_peer_registry_register_and_lookup() {
        let mut reg = PeerRegistry::new();
        let id = ValidatorIdentity::new([1u8; 32], [2u8; 32], "10.0.0.1".into(), 9000);
        let fp = id.fingerprint();
        reg.register(id);
        assert_eq!(reg.count(), 1);
        assert!(reg.get(&fp).is_some());
    }

    #[test]
    fn n57_peer_registry_remove() {
        let mut reg = PeerRegistry::new();
        let id = ValidatorIdentity::new([1u8; 32], [2u8; 32], "10.0.0.1".into(), 9000);
        let fp = id.fingerprint();
        reg.register(id);
        reg.remove(&fp);
        assert_eq!(reg.count(), 0);
    }

    #[test]
    fn n57_sync_export_and_import() {
        let source_reg = build_registry(50);
        let state_root = source_reg.compute_state_root();
        let history_root = [0xab; 32];

        let package = SyncTransport::export_snapshot(
            &source_reg,
            100,
            [0xcd; 32],
            history_root,
            "test".into(),
        );

        let result = SyncTransport::import_snapshot(&package, history_root);
        assert!(result.is_ok());
        let imported_reg = result.unwrap();
        assert_eq!(imported_reg.compute_state_root(), state_root);
        assert_eq!(imported_reg.total(), 50);
    }

    #[test]
    fn n57_sync_reject_wrong_history_root() {
        let source_reg = build_registry(10);
        let history_root = [0xab; 32];

        let package =
            SyncTransport::export_snapshot(&source_reg, 1, [0u8; 32], history_root, "test".into());

        let result = SyncTransport::import_snapshot(&package, [0x99; 32]);
        assert!(result.is_err());
    }

    #[test]
    fn n57_rejoin_protocol_full_roundtrip() {
        // Source node has state
        let source_reg = build_registry(100);
        let _state_root = source_reg.compute_state_root();
        let history_root = [0x10; 32];
        let block_hash = [0x20; 32];

        // Rejoining node trusts history_root and executes rejoin
        let result =
            RejoinProtocol::rejoin(&source_reg, 42, block_hash, history_root, history_root);

        match result {
            RejoinResult::Rejoined {
                height,
                resources_imported,
            } => {
                assert_eq!(height, 42);
                assert_eq!(resources_imported, 100);
            }
            RejoinResult::Failed { reason } => {
                panic!("Rejoin failed: {reason}");
            }
        }
    }

    #[test]
    fn n57_rejoin_rejects_wrong_history() {
        let source_reg = build_registry(10);
        let result = RejoinProtocol::rejoin(&source_reg, 1, [0u8; 32], [0x99; 32], [0x10; 32]);
        assert!(matches!(result, RejoinResult::Failed { .. }));
    }

    #[test]
    fn n57_network_message_serialization() {
        let msg = NetworkMessage::StateSyncRequest {
            request_id: [0xaa; 32],
            height: 100,
            requester_id: [0xbb; 32],
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: NetworkMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, parsed);
    }
}
