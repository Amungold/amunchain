use amun_authority_registry::{AuthorityRegistry, ConstitutionalAuthority};
use crate::config::load_genesis_authority;

/// Build the authority registry from genesis.
pub fn build_authority_registry() -> AuthorityRegistry {
    let genesis = load_genesis_authority(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/genesis/genesis_authority.json"
    ));
    let authority = ConstitutionalAuthority::new(
        genesis.authority_public_key,
        genesis.authority_version,
        0,
    );
    AuthorityRegistry::from_genesis(authority)
}
