use amun_kernel_types::{PublicHash32, PublicKey, ChainId, Nonce, Amount, Gas};
use amun_transaction::tx::UnsignedTransaction;
use crate::types::SdkResult;

pub struct TransactionBuilder;

impl TransactionBuilder {
    pub fn build_transfer(chain_id: u64, nonce: u64, sender: PublicKey, recipient: PublicHash32, amount: u128, gas: u64) -> SdkResult<UnsignedTransaction> {
        match UnsignedTransaction::new_transfer(1, ChainId(chain_id), Nonce(nonce), sender, recipient, Amount(amount), Gas(gas), b"") {
            Ok(tx) => SdkResult::ok(tx),
            Err(_) => SdkResult::err("Transaction build failed"),
        }
    }

    pub fn build_stake(chain_id: u64, nonce: u64, sender: PublicKey, validator: PublicHash32, amount: u128, gas: u64) -> SdkResult<UnsignedTransaction> {
        match UnsignedTransaction::new_stake(1, ChainId(chain_id), Nonce(nonce), sender, validator, Amount(amount), Gas(gas)) {
            Ok(tx) => SdkResult::ok(tx),
            Err(_) => SdkResult::err("Stake transaction build failed"),
        }
    }
}
