use bytes::Bytes;

/// Network payload type — zero-copy reference counted bytes.
/// Replaces `Vec<u8>` throughout the networking layer.
pub type Payload = Bytes;

/// Create a Payload from static data (no allocation).
pub fn payload_from_static(data: &'static [u8]) -> Payload {
    Bytes::from_static(data)
}

/// Create a Payload from owned data (single allocation).
pub fn payload_from_vec(data: Vec<u8>) -> Payload {
    Bytes::from(data)
}
