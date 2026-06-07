use crate::block::ConstitutionalBlock;
use crate::chain::Blockchain;
use amun_consensus::action::ActionLog;
use amun_constitutional_state::ConstitutionalStateRuntime;

#[derive(Debug, Clone)]
pub struct FinalizationContext {
    pub state_runtime: Option<ConstitutionalStateRuntime>,
    pub pre_state_root: [u8; 32],
    pub governance_root: String,
    pub execution_root: String,
    pub timestamp: String,
    pub proposer: String,
}

pub struct BlockFinalizer;

impl BlockFinalizer {
    pub fn finalize(
        chain: &mut Blockchain,
        action_log: &ActionLog,
        ctx: FinalizationContext,
    ) -> Result<ConstitutionalBlock, String> {
        let parent_hash = chain.blocks.last()
            .map(|b| b.block_hash.clone())
            .unwrap_or_else(|| "0".repeat(64));

        let height = chain.blocks.last()
            .map(|b| b.block_height + 1)
            .unwrap_or(0);

        let evidence_root = hex::encode(action_log.evidence_root());

        let (state_root, replay_certificate_root) = match &ctx.state_runtime {
            Some(rt) => {
                let cert = rt.create_certificate(height, ctx.pre_state_root);
                let merkle_root = ConstitutionalStateRuntime::certificate_merkle_root(&[cert]);
                (hex::encode(rt.state_root()), hex::encode(merkle_root))
            }
            None => (String::new(), String::new()),
        };

        let block = ConstitutionalBlock::new(
            height,
            parent_hash,
            ctx.timestamp,
            ctx.proposer,
            vec![],
            state_root,
            ctx.governance_root,
            ctx.execution_root,
            evidence_root,
            replay_certificate_root,
        );

        chain.append(block.clone())?;
        Ok(block)
    }
}
