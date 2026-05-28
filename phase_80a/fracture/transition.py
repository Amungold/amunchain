from typing import Tuple, Optional
from .types import ConstitutionalState, FractureType, ConstitutionalMetrics, ConstitutionalTransition

class ConstitutionalTransitionEngine:
    def __init__(self):
        self.transition_history = []
    
    def compute_next_state(self, metrics: ConstitutionalMetrics, 
                           current_state: ConstitutionalState) -> Tuple[ConstitutionalState, Optional[FractureType]]:
        
        if current_state == ConstitutionalState.ORTHODOX:
            if metrics.authority_retention < 0.7:
                return ConstitutionalState.DRIFTING, FractureType.COLLECTIVE_DRIFT
            elif metrics.cumulative_entropy > 0.5:
                return ConstitutionalState.DRIFTING, FractureType.COLLECTIVE_DRIFT
            return ConstitutionalState.ORTHODOX, None
        
        elif current_state == ConstitutionalState.DRIFTING:
            if metrics.consensus_weight < 0.3:
                return ConstitutionalState.SCHISMATIC, FractureType.ORIGIN_SEVERANCE
            elif metrics.authority_retention < 0.3:
                return ConstitutionalState.SCHISMATIC, FractureType.SHADOW_AUTHORITY
            elif metrics.cumulative_entropy > 0.7:
                return ConstitutionalState.SCHISMATIC, FractureType.DOCTRINAL_COLLAPSE
            elif metrics.authority_retention > 0.8 and metrics.cumulative_entropy < 0.3:
                return ConstitutionalState.ORTHODOX, None
            return ConstitutionalState.DRIFTING, None
        
        elif current_state == ConstitutionalState.SCHISMATIC:
            if metrics.consensus_weight < 0.15:
                return ConstitutionalState.EXILED, FractureType.ORIGIN_SEVERANCE
            elif metrics.authority_retention > 0.5:
                return ConstitutionalState.SHADOW, FractureType.EMERGENT_AUTHORITY
            elif metrics.cumulative_entropy > 0.85:
                return ConstitutionalState.DEAD, FractureType.DOCTRINAL_COLLAPSE
            return ConstitutionalState.SCHISMATIC, None
        
        elif current_state == ConstitutionalState.SHADOW:
            if metrics.consensus_weight > 0.5 and metrics.authority_retention > 0.6:
                return ConstitutionalState.SCHISMATIC, None
            elif metrics.consensus_weight < 0.1:
                return ConstitutionalState.EXILED, FractureType.PARASITIC_LINEAGE
            return ConstitutionalState.SHADOW, None
        
        elif current_state == ConstitutionalState.EXILED:
            if metrics.cumulative_entropy > 0.9 or metrics.constraint_influence < 0.1:
                return ConstitutionalState.DEAD, None
            return ConstitutionalState.EXILED, None
        
        return ConstitutionalState.DEAD, None
    
    def can_recover(self, state: ConstitutionalState, metrics: ConstitutionalMetrics) -> bool:
        if state == ConstitutionalState.DRIFTING:
            return metrics.authority_retention > 0.7 and metrics.cumulative_entropy < 0.3
        elif state == ConstitutionalState.SCHISMATIC:
            return False
        elif state == ConstitutionalState.SHADOW:
            return metrics.consensus_weight > 0.5 and metrics.authority_retention > 0.6
        elif state == ConstitutionalState.EXILED:
            return False
        return True
    
    def get_state_description(self, state: ConstitutionalState) -> str:
        descriptions = {
            ConstitutionalState.ORTHODOX: "Fully legitimate, aligned with constitutional origin",
            ConstitutionalState.DRIFTING: "Gradual semantic decay, under observation",
            ConstitutionalState.SCHISMATIC: "Authority detached from origin, structural divergence",
            ConstitutionalState.SHADOW: "Alternative authority system emerging",
            ConstitutionalState.EXILED: "Outside constitutional protection, no legitimacy",
            ConstitutionalState.DEAD: "Constitutionally extinct"
        }
        return descriptions.get(state, "Unknown state")
