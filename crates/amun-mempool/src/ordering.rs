use amun_transaction::tx::UnsignedTransaction;

pub struct CanonicalOrdering;

impl CanonicalOrdering {
    pub fn order(txs: &mut [UnsignedTransaction]) {
        txs.sort_unstable_by(|a, b| {
            b.gas_limit
                .0
                .cmp(&a.gas_limit.0)
                .then(a.nonce.0.cmp(&b.nonce.0))
                .then(a.sender_pubkey.0.cmp(&b.sender_pubkey.0))
        });
    }
}
