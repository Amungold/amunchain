use amun_resource_core::{
    RegistryError, ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata,
    ResourceRegistry, ResourceState,
};

#[test]
fn n130_duplicate_token_rejected() {
    let mut reg = ResourceRegistry::new(10);
    let col_id = ResourceId([1u8; 32]);

    // register collection
    let meta = ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [10u8; 32],
    };
    reg.register_genesis(meta).unwrap();

    let token1 = ResourceId([2u8; 32]);
    // احصل على الهاش والإصدار قبل الاستعارة القابلة للتغيير
    let parent_hash = reg.resource_hash(&col_id).unwrap();
    let parent_version = reg.get(&col_id).unwrap().lineage.version;

    let child_meta = ResourceMetadata {
        resource_id: token1,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::single_ancestor(token1, col_id, parent_hash, parent_version + 1),
        contract_id: [0u8; 32],
        owner: [1u8; 32],
    };

    assert!(reg
        .derive_from_collection(&col_id, child_meta.clone())
        .is_ok());
    // محاولة تكرار نفس الـ id
    assert!(reg.derive_from_collection(&col_id, child_meta).is_err());
}

#[test]
fn n130_invalid_parent_hash_rejected() {
    let mut reg = ResourceRegistry::new(10);
    let col_id = ResourceId([1u8; 32]);
    let meta = ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [10u8; 32],
    };
    reg.register_genesis(meta).unwrap();

    let bad_hash = [0xFFu8; 32];
    let child = ResourceMetadata {
        resource_id: ResourceId([3u8; 32]),
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::single_ancestor(ResourceId([3u8; 32]), col_id, bad_hash, 2),
        contract_id: [0u8; 32],
        owner: [1u8; 32],
    };
    assert!(matches!(
        reg.derive_from_collection(&col_id, child),
        Err(RegistryError::ParentHashMismatch(_))
    ));
}

#[test]
fn n130_collection_remains_active() {
    let mut reg = ResourceRegistry::new(10);
    let col_id = ResourceId([1u8; 32]);
    let meta = ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [10u8; 32],
    };
    reg.register_genesis(meta).unwrap();

    for i in 2..5u8 {
        let token_id = ResourceId([i; 32]);
        // حساب التبعيات قبل الاستعارة
        let parent_hash = reg.resource_hash(&col_id).unwrap();
        let parent_version = reg.get(&col_id).unwrap().lineage.version;

        let child = ResourceMetadata {
            resource_id: token_id,
            archetype: ResourceArchetype::NFTAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(
                token_id,
                col_id,
                parent_hash,
                parent_version + 1,
            ),
            contract_id: [0u8; 32],
            owner: [i; 32],
        };
        reg.derive_from_collection(&col_id, child).unwrap();
    }

    let collection = reg.get(&col_id).unwrap();
    assert!(matches!(collection.state, ResourceState::Active));
    assert_eq!(reg.total_active(), 4); // 1 collection + 3 NFTs
}

#[test]
fn n130_replay_determinism() {
    let mut reg1 = ResourceRegistry::new(10);
    let mut reg2 = ResourceRegistry::new(10);
    let col_id = ResourceId([1u8; 32]);

    let setup = |reg: &mut ResourceRegistry| {
        reg.register_genesis(ResourceMetadata {
            resource_id: col_id,
            archetype: ResourceArchetype::NFTCollection,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(col_id),
            contract_id: [0u8; 32],
            owner: [10u8; 32],
        })
        .unwrap();

        for i in 2..4u8 {
            let id = ResourceId([i; 32]);
            // احصل على الهاش داخل الإغلاق
            let hash = reg.resource_hash(&col_id).unwrap();
            let version = reg.get(&col_id).unwrap().lineage.version;

            reg.derive_from_collection(
                &col_id,
                ResourceMetadata {
                    resource_id: id,
                    archetype: ResourceArchetype::NFTAsset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::single_ancestor(id, col_id, hash, version + 1),
                    contract_id: [0u8; 32],
                    owner: [i; 32],
                },
            )
            .unwrap();
        }
    };

    setup(&mut reg1);
    setup(&mut reg2);
    assert_eq!(reg1.compute_state_root(), reg2.compute_state_root());
}
