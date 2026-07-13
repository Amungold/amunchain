use crate::config::ValidatorConfig;
use amun_networking::peer_identity::PeerId;
use amun_validator_identity::derive_validator_id;
use ed25519_dalek::SigningKey;

#[derive(Clone)]
pub struct IdentityBundle{
    pub signing_key:SigningKey,
    pub peer_id:PeerId,
    pub public_key:[u8;32],
    pub validator_id:[u8;32],
}

#[derive(Clone)]
pub struct IdentityContext{
    pub bundle:IdentityBundle,
}

pub struct IdentityBootstrap;

pub fn load_identity(
    config:&ValidatorConfig
)->Result<IdentityContext,Box<dyn std::error::Error>>{

    Ok(IdentityBootstrap::load(config))
}

impl IdentityBootstrap{

    pub fn load(config:&ValidatorConfig)->IdentityContext{

        let mut seed=[0u8;32];
        seed[0]=config.validator_id[0];

        let signing_key=SigningKey::from_bytes(&seed);

        let public_key=signing_key.verifying_key().to_bytes();

        let validator_id=derive_validator_id(&public_key);

        let peer_id=PeerId::from_bytes(public_key);

        IdentityContext{
            bundle:IdentityBundle{
                signing_key,
                peer_id,
                public_key,
                validator_id,
            }
        }
    }
}
