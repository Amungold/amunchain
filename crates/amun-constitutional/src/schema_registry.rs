//! Schema Registry — global constitutional namespace governance.
//!
//! Every constitutional object type has a unique schema ID.
//! This registry is the SINGLE source of truth for schema identities.
//!
//! RULES:
//!   1. Schema IDs are IMMUTABLE once assigned.
//!   2. Changing a schema ID requires a constitutional revision.
//!   3. Ranges are reserved: core (0x0001-0x00FF), runtime (0x0100-0x01FF),
//!      experimental (0xF000-0xFFFF).
//!   4. NO two constitutional object types may share a schema ID.
//!
//! This registry is COMPILE-TIME governance, not runtime metadata.

/// Reserved schema ID ranges.
pub mod schema_ranges {
    /// Constitutional core objects (0x0001 - 0x00FF)
    pub const CORE_START: u16 = 0x0001;
    pub const CORE_END: u16 = 0x00FF;

    /// Runtime substrate objects (0x0100 - 0x01FF)
    pub const RUNTIME_START: u16 = 0x0100;
    pub const RUNTIME_END: u16 = 0x01FF;

    /// Experimental / development (0xF000 - 0xFFFF)
    pub const EXPERIMENTAL_START: u16 = 0xF000;
    pub const EXPERIMENTAL_END: u16 = 0xFFFF;

    /// Check if a schema ID falls within a reserved range.
    pub const fn is_core(id: u16) -> bool {
        id >= CORE_START && id <= CORE_END
    }
    pub const fn is_runtime(id: u16) -> bool {
        id >= RUNTIME_START && id <= RUNTIME_END
    }
    pub const fn is_experimental(id: u16) -> bool {
        id >= EXPERIMENTAL_START
    }
    pub const fn is_reserved(id: u16) -> bool {
        is_core(id) || is_runtime(id) || is_experimental(id)
    }
}

/// Constitutional core schema IDs — IMMUTABLE.
///
/// These IDs are part of the constitutional ABI.
/// Any change requires a constitutional revision.
pub mod core_schemas {
    use super::schema_ranges;

    /// ConstitutionalFailure
    pub const CONSTITUTIONAL_FAILURE: u16 = 0x0004;
    /// ExecutionLimits
    pub const EXECUTION_LIMITS: u16 = 0x0005;
    /// ExecutionContext
    pub const EXECUTION_CONTEXT: u16 = 0x0006;
    /// ExecutionBoundary
    pub const EXECUTION_BOUNDARY: u16 = 0x0007;
    /// ExecutionJournal
    pub const EXECUTION_JOURNAL: u16 = 0x0008;
    /// TransitionEvidence
    pub const TRANSITION_EVIDENCE: u16 = 0x0009;
    /// TransitionCommitment
    pub const TRANSITION_COMMITMENT: u16 = 0x000A;
    /// JournalEntry
    pub const JOURNAL_ENTRY: u16 = 0x000B;
    /// ExecutionReceipt
    pub const EXECUTION_RECEIPT: u16 = 0x000C;
    /// ReplayCertificate
    pub const REPLAY_CERTIFICATE: u16 = 0x000D;
    /// ConstitutionalStateAnchor
    pub const STATE_ANCHOR: u16 = 0x000E;
    /// ConstitutionalSnapshot
    pub const SNAPSHOT: u16 = 0x000F;
    /// RestorationPoint
    pub const RESTORATION_POINT: u16 = 0x0010;
    /// ContinuationChain
    pub const CONTINUATION_CHAIN: u16 = 0x0011;
    /// DivergencePoint
    pub const DIVERGENCE_POINT: u16 = 0x0012;
    /// DivergenceResolution
    pub const DIVERGENCE_RESOLUTION: u16 = 0x0013;
    /// CausalEdge — FIXED: was 0x0014 colliding with ConstitutionalWitness
    pub const CAUSAL_EDGE: u16 = 0x0014;
    /// ConstitutionalWitness — FIXED: now 0x0015 (was 0x0014)
    pub const CONSTITUTIONAL_WITNESS: u16 = 0x0015;

    /// Verify that all core schema IDs are unique.
    pub const fn verify_uniqueness() -> bool {
        let ids = [
            CONSTITUTIONAL_FAILURE,
            EXECUTION_LIMITS,
            EXECUTION_CONTEXT,
            EXECUTION_BOUNDARY,
            EXECUTION_JOURNAL,
            TRANSITION_EVIDENCE,
            TRANSITION_COMMITMENT,
            JOURNAL_ENTRY,
            EXECUTION_RECEIPT,
            REPLAY_CERTIFICATE,
            STATE_ANCHOR,
            SNAPSHOT,
            RESTORATION_POINT,
            CONTINUATION_CHAIN,
            DIVERGENCE_POINT,
            DIVERGENCE_RESOLUTION,
            CAUSAL_EDGE,
            CONSTITUTIONAL_WITNESS,
        ];
        let mut i = 0;
        while i < ids.len() {
            let mut j = i + 1;
            while j < ids.len() {
                if ids[i] == ids[j] {
                    return false;
                }
                j += 1;
            }
            i += 1;
        }
        true
    }

    /// Verify all core schema IDs are in the core range.
    pub const fn verify_range() -> bool {
        let ids = [
            CONSTITUTIONAL_FAILURE,
            EXECUTION_LIMITS,
            EXECUTION_CONTEXT,
            EXECUTION_BOUNDARY,
            EXECUTION_JOURNAL,
            TRANSITION_EVIDENCE,
            TRANSITION_COMMITMENT,
            JOURNAL_ENTRY,
            EXECUTION_RECEIPT,
            REPLAY_CERTIFICATE,
            STATE_ANCHOR,
            SNAPSHOT,
            RESTORATION_POINT,
            CONTINUATION_CHAIN,
            DIVERGENCE_POINT,
            DIVERGENCE_RESOLUTION,
            CAUSAL_EDGE,
            CONSTITUTIONAL_WITNESS,
        ];
        let mut i = 0;
        while i < ids.len() {
            if !schema_ranges::is_core(ids[i]) {
                return false;
            }
            i += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_core_schemas_unique() {
        assert!(core_schemas::verify_uniqueness());
    }

    #[test]
    fn test_all_core_schemas_in_core_range() {
        assert!(core_schemas::verify_range());
    }

    #[test]
    fn test_ranges_disjoint() {
        // Core and runtime ranges should not overlap
        const _: () = {
            assert!(schema_ranges::CORE_END < schema_ranges::RUNTIME_START);
        };
        const _: () = {
            assert!(schema_ranges::RUNTIME_END < schema_ranges::EXPERIMENTAL_START);
        };
    }

    #[test]
    fn test_range_classification() {
        assert!(schema_ranges::is_core(0x0004));
        assert!(!schema_ranges::is_core(0x0100));
        assert!(schema_ranges::is_runtime(0x0100));
        assert!(!schema_ranges::is_runtime(0x0004));
        assert!(schema_ranges::is_experimental(0xF000));
        assert!(schema_ranges::is_reserved(0x0004));
        assert!(schema_ranges::is_reserved(0x0100));
        assert!(schema_ranges::is_reserved(0xF000));
        assert!(!schema_ranges::is_reserved(0x1000));
    }
}
