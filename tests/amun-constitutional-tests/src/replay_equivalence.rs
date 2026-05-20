use amun_state_root::root::StateLeaf;
use amun_state_root::StateRootEngine;

#[test]
fn test_replay_equivalence_same_state_same_root() {
    let leaves = vec![
        StateLeaf {
            key: "a".into(),
            value: vec![1],
        },
        StateLeaf {
            key: "b".into(),
            value: vec![2],
        },
    ];
    let root1 = StateRootEngine::domain_root(&leaves).expect("root1");
    let root2 = StateRootEngine::domain_root(&leaves).expect("root2");
    assert_eq!(root1, root2);
}

#[test]
fn test_replay_equivalence_deterministic_ordering() {
    let leaves = vec![
        StateLeaf {
            key: "c".into(),
            value: vec![3],
        },
        StateLeaf {
            key: "a".into(),
            value: vec![1],
        },
    ];
    let root = StateRootEngine::domain_root(&leaves).expect("root");
    let leaves_sorted = vec![
        StateLeaf {
            key: "a".into(),
            value: vec![1],
        },
        StateLeaf {
            key: "c".into(),
            value: vec![3],
        },
    ];
    let root_sorted = StateRootEngine::domain_root(&leaves_sorted).expect("root_sorted");
    assert_eq!(root, root_sorted, "Root must be order-independent");
}
