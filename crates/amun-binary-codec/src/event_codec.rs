use crate::codec::{CanonicalDecoder, CanonicalEncoder};
use amun_chain_position::ChainPosition;

// Re-export the codec functions. The ProtocolEvent type lives in amun-protocol-event.
// These functions work with the ProtocolEvent type from that crate.

pub fn encode_event_position(enc: &mut CanonicalEncoder, position: ChainPosition) {
    enc.write_u64(position.epoch);
    enc.write_u64(position.sequence);
}

pub fn decode_event_position(dec: &mut CanonicalDecoder) -> Option<ChainPosition> {
    let epoch = dec.read_u64()?;
    let sequence = dec.read_u64()?;
    Some(ChainPosition::new(epoch, sequence))
}
