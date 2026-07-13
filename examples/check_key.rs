use amun_networking::crypto_identity::PeerKeyPair;
fn main() {
    let kp = PeerKeyPair::from_seed([7u8; 32]);
    let pk = kp.verifying_key.to_bytes();
    println!("Public key from seed [7u8;32]: {}", hex::encode(pk));
}
