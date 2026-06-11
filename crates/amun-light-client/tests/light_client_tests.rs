use amun_constitutional_runtime::block_validator::BlockValidationResult;
use amun_constitutional_runtime::certificate_chain::CertificateChain;
use amun_constitutional_runtime::finality_certificate::ConstitutionalFinalityCertificate;
use amun_constitutional_runtime::history_root::ConstitutionalHistoryRoot;
use amun_light_client::constitutional_client::{
    ConstitutionalCheckpoint, ConstitutionalLightClient,
};
use amun_resource_core::ResourceId;
use amun_transition_proof::transition_proof::TransitionProof;

fn make_id(seed: u8) -> ResourceId {
    let mut h = [0u8; 32];
    h[0] = seed;
    ResourceId(h)
}

fn make_cert(
    height: u64,
    state_root: [u8; 32],
    qc_hash: [u8; 32],
    prev_hash: [u8; 32],
) -> ConstitutionalFinalityCertificate {
    let block_result = BlockValidationResult {
        total_transactions: 1,
        committed: 1,
        rejected: 0,
        pccv_verified: 1,
        pccv_failed: 0,
        block_valid: true,
        state_root,
    };
    let transitions = vec![TransitionProof::new(
        [0xaa; 32],
        make_id(1),
        height,
        [0u8; 32],
        [0u8; 32],
        state_root,
        vec![],
        vec![],
        vec![],
        vec![],
        0,
    )];
    let mut cert = ConstitutionalFinalityCertificate::issue(
        &block_result,
        transitions,
        qc_hash,
        height,
        [0xbb; 32],
    );
    cert.previous_certificate_hash = prev_hash;
    cert.certificate_hash = cert.compute_hash();
    cert
}

#[test]
fn n55_light_client_full_workflow() {
    let cert1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
    let chain1 = CertificateChain::new(vec![cert1.clone()]).unwrap();
    let history_root = ConstitutionalHistoryRoot::from_chain(&chain1);

    let checkpoint = ConstitutionalCheckpoint {
        block_height: cert1.block_height,
        block_hash: cert1.block_hash,
        certificate_hash: cert1.certificate_hash,
        history_root: history_root.history_root,
        state_root: cert1.state_root,
        proof_root: cert1.proof_root,
        evidence_root: cert1.evidence_root,
        pccv_root: cert1.pccv_root,
    };

    let mut client = ConstitutionalLightClient::new();
    client.bootstrap(checkpoint);
    assert_eq!(client.trusted_height(), Some(1));

    let cert2 = make_cert(2, [0x02; 32], [0xdd; 32], cert1.certificate_hash);
    let cert3 = make_cert(3, [0x03; 32], [0xee; 32], cert2.certificate_hash);
    let chain2 = CertificateChain::new(vec![cert2, cert3]).unwrap();

    assert!(client.advance(&chain2).is_ok());
    assert_eq!(client.trusted_height(), Some(3));
}

#[test]
fn n55_light_client_rejects_broken_chain() {
    let cert1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
    let checkpoint = ConstitutionalCheckpoint {
        block_height: cert1.block_height,
        block_hash: cert1.block_hash,
        certificate_hash: cert1.certificate_hash,
        history_root: [0u8; 32],
        state_root: cert1.state_root,
        proof_root: cert1.proof_root,
        evidence_root: cert1.evidence_root,
        pccv_root: cert1.pccv_root,
    };

    let mut client = ConstitutionalLightClient::new();
    client.bootstrap(checkpoint);

    // Create a chain that doesn't link to the checkpoint
    let broken_cert = make_cert(2, [0xff; 32], [0xdd; 32], [0xde; 32]); // wrong prev_hash
    let chain = CertificateChain::new(vec![broken_cert]).unwrap();

    assert!(!client.verify_chain_extension(&chain));
    assert!(client.advance(&chain).is_err());
}
