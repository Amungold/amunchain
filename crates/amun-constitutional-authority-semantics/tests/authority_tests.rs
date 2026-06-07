use amun_constitutional_signing::ConstitutionalKeyPair;
use amun_constitutional_authority_semantics::capability::{AuthorityCapability, CapabilityWitness};
use amun_constitutional_authority_semantics::delegation::DelegationChain;
use amun_constitutional_authority_semantics::revocation::{RevocationWitness, RevocationRegistry};

#[test]
fn test_capability_determinism() {
    let a = AuthorityCapability::new("vote".into(), "governance".into(), "key".into(), "t1".into(), "t2".into(), vec![]);
    let b = AuthorityCapability::new("vote".into(), "governance".into(), "key".into(), "t1".into(), "t2".into(), vec![]);
    assert_eq!(a.capability_id, b.capability_id);
}

#[test]
fn test_delegation_chain_valid() {
    let root_key = ConstitutionalKeyPair::generate();
    let delegate1 = ConstitutionalKeyPair::generate();
    let delegate2 = ConstitutionalKeyPair::generate();

    let cap1 = AuthorityCapability::new("vote".into(), "governance".into(), delegate1.verifying_key_hex(), "2026-01-01".into(), "2030-01-01".into(), vec![]);
    let w1 = CapabilityWitness::sign(cap1, &root_key);

    let cap2 = AuthorityCapability::new("vote".into(), "governance".into(), delegate2.verifying_key_hex(), "2027-01-01".into(), "2029-12-31".into(), vec![]);
    let w2 = CapabilityWitness::sign(cap2, &delegate1);

    let mut chain = DelegationChain::new(w1);
    assert!(chain.append(w2).is_ok());
    assert!(chain.verify().is_ok());
}

#[test]
fn test_delegation_chain_rejects_broken() {
    let root_key = ConstitutionalKeyPair::generate();
    let delegate1 = ConstitutionalKeyPair::generate();
    let wrong_key = ConstitutionalKeyPair::generate();

    let cap1 = AuthorityCapability::new("vote".into(), "governance".into(), delegate1.verifying_key_hex(), "2026-01-01".into(), "2030-01-01".into(), vec![]);
    let w1 = CapabilityWitness::sign(cap1, &root_key);

    let cap2 = AuthorityCapability::new("vote".into(), "governance".into(), delegate1.verifying_key_hex(), "2027-01-01".into(), "2029-12-31".into(), vec![]);
    let w2 = CapabilityWitness::sign(cap2, &wrong_key);

    let mut chain = DelegationChain::new(w1);
    assert!(chain.append(w2).is_err());
}

#[test]
fn test_revocation_registry() {
    let mut reg = RevocationRegistry::new();
    let w = RevocationWitness { capability_id: "cap-1".into(), revoked_by: "root".into(), timestamp: "t".into() };
    reg.revoke(&w);
    assert!(reg.is_revoked("cap-1"));
    assert!(!reg.is_revoked("cap-2"));
}
