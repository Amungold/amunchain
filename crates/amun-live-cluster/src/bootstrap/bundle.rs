use amun_networking::peer_identity::PeerId;
use amun_networking::validator_certificate::ValidatorCertificate;

#[derive(Debug, Clone)]
pub struct IdentityBundle {
    pub validator_id: [u8; 32],
    pub peer_id: PeerId,
    pub public_key: [u8; 32],
    pub certificate: Option<ValidatorCertificate>,
}
