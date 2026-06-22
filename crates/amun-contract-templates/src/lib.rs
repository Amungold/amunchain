use amun_bytecode::OpCode;

pub struct ContractTemplate {
    pub name: String,
    pub code: Vec<OpCode>,
}

pub fn token_template() -> ContractTemplate {
    ContractTemplate {
        name: "Token".into(),
        code: vec![
            OpCode::Push(0),       // total_supply
            OpCode::Push(18),      // decimals
            OpCode::Push(0),       // balance_of(owner)
            OpCode::Halt,
        ],
    }
}

pub fn nft_template() -> ContractTemplate {
    ContractTemplate {
        name: "NFT".into(),
        code: vec![
            OpCode::Push(1),       // token_id
            OpCode::Push(0),       // owner
            OpCode::Push(0),       // metadata_uri_ptr
            OpCode::Halt,
        ],
    }
}
