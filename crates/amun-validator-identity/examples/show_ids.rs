use amun_validator_identity::derive_validator_id;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

fn main() {
    let mut csprng = OsRng;
    for i in 0..4 {
        let signing_key = SigningKey::generate(&mut csprng);
        let pk = signing_key.verifying_key().to_bytes();
        let vid = derive_validator_id(&pk);
        println!("Validator {}: id={:?}", i, &vid[..8]);
    }
}
