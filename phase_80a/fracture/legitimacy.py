from dataclasses import dataclass
from typing import Dict, Optional, List
from enum import Enum
from ..math.ops import (
    to_fixed, from_fixed, to_fixed_int, from_fixed_int,
    F_add, F_sub, F_mul, F_div, F_abs, F_sqrt, F_exp,
    ZERO, ONE, HALF, QUARTER, THIRD, TWO_THIRDS,
    MAX_TOTAL_COUPLING,
    THRESHOLD_LEGITIMACY_LOW, THRESHOLD_LEGITIMACY_VERY_LOW,
    THRESHOLD_ENTROPY_HIGH, THRESHOLD_ENTROPY_CRITICAL
)
from .conservation import LegitimacyConservationEngine

class LegitimacySource(Enum):
    ORIGIN = "origin"
    CONSENSUS = "consensus"
    SHADOW = "shadow"
    FORK = "fork"
    COHERENCE = "coherence"
    SURVIVABILITY = "survivability"
    NONE = "none"

class PhaseTransition(Enum):
    STABLE = "stable"
    SCHISM = "schism"
    SHADOW_CRYSTALLIZATION = "shadow_crystallization"
    DOCTRINAL_COLLAPSE = "doctrinal_collapse"
    LEGITIMACY_INVERSION = "legitimacy_inversion"
    CONSTITUTIONAL_ATTRACTOR = "constitutional_attractor"

@dataclass
class LegitimacyVector:
    origin: int = ZERO
    consensus: int = ZERO
    shadow: int = ZERO
    fork: int = ZERO
    coherence: int = ZERO
    survivability: int = ZERO
    
    COUPLING_MATRIX = {
        ("origin", "consensus"): to_fixed(0.4),
        ("origin", "coherence"): to_fixed(0.3),
        ("consensus", "survivability"): to_fixed(0.5),
        ("shadow", "coherence"): to_fixed(0.7),
        ("shadow", "fork"): to_fixed(0.6),
        ("fork", "survivability"): to_fixed(0.4),
        ("entropy", "origin"): to_fixed(-0.8),
        ("entropy", "coherence"): to_fixed(-0.5),
        ("entropy", "survivability"): to_fixed(-0.6),
    }
    
    def magnitude(self) -> int:
        sum_sq = ZERO
        for v in [self.origin, self.consensus, self.shadow, self.fork, self.coherence, self.survivability]:
            sum_sq = F_add(sum_sq, F_mul(v, v))
        return F_sqrt(sum_sq)
    
    def weighted_magnitude(self, weights: Dict[str, int]) -> int:
        total = ZERO
        total = F_add(total, F_mul(self.origin, weights.get("origin", ONE)))
        total = F_add(total, F_mul(self.consensus, weights.get("consensus", to_fixed(0.8))))
        total = F_add(total, F_mul(self.shadow, weights.get("shadow", to_fixed(0.3))))
        total = F_add(total, F_mul(self.fork, weights.get("fork", to_fixed(0.4))))
        total = F_add(total, F_mul(self.coherence, weights.get("coherence", to_fixed(0.7))))
        total = F_add(total, F_mul(self.survivability, weights.get("survivability", to_fixed(0.5))))
        return F_div(total, to_fixed_int(6))

@dataclass
class LegitimacyVerdict:
    node_id: str
    legitimacy_score: int
    legitimacy_vector: LegitimacyVector
    primary_source: LegitimacySource
    verdict_at_tick: int
    is_exiled: bool
    is_heretic: bool
    divergence_potential: int
    phase_transition: PhaseTransition
    exile_reason: Optional[str] = None
    heresy_level: Optional[str] = None
    
    def legitimacy_score_float(self) -> float:
        return from_fixed(self.legitimacy_score)

class ConstitutionalLegitimacyKernel:
    """
    Legitimacy kernel with conservation - all legitimacy derived from ledger.
    No ex-nihilo legitimacy creation.
    """
    
    def __init__(self):
        self.conservation_engine = LegitimacyConservationEngine()
        self.verdicts: Dict[str, LegitimacyVerdict] = {}
        self.previous_vectors: Dict[str, LegitimacyVector] = {}
        self.current_tick = 0
        self.source_weights = {
            LegitimacySource.ORIGIN: ONE,
            LegitimacySource.CONSENSUS: to_fixed(0.8),
            LegitimacySource.COHERENCE: to_fixed(0.7),
            LegitimacySource.SURVIVABILITY: to_fixed(0.5),
            LegitimacySource.FORK: to_fixed(0.3),
            LegitimacySource.SHADOW: to_fixed(0.2),
        }
    
    def set_current_tick(self, tick: int):
        self.current_tick = tick
        self.conservation_engine.set_current_tick(tick)
    
    def mint_genesis_legitimacy(self, node_id: str, amount: int, proof: str = "genesis") -> bool:
        """Minting only allowed for genesis (constitutional ratification)."""
        if not self.verdicts:
            return self.conservation_engine.mint_legitimacy(node_id, amount, proof)
        return False
    
    def allocate_legitimacy(self, node_id: str, amount: int) -> bool:
        """Allocate from available legitimacy budget."""
        return self.conservation_engine.allocate_legitimacy(node_id, amount)
    
    def transfer_legitimacy(self, from_node: str, to_node: str, amount: int) -> bool:
        """Transfer legitimacy between nodes."""
        return self.conservation_engine.transfer_legitimacy(from_node, to_node, amount)
    
    def compute_divergence_potential(self, vector: LegitimacyVector, entropy: int) -> int:
        magnitude = vector.magnitude()
        entropy_term = F_mul(entropy, HALF)
        
        coupling_term = ZERO
        names = ["origin", "consensus", "shadow", "fork", "coherence", "survivability"]
        for i, name in enumerate(names):
            for j, other in enumerate(names):
                if i != j:
                    coupling = LegitimacyVector.COUPLING_MATRIX.get((name, other), ZERO)
                    if coupling != ZERO:
                        coupling_term = F_add(coupling_term, F_mul(coupling, 
                            F_mul(getattr(vector, name), getattr(vector, other))))
        
        divergence = F_add(entropy_term, F_mul(coupling_term, to_fixed(0.3)))
        divergence = F_sub(divergence, F_mul(magnitude, to_fixed(0.2)))
        return F_clamp(divergence, ZERO, ONE)
    
    def compute_legitimacy(self, node_id: str, base_influence: float, 
                           entropy: float, authority_retention: float,
                           consensus_weight: float, origin_present: bool,
                           coherence_score: float = 0.5,
                           survivability_score: float = 0.5) -> LegitimacyVerdict:
        
        base_influence_fixed = to_fixed(base_influence)
        entropy_fixed = to_fixed(entropy)
        authority_retention_fixed = to_fixed(authority_retention)
        consensus_weight_fixed = to_fixed(consensus_weight)
        coherence_fixed = to_fixed(coherence_score)
        survivability_fixed = to_fixed(survivability_score)
        
        # Get conserved legitimacy balance
        conserved_balance = self.conservation_engine.get_balance(node_id)
        
        vector = LegitimacyVector()
        
        if origin_present and authority_retention_fixed > to_fixed(0.3):
            vector.origin = F_mul(base_influence_fixed, F_add(to_fixed(0.6), F_mul(to_fixed(0.4), authority_retention_fixed)))
        else:
            vector.origin = F_mul(base_influence_fixed, F_mul(to_fixed(0.3), F_sub(ONE, entropy_fixed)))
            vector.origin = F_max(vector.origin, ZERO)
        
        vector.consensus = F_mul(consensus_weight_fixed, to_fixed(0.9))
        vector.coherence = F_mul(coherence_fixed, F_sub(ONE, F_mul(entropy_fixed, HALF)))
        vector.survivability = F_mul(survivability_fixed, F_sub(ONE, F_mul(entropy_fixed, to_fixed(0.3))))
        vector.shadow = to_fixed(0.3)
        vector.fork = ZERO
        
        # Apply entropy penalty
        entropy_penalty = F_sub(ONE, F_mul(entropy_fixed, to_fixed(0.6)))
        vector.origin = F_mul(vector.origin, entropy_penalty)
        
        vector = vector.apply_coupling(entropy_fixed)
        
        for comp in ["origin", "consensus", "shadow", "fork", "coherence", "survivability"]:
            current = getattr(vector, comp)
            setattr(vector, comp, F_clamp(current, ZERO, ONE))
        
        divergence_potential = self.compute_divergence_potential(vector, entropy_fixed)
        
        # Weighted legitimacy from vector
        weighted_total = ZERO
        weighted_total = F_add(weighted_total, F_mul(vector.origin, ONE))
        weighted_total = F_add(weighted_total, F_mul(vector.consensus, to_fixed(0.8)))
        weighted_total = F_add(weighted_total, F_mul(vector.coherence, to_fixed(0.7)))
        weighted_total = F_add(weighted_total, F_mul(vector.survivability, to_fixed(0.5)))
        weighted_total = F_add(weighted_total, F_mul(vector.fork, to_fixed(0.3)))
        weighted_total = F_add(weighted_total, F_mul(vector.shadow, to_fixed(0.2)))
        vector_legitimacy = F_div(weighted_total, to_fixed_int(6))
        
        # Final legitimacy = min(conserved_balance, vector_legitimacy)
        legitimacy_score = F_min(conserved_balance, vector_legitimacy)
        
        is_exiled = False
        exile_reason = None
        
        if legitimacy_score < THRESHOLD_LEGITIMACY_LOW:
            is_exiled = True
            exile_reason = "legitimacy_collapse"
        elif legitimacy_score < THRESHOLD_LEGITIMACY_VERY_LOW and entropy_fixed > THRESHOLD_ENTROPY_HIGH:
            is_exiled = True
            exile_reason = "entropy_threshold"
        
        verdict = LegitimacyVerdict(
            node_id=node_id,
            legitimacy_score=legitimacy_score,
            legitimacy_vector=vector,
            primary_source=LegitimacySource.ORIGIN if vector.origin > vector.consensus else LegitimacySource.CONSENSUS,
            verdict_at_tick=self.current_tick,
            is_exiled=is_exiled,
            is_heretic=False,
            divergence_potential=divergence_potential,
            phase_transition=PhaseTransition.STABLE,
            exile_reason=exile_reason
        )
        
        self.verdicts[node_id] = verdict
        return verdict
    
    def get_constitutional_state(self, node_id: str) -> str:
        verdict = self.verdicts.get(node_id)
        if not verdict:
            return "unknown"
        
        if verdict.is_exiled:
            return "exiled"
        elif verdict.legitimacy_score > to_fixed(0.7):
            return "orthodox"
        elif verdict.legitimacy_score > to_fixed(0.45):
            return "drifting"
        elif verdict.legitimacy_score > to_fixed(0.25):
            return "schismatic"
        else:
            return "collapsed"
    
    def get_conservation_report(self) -> dict:
        return self.conservation_engine.get_conservation_report()
