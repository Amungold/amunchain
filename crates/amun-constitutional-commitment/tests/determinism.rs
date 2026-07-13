use amun_constitutional_commitment::{
    commitment_root, compute_constitutional_root, ConstitutionalCommitment, Hash32,
};

fn make_hash(byte: u8) -> Hash32 {
    [byte; 32]
}

#[test]
fn identical_inputs_produce_identical_constitutional_root() {
    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);
    let ec = make_hash(0x44);

    let root_a = compute_constitutional_root(id, ev, gv, ec);
    let root_b = compute_constitutional_root(id, ev, gv, ec);
    assert_eq!(root_a, root_b);
}

#[test]
fn single_byte_change_produces_different_constitutional_root() {
    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);
    let ec = make_hash(0x44);

    let root_a = compute_constitutional_root(id, ev, gv, ec);

    let mut id2 = id;
    id2[0] ^= 1;
    let root_b = compute_constitutional_root(id2, ev, gv, ec);
    assert_ne!(root_a, root_b);
}

#[test]
fn identical_commitments_produce_identical_commitment_root() {
    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);
    let ec = make_hash(0x44);
    let cr = compute_constitutional_root(id, ev, gv, ec);

    let c1 = ConstitutionalCommitment {
        version: 1,
        identity_root: id,
        evidence_root: ev,
        governance_root: gv,
        economic_root: ec,
        constitutional_root: cr,
    };

    let c2 = ConstitutionalCommitment {
        version: 1,
        identity_root: id,
        evidence_root: ev,
        governance_root: gv,
        economic_root: ec,
        constitutional_root: cr,
    };

    assert_eq!(commitment_root(&c1), commitment_root(&c2));
}

#[test]
fn single_byte_change_in_commitment_produces_different_commitment_root() {
    let id = make_hash(0x11);
    let ev = make_hash(0x22);
    let gv = make_hash(0x33);
    let ec = make_hash(0x44);
    let cr = compute_constitutional_root(id, ev, gv, ec);

    let mut c = ConstitutionalCommitment {
        version: 1,
        identity_root: id,
        evidence_root: ev,
        governance_root: gv,
        economic_root: ec,
        constitutional_root: cr,
    };

    let root_a = commitment_root(&c);
    c.version = 2;
    let root_b = commitment_root(&c);
    assert_ne!(root_a, root_b);
}
