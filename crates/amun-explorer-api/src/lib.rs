// ============================================================
// NEW ARCHITECTURE (active)
// ============================================================
pub mod config;
pub mod state;
pub mod error;
pub mod rpc;
pub mod handlers;
pub mod middleware;
pub mod response;

// ============================================================
// OLD ARCHITECTURE (disabled during migration)
// Will be removed after all services migrate to new handlers.
// ============================================================
// pub mod errors;
// pub mod server;
// pub mod types;
// pub mod routes { ... }
// pub mod services { ... }

// Re-export for temporary compatibility
pub use error as errors;
