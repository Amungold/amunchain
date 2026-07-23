use amun_merkle;

#[test]
fn fuzz_merkle_power_of_two_sizes() {
    for n in [0, 1, 2, 4, 8, 16, 32, 64] {
        let leaves: Vec<[u8; 32]> = (0..n).map(|i| [i as u8; 32]).collect();
        let root = amun_merkle::merkle_root(
            &leaves, amun_merkle::TX_LEAF_DOMAIN,
            amun_merkle::TX_NODE_DOMAIN, amun_merkle::empty_tx_root(),
        );
        if n == 0 { assert_eq!(root, amun_merkle::empty_tx_root()); }
        else { assert_ne!(root, [0u8; 32]); }
    }
}

#[test]
fn fuzz_merkle_odd_sizes() {
    for n in [3, 5, 7, 9, 11, 13, 15] {
        let leaves: Vec<[u8; 32]> = (0..n).map(|i| [i as u8; 32]).collect();
        let root = amun_merkle::merkle_root(
            &leaves, amun_merkle::TX_LEAF_DOMAIN,
            amun_merkle::TX_NODE_DOMAIN, amun_merkle::empty_tx_root(),
        );
        assert_ne!(root, [0u8; 32], "Odd size {} must produce non-zero root", n);
    }
}

#[test]
fn fuzz_merkle_large_input() {
    let leaves: Vec<[u8; 32]> = (0..1000).map(|i| [i as u8; 32]).collect();
    let root1 = amun_merkle::merkle_root(
        &leaves, amun_merkle::TX_LEAF_DOMAIN,
        amun_merkle::TX_NODE_DOMAIN, amun_merkle::empty_tx_root(),
    );
    let root2 = amun_merkle::merkle_root(
        &leaves, amun_merkle::TX_LEAF_DOMAIN,
        amun_merkle::TX_NODE_DOMAIN, amun_merkle::empty_tx_root(),
    );
    assert_eq!(root1, root2);
}
