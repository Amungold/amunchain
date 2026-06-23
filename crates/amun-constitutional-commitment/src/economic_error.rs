use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EconomicError {
    #[error("Invalid circulating supply: computed {computed}, stored {stored}")]
    InvalidCirculatingSupply { computed: u64, stored: u64 },
}
