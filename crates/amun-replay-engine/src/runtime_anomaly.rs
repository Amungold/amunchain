//! RuntimeAnomalySurface — operational pathology description.
//!
//! Describes anomalous runtime behavior WITHOUT semantic interpretation.
//! An anomaly is an OPERATIONAL observation, not a constitutional judgment.
//!
//! CRITICAL: Anomaly != Invalidity. Suspicion != Condemnation.

use amun_constitutional::kernel_types::ConstitutionalHash;

/// Types of runtime anomalies that can be observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnomalyType {
    /// Worker produced an unexpected artifact hash.
    UnexpectedArtifact,
    /// Worker propagated a witness with suspicious structure.
    SuspiciousWitness,
    /// Worker flooded the network with closure requests.
    ClosureFlood,
    /// Worker sent conflicting derivational frontiers.
    FrontierConflict,
    /// Worker attempted to spoof an admissibility fingerprint.
    AdmissibilitySpoof,
    /// Worker exceeded its capability boundary.
    CapabilityViolation,
    /// Worker propagated poisoned equivalence classes.
    EquivalencePoisoning,
    /// Worker is unresponsive (potential silent failure).
    WorkerSilence,
}

/// An observed runtime anomaly — operational, not constitutional.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeAnomaly {
    /// Unique anomaly identifier (operational).
    pub anomaly_id: u64,
    /// The worker exhibiting anomalous behavior.
    pub worker_id: u64,
    /// The type of anomaly observed.
    pub anomaly_type: AnomalyType,
    /// The context where the anomaly was observed.
    pub context_hash: ConstitutionalHash,
    /// Operational details about the anomaly (informational).
    pub details: Option<Vec<u8>>,
    /// How many times this anomaly type has been observed from this worker.
    pub occurrence_count: u64,
}

impl RuntimeAnomaly {
    pub fn new(
        anomaly_id: u64, worker_id: u64, anomaly_type: AnomalyType,
        context_hash: ConstitutionalHash,
    ) -> Self {
        Self { anomaly_id, worker_id, anomaly_type, context_hash, details: None, occurrence_count: 1 }
    }

    /// Returns true if this anomaly type is potentially hostile.
    pub fn is_hostile(&self) -> bool {
        matches!(
            self.anomaly_type,
            AnomalyType::AdmissibilitySpoof
                | AnomalyType::EquivalencePoisoning
                | AnomalyType::FrontierConflict
                | AnomalyType::CapabilityViolation
        )
    }

    /// Returns true if this anomaly type is potentially benign (network issue, etc.).
    pub fn is_benign(&self) -> bool {
        matches!(
            self.anomaly_type,
            AnomalyType::WorkerSilence | AnomalyType::ClosureFlood
        )
    }

    /// Increment occurrence count for repeated anomalies.
    pub fn increment(&mut self) { self.occurrence_count += 1; }
}

/// A surface collecting all observed anomalies for a worker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeAnomalySurface {
    pub worker_id: u64,
    pub anomalies: Vec<RuntimeAnomaly>,
    pub total_anomalies: u64,
    pub hostile_count: u64,
}

impl RuntimeAnomalySurface {
    pub fn new(worker_id: u64) -> Self {
        Self { worker_id, anomalies: Vec::new(), total_anomalies: 0, hostile_count: 0 }
    }

    pub fn record(&mut self, anomaly: RuntimeAnomaly) {
        if anomaly.is_hostile() { self.hostile_count += 1; }
        self.total_anomalies += 1;
        self.anomalies.push(anomaly);
    }

    /// Returns true if this worker should be quarantined based on anomaly pattern.
    pub fn should_quarantine(&self) -> bool {
        self.hostile_count >= 3 || self.total_anomalies >= 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anomaly_classification() {
        let hostile = RuntimeAnomaly::new(1, 100, AnomalyType::AdmissibilitySpoof, [0xAB; 32]);
        assert!(hostile.is_hostile());
        assert!(!hostile.is_benign());

        let benign = RuntimeAnomaly::new(2, 100, AnomalyType::WorkerSilence, [0xAB; 32]);
        assert!(!benign.is_hostile());
        assert!(benign.is_benign());
    }

    #[test]
    fn test_quarantine_threshold() {
        let mut surface = RuntimeAnomalySurface::new(100);
        assert!(!surface.should_quarantine());
        surface.record(RuntimeAnomaly::new(1, 100, AnomalyType::AdmissibilitySpoof, [0xAB; 32]));
        surface.record(RuntimeAnomaly::new(2, 100, AnomalyType::FrontierConflict, [0xAB; 32]));
        surface.record(RuntimeAnomaly::new(3, 100, AnomalyType::EquivalencePoisoning, [0xAB; 32]));
        assert!(surface.should_quarantine());
    }

    #[test]
    fn test_anomaly_is_not_invalidity() {
        // An anomaly is an observation, not a constitutional judgment.
        // The constitutional kernel judges artifacts, not behaviors.
        let anomaly = RuntimeAnomaly::new(1, 100, AnomalyType::SuspiciousWitness, [0xAB; 32]);
        // The anomaly exists — that's all. No validity claim is made.
        assert_eq!(anomaly.worker_id, 100);
    }
}
