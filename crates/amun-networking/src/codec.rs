use crate::envelope::Envelope;

/// Serialize an envelope into bytes for transmission.
pub fn serialize_envelope(envelope: &Envelope) -> Result<Vec<u8>, String> {
    serde_json::to_vec(envelope).map_err(|e| e.to_string())
}

/// Deserialize bytes back into an envelope.
pub fn deserialize_envelope(data: &[u8]) -> Result<Envelope, String> {
    serde_json::from_slice(data).map_err(|e| e.to_string())
}
