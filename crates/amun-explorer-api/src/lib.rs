// ============================================================
// NEW ARCHITECTURE (active)
// ============================================================
pub mod config;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod response;
pub mod rpc;
pub mod state;

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
