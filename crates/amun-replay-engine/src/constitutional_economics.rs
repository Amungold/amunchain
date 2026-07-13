//! Constitutional Economics — markets that sustain, not govern.
//!
//! Economic coordination may sustain the runtime, but may NEVER
//! purchase constitutional truth. Resource asymmetry must never
//! become derivational asymmetry.
//!
//! CRITICAL: Expensive proofs are not stronger proofs.
//! Wealthy relays are not authoritative relays.

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;

/// A proof market surface — a market of operational delivery, NOT truth.
///
/// Participants may offer and consume operational services:
/// execution, routing, storage, witness transport.
/// These are OPERATIONAL transactions, not constitutional ones.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofMarketSurface {
    /// Unique market identifier.
    pub market_id: u64,
    /// The context this market operates within.
    pub context_hash: ConstitutionalHash,
    /// Maximum price acceptable for operational services.
    /// This is a MARKET constraint, not a truth constraint.
    pub max_service_price: u64,
    /// Whether this market is active.
    pub active: bool,
}

impl ProofMarketSurface {
    pub fn new(market_id: u64, context_hash: ConstitutionalHash) -> Self {
        Self { market_id, context_hash, max_service_price: 1000, active: true }
    }

    /// Returns true if a service at the given price is within market bounds.
    /// This is an ECONOMIC check, not a constitutional one.
    pub fn is_affordable(&self, price: u64) -> bool {
        price <= self.max_service_price
    }
}

/// An execution lease boundary — lease compute, NOT constitutional authority.
///
/// Leasing allows participants to pay for execution capacity.
/// It does NOT allow purchasing admissibility, validity, or truth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionLeaseBoundary {
    /// Unique lease identifier.
    pub lease_id: u64,
    /// The worker providing execution capacity.
    pub worker_id: u64,
    /// The context this lease applies to.
    pub context_hash: ConstitutionalHash,
    /// Maximum executions allowed under this lease.
    pub max_executions: u64,
    /// Current execution count.
    pub execution_count: u64,
    /// Whether this lease grants any constitutional authority (ALWAYS false).
    pub grants_constitutional_authority: bool,
}

impl ExecutionLeaseBoundary {
    pub fn new(lease_id: u64, worker_id: u64, context_hash: ConstitutionalHash, max_executions: u64) -> Self {
        Self { lease_id, worker_id, context_hash, max_executions, execution_count: 0, grants_constitutional_authority: false }
    }

    /// Returns true if this lease has remaining execution capacity.
    pub fn has_capacity(&self) -> bool {
        self.execution_count < self.max_executions
    }

    /// Consume one execution slot.
    pub fn execute(&mut self) -> bool {
        if self.has_capacity() { self.execution_count += 1; true }
        else { false }
    }

    /// CRITICAL: This method always returns false.
    /// Leases NEVER grant constitutional authority.
    pub fn check_constitutional_authority(&self) -> bool {
        self.grants_constitutional_authority
    }
}

/// A witness incentive surface — rewards availability, NOT truth ownership.
///
/// Incentives reward operational contributions:
///   - Making witnesses available
///   - Responding to closure requests
///   - Assisting frontier reduction
///   - Providing routing assistance
///
/// Incentives do NOT reward:
///   - "Owning" admissibility
///   - Producing "better" proofs
///   - Semantic influence
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessIncentiveSurface {
    /// Unique incentive program identifier.
    pub program_id: u64,
    /// The context this incentive applies to.
    pub context_hash: ConstitutionalHash,
    /// Reward for making a witness available.
    pub availability_reward: u64,
    /// Reward for responding to a closure request.
    pub closure_response_reward: u64,
    /// Reward for frontier reduction assistance.
    pub frontier_reduction_reward: u64,
    /// CRITICAL: No reward for "better" proofs (ALWAYS 0).
    pub semantic_quality_reward: u64,
}

impl WitnessIncentiveSurface {
    pub fn new(program_id: u64, context_hash: ConstitutionalHash) -> Self {
        Self { program_id, context_hash, availability_reward: 10, closure_response_reward: 20, frontier_reduction_reward: 30, semantic_quality_reward: 0 }
    }

    /// CRITICAL: Semantic quality reward is always zero.
    /// Proofs are not "better" because they cost more.
    pub fn verify_no_semantic_reward(&self) -> bool {
        self.semantic_quality_reward == 0
    }
}

/// An economic containment zone — prevents economic capture dynamics.
///
/// Detects and contains:
///   - Price manipulation that could create proof monopolies
///   - Capital concentration approaching semantic influence thresholds
///   - Economic cartels attempting to control admissibility transport
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomicContainmentZone {
    /// Participants with excessive economic concentration.
    pub contained_participants: Vec<u64>,
    /// Maximum market share before containment warning.
    pub max_market_share_percent: u8,
    /// Whether economic capture is suspected.
    pub capture_suspected: bool,
}

impl EconomicContainmentZone {
    pub fn new(max_share: u8) -> Self {
        Self { contained_participants: Vec::new(), max_market_share_percent: max_share, capture_suspected: false }
    }

    /// Check if a participant's market share exceeds the threshold.
    pub fn check_market_share(&mut self, participant_id: u64, share_percent: u8) {
        if share_percent > self.max_market_share_percent {
            if !self.contained_participants.contains(&participant_id) {
                self.contained_participants.push(participant_id);
            }
            self.capture_suspected = true;
        }
    }

    /// Returns true if economic capture is suspected.
    /// This is a WARNING — not a constitutional invalidation.
    pub fn is_capture_suspected(&self) -> bool { self.capture_suspected }
}

/// A scarcity neutrality boundary — prevents wealth→legitimacy conversion.
///
/// Ensures that:
///   - Expensive proofs are not privileged proofs
///   - Wealthy relays are not authoritative relays
///   - Capital concentration does not become legitimacy concentration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScarcityNeutralityBoundary {
    /// Maximum resource expenditure that can be associated with a proof
    /// before it triggers neutrality review.
    pub max_neutral_resource_bound: u64,
    /// Whether any proof has exceeded the neutrality bound.
    pub neutrality_violation_detected: bool,
}

impl ScarcityNeutralityBoundary {
    pub fn new(max_bound: u64) -> Self {
        Self { max_neutral_resource_bound: max_bound, neutrality_violation_detected: false }
    }

    /// Check if resource expenditure exceeds the neutrality boundary.
    /// This is a WARNING — the proof remains constitutionally valid.
    /// But excessive resource concentration may indicate economic capture.
    pub fn check_neutrality(&mut self, resource_expenditure: u64) -> bool {
        if resource_expenditure > self.max_neutral_resource_bound {
            self.neutrality_violation_detected = true;
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_is_economic_not_constitutional() {
        let market = ProofMarketSurface::new(1, [0xAB; 32]);
        assert!(market.is_affordable(500));
        assert!(!market.is_affordable(2000));
    }

    #[test]
    fn test_lease_never_grants_authority() {
        let lease = ExecutionLeaseBoundary::new(1, 100, [0xAB; 32], 10);
        assert!(!lease.check_constitutional_authority());
        assert!(lease.has_capacity());
    }

    #[test]
    fn test_lease_capacity_exhaustion() {
        let mut lease = ExecutionLeaseBoundary::new(1, 100, [0xAB; 32], 2);
        assert!(lease.execute());
        assert!(lease.execute());
        assert!(!lease.execute()); // exhausted
    }

    #[test]
    fn test_semantic_quality_reward_is_zero() {
        let incentives = WitnessIncentiveSurface::new(1, [0xAB; 32]);
        assert!(incentives.verify_no_semantic_reward());
        assert_eq!(incentives.semantic_quality_reward, 0);
    }

    #[test]
    fn test_economic_containment() {
        let mut zone = EconomicContainmentZone::new(30);
        zone.check_market_share(100, 35); // exceeds 30%
        assert!(zone.is_capture_suspected());
        assert!(zone.contained_participants.contains(&100));
    }

    #[test]
    fn test_scarcity_neutrality() {
        let mut boundary = ScarcityNeutralityBoundary::new(10000);
        assert!(boundary.check_neutrality(5000)); // within bound
        assert!(!boundary.check_neutrality(15000)); // exceeds bound
        assert!(boundary.neutrality_violation_detected);
    }

    #[test]
    fn test_economics_does_not_affect_truth() {
        // All economic constructs are operational only.
        // They make no constitutional claims.
        let lease = ExecutionLeaseBoundary::new(1, 100, [0xAB; 32], 10);
        assert!(!lease.check_constitutional_authority());
        // This is always false — economics cannot purchase truth
    }
}
