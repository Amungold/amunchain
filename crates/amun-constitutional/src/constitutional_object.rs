use crate::constitutional_failure::ConstitutionalFailure;

pub trait ConstitutionalObject {
    fn constitutional_hash(&self) -> [u8; 32];
    fn verify_structure(&self) -> Result<(), ConstitutionalFailure>;
    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure>;
    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure>;
    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
        Ok(())
    }
    fn verify(&self) -> Result<(), ConstitutionalFailure> {
        self.verify_structure()?;
        self.verify_semantics()?;
        self.verify_provenance()?;
        self.verify_constitutional()?;
        Ok(())
    }
}

pub trait ConstitutionalIdentity {
    fn schema_id(&self) -> u16;
    fn schema_version(&self) -> u16;
    fn constitutional_revision(&self) -> u32;
    fn replay_revision(&self) -> u32;
}
