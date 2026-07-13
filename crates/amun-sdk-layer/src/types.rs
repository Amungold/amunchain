use amun_kernel_types::PublicHash32;

#[derive(Clone, Debug)]
pub struct SdkConfig {
    pub network_id: u8,
    pub chain_id: u16,
    pub rpc_endpoint: heapless::String<128>,
    pub validator_pubkey: PublicHash32,
}

#[derive(Clone, Debug)]
pub struct SdkResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<heapless::String<64>>,
}

impl<T> SdkResult<T> {
    pub fn ok(data: T) -> Self { Self { success: true, data: Some(data), error: None } }
    pub fn err(msg: &str) -> Self { let mut s = heapless::String::new(); s.push_str(msg).ok(); Self { success: false, data: None, error: Some(s) } }
}
