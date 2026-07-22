pub mod rpc;
pub use rpc::RpcError;

// ============================================================
// TEMPORARY: Compatibility aliases for old routes/services
// These will be REMOVED after migration to new handlers.
// ============================================================
pub type ApiError = RpcError;
pub type ApiResult<T> = Result<T, RpcError>;
