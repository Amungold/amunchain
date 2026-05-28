// Constitutional Compatibility Engine
// Determines how civilizations relate across evolution events.

pub mod matrix;
pub mod migration;

pub use matrix::CompatibilityMatrix;
pub use migration::{MigrationProof, MigrationRules};
