#![allow(clippy::too_many_arguments)]
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use amun_kernel_types::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionType {
    Transfer,
    Stake,
    Unstake,
    ContractCall,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnsignedTransaction {
    pub tx_type: TransactionType,
    pub version: u8,
    pub chain_id: ChainId,
    pub nonce: Nonce,
    pub sender_pubkey: PublicKey,
    pub recipient: PublicHash32,
    pub amount: Amount,
    pub gas_limit: Gas,
    pub payload: heapless::Vec<u8, 128>,
}

impl UnsignedTransaction {
    pub fn new_transfer(
        version: u8,
        chain_id: ChainId,
        nonce: Nonce,
        sender_pubkey: PublicKey,
        recipient: PublicHash32,
        amount: Amount,
        gas_limit: Gas,
        payload: &[u8],
    ) -> AmunResult<Self> {
        Self::build(
            TransactionType::Transfer,
            version,
            chain_id,
            nonce,
            sender_pubkey,
            recipient,
            amount,
            gas_limit,
            payload,
        )
    }

    pub fn new_stake(
        version: u8,
        chain_id: ChainId,
        nonce: Nonce,
        sender_pubkey: PublicKey,
        validator: PublicHash32,
        amount: Amount,
        gas_limit: Gas,
    ) -> AmunResult<Self> {
        Self::build(
            TransactionType::Stake,
            version,
            chain_id,
            nonce,
            sender_pubkey,
            validator,
            amount,
            gas_limit,
            &[],
        )
    }

    pub fn new_unstake(
        version: u8,
        chain_id: ChainId,
        nonce: Nonce,
        sender_pubkey: PublicKey,
        validator: PublicHash32,
        amount: Amount,
        gas_limit: Gas,
    ) -> AmunResult<Self> {
        Self::build(
            TransactionType::Unstake,
            version,
            chain_id,
            nonce,
            sender_pubkey,
            validator,
            amount,
            gas_limit,
            &[],
        )
    }

    pub fn new_contract_call(
        version: u8,
        chain_id: ChainId,
        nonce: Nonce,
        sender_pubkey: PublicKey,
        contract: PublicHash32,
        gas_limit: Gas,
        payload: &[u8],
    ) -> AmunResult<Self> {
        Self::build(
            TransactionType::ContractCall,
            version,
            chain_id,
            nonce,
            sender_pubkey,
            contract,
            Amount(0),
            gas_limit,
            payload,
        )
    }

    fn build(
        tx_type: TransactionType,
        version: u8,
        chain_id: ChainId,
        nonce: Nonce,
        sender_pubkey: PublicKey,
        recipient: PublicHash32,
        amount: Amount,
        gas_limit: Gas,
        payload: &[u8],
    ) -> AmunResult<Self> {
        if chain_id.0 == 0 {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x000E,
                0x0001,
            ));
        }
        if gas_limit.0 == 0 || gas_limit.0 > 10_000_000 {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x000E,
                0x0002,
            ));
        }
        let mut pl = heapless::Vec::new();
        if !payload.is_empty() {
            pl.extend_from_slice(payload).map_err(|_| {
                FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000E, 0x0003)
            })?;
        }
        Ok(Self {
            tx_type,
            version,
            chain_id,
            nonce,
            sender_pubkey,
            recipient,
            amount,
            gas_limit,
            payload: pl,
        })
    }

    pub fn validate_basic(&self) -> AmunResult<()> {
        if self.chain_id.0 == 0 {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x000E,
                0x0010,
            ));
        }
        if self.gas_limit.0 == 0 || self.gas_limit.0 > 10_000_000 {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x000E,
                0x0011,
            ));
        }
        if self.sender_pubkey.0.iter().all(|&b| b == 0) {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x000E,
                0x0012,
            ));
        }
        Ok(())
    }
}
