import math
from dataclasses import dataclass, field
from typing import List, Optional
from datetime import datetime
from ..core.enums import AlivenessState, DerivationType, DependencyType
from ..core.hash_utils import compute_node_hash, compute_parent_hash, compute_semantic_commitment
from .semantic_delta import SemanticDelta
from .authority_proof import AuthorityTransitionProof

DECAY_FACTORS = {
    DerivationType.TRANSLATION: 0.98,
    DerivationType.EXTENSION: 0.92,
    DerivationType.REFRAMING: 0.80,
    DerivationType.DRIFT: 0.60,
    DerivationType.MUTATION: 0.35,
    DerivationType.SEVERANCE: 0.15,
}

ENTROPY_BASE_RATE = 0.10
ENTROPY_DRIFT_BONUS = 0.06
ENTROPY_DISSIPATION_RATE = 0.03
REINFORCEMENT_BONUS = 0.08
RECOVERY_THRESHOLD = 0.25

@dataclass
class InterpretiveNode:
    node_id: str
    semantic_origin_hash: str
    parent_id: Optional[str]
    parent_hash: Optional[str]
    node_hash: str
    semantic_commitment: str
    source_text: str
    derivation_type: DerivationType
    contextual_modifiers: List[str]
    semantic_invariants: List[str]
    constraint_dependency: List[str]
    constraint_dependency_types: List[DependencyType]
    aliveness: AlivenessState
    effective_constraint_authority: List[str]
    lineage_status: str = "complete"
    authenticity_status: str = "authentic"
    constraint_influence: float = 1.0
    semantic_weight: float = 1.0
    participation_activity: float = 1.0
    metabolic_rate: float = 1.0
    cumulative_entropy: float = 0.0
    authority_retention: float = 1.0
    reinforcement_count: int = 0
    cooling_factor: float = 1.0
    constitutional_notes: List[str] = field(default_factory=list)
    created_at: Optional[str] = None
    children: List[str] = field(default_factory=list)
    semantic_delta: Optional[SemanticDelta] = None
    authority_proof: Optional[AuthorityTransitionProof] = None

    @classmethod
    def create(cls, node_id: str, semantic_origin_hash: str, parent_id: Optional[str],
               source_text: str, derivation_type: DerivationType,
               contextual_modifiers: List[str], semantic_invariants: List[str],
               constraint_dependency: List[str], constraint_dependency_types: List[DependencyType],
               aliveness: AlivenessState, parent_node: Optional['InterpretiveNode'] = None,
               effective_constraint_authority: Optional[List[str]] = None,
               authority_proof: Optional[AuthorityTransitionProof] = None,
               override_influence: Optional[float] = None,
               reinforcement_boost: bool = False) -> 'InterpretiveNode':
        
        if effective_constraint_authority is not None:
            effective_authority = effective_constraint_authority.copy()
        else:
            effective_authority = constraint_dependency.copy()
        
        if node_id == "I4":
            print(f"[DEBUG CREATE] I4 effective_authority = {effective_authority}")
        
        parent_hash = None
        decay_factor = DECAY_FACTORS.get(derivation_type, 0.70)
        
        if parent_id and parent_node:
            parent_hash = compute_parent_hash(parent_node.node_id, parent_node.node_hash)
            
            if override_influence is None:
                base_influence = parent_node.constraint_influence * decay_factor
                
                if reinforcement_boost and parent_node.reinforcement_count > 0:
                    reinforcement = min(0.15, parent_node.reinforcement_count * 0.03)
                    constraint_influence = min(0.98, base_influence + reinforcement)
                else:
                    constraint_influence = base_influence
                
                authority_intersection = set(effective_authority) & set(parent_node.effective_constraint_authority)
                authority_retention = len(authority_intersection) / max(1, len(parent_node.effective_constraint_authority))
                
                invariant_retention = len(set(semantic_invariants) & set(parent_node.semantic_invariants)) / max(1, len(parent_node.semantic_invariants))
                
                base_weight = parent_node.semantic_weight * decay_factor * authority_retention * invariant_retention
                
                if reinforcement_boost:
                    semantic_weight = min(0.95, base_weight + 0.08)
                else:
                    semantic_weight = base_weight
                
                local_drift = (1.0 - decay_factor) * (1.0 - parent_node.constraint_influence)
                drift_bonus = ENTROPY_DRIFT_BONUS if derivation_type == DerivationType.DRIFT else 0.0
                
                raw_entropy_generation = ENTROPY_BASE_RATE + local_drift + drift_bonus
                
                dissipation = parent_node.cumulative_entropy * ENTROPY_DISSIPATION_RATE
                
                if reinforcement_boost:
                    dissipation += parent_node.cumulative_entropy * 0.05
                
                raw_entropy = parent_node.cumulative_entropy + raw_entropy_generation - dissipation
                raw_entropy = max(0.0, raw_entropy)
                
                cumulative_entropy = 1.0 - math.exp(-raw_entropy)
                cumulative_entropy = min(0.95, max(0.0, cumulative_entropy))
                
                participation_activity = parent_node.participation_activity * (0.94 if derivation_type == DerivationType.DRIFT else 0.98)
                
                base_metabolic = parent_node.metabolic_rate * (0.90 if derivation_type == DerivationType.DRIFT else 0.97)
                if reinforcement_boost:
                    metabolic_rate = min(0.95, base_metabolic + 0.05)
                else:
                    metabolic_rate = base_metabolic
                
                cooling_factor = 1.0 - (cumulative_entropy * 0.5)
                cooling_factor = max(0.3, min(1.0, cooling_factor))
                
            else:
                constraint_influence = override_influence
                semantic_weight = override_influence
                cumulative_entropy = 1.0 - override_influence
                authority_retention = override_influence
                participation_activity = override_influence
                metabolic_rate = override_influence
                cooling_factor = override_influence
        else:
            constraint_influence = 1.0
            semantic_weight = 1.0
            cumulative_entropy = 0.0
            authority_retention = 1.0
            participation_activity = 1.0
            metabolic_rate = 1.0
            cooling_factor = 1.0
        
        temp_node = cls.__new__(cls)
        temp_node.node_id = node_id
        temp_node.parent_id = parent_id
        temp_node.semantic_origin_hash = semantic_origin_hash
        temp_node.semantic_invariants = semantic_invariants
        temp_node.constraint_dependency = constraint_dependency
        temp_node.effective_constraint_authority = effective_authority
        temp_node.derivation_type = derivation_type
        temp_node.source_text = source_text
        temp_node.contextual_modifiers = contextual_modifiers
        
        node_hash = compute_node_hash(temp_node)
        semantic_commitment = compute_semantic_commitment(temp_node)
        
        created_at = datetime.utcnow().isoformat() + "Z"
        
        if constraint_influence < 0.12:
            aliveness_state = AlivenessState.DEAD
        elif constraint_influence < 0.25:
            aliveness_state = AlivenessState.CRITICAL
        elif constraint_influence < 0.45:
            aliveness_state = AlivenessState.INERT
        elif constraint_influence < 0.65:
            aliveness_state = AlivenessState.WEAKENED
        elif constraint_influence < 0.85:
            aliveness_state = AlivenessState.STABLE
        else:
            aliveness_state = AlivenessState.LIVING
        
        reinforcement_count = parent_node.reinforcement_count + 1 if parent_node and reinforcement_boost else (parent_node.reinforcement_count if parent_node else 0)
        
        if node_id == "I4":
            print(f"[DEBUG CREATE FINAL] I4 effective_constraint_authority = {effective_authority}")
        
        return cls(
            node_id=node_id,
            semantic_origin_hash=semantic_origin_hash,
            parent_id=parent_id,
            parent_hash=parent_hash,
            node_hash=node_hash,
            semantic_commitment=semantic_commitment,
            source_text=source_text,
            derivation_type=derivation_type,
            contextual_modifiers=contextual_modifiers,
            semantic_invariants=semantic_invariants,
            constraint_dependency=constraint_dependency,
            constraint_dependency_types=constraint_dependency_types,
            aliveness=aliveness_state,
            effective_constraint_authority=effective_authority,
            constraint_influence=constraint_influence,
            semantic_weight=semantic_weight,
            participation_activity=participation_activity,
            metabolic_rate=metabolic_rate,
            cumulative_entropy=cumulative_entropy,
            authority_retention=authority_retention,
            reinforcement_count=reinforcement_count,
            cooling_factor=cooling_factor,
            created_at=created_at,
            authority_proof=authority_proof
        )
    
    def reinforce(self) -> 'InterpretiveNode':
        new_node = self.__class__(
            node_id=self.node_id,
            semantic_origin_hash=self.semantic_origin_hash,
            parent_id=self.parent_id,
            parent_hash=self.parent_hash,
            node_hash=self.node_hash,
            semantic_commitment=self.semantic_commitment,
            source_text=self.source_text,
            derivation_type=self.derivation_type,
            contextual_modifiers=self.contextual_modifiers,
            semantic_invariants=self.semantic_invariants,
            constraint_dependency=self.constraint_dependency,
            constraint_dependency_types=self.constraint_dependency_types,
            aliveness=self.aliveness,
            effective_constraint_authority=self.effective_constraint_authority,
            lineage_status=self.lineage_status,
            authenticity_status=self.authenticity_status,
            constraint_influence=min(0.98, self.constraint_influence + 0.05),
            semantic_weight=min(0.95, self.semantic_weight + 0.06),
            participation_activity=min(0.95, self.participation_activity + 0.05),
            metabolic_rate=min(0.95, self.metabolic_rate + 0.04),
            cumulative_entropy=max(0.0, self.cumulative_entropy - 0.08),
            authority_retention=min(1.0, self.authority_retention + 0.07),
            reinforcement_count=self.reinforcement_count + 1,
            cooling_factor=min(1.0, self.cooling_factor + 0.05),
            created_at=self.created_at,
            children=self.children,
            semantic_delta=self.semantic_delta,
            authority_proof=self.authority_proof
        )
        return new_node
    
    def to_dict(self) -> dict:
        return {
            "node_id": self.node_id,
            "semantic_origin_hash": self.semantic_origin_hash,
            "parent_id": self.parent_id,
            "parent_hash": self.parent_hash,
            "node_hash": self.node_hash,
            "semantic_commitment": self.semantic_commitment,
            "source_text": self.source_text,
            "derivation_type": self.derivation_type.value,
            "aliveness": self.aliveness.value,
            "constraint_dependency": self.constraint_dependency,
            "effective_constraint_authority": self.effective_constraint_authority,
            "constraint_influence": self.constraint_influence,
            "cumulative_entropy": self.cumulative_entropy,
            "authority_retention": self.authority_retention,
            "reinforcement_count": self.reinforcement_count,
            "cooling_factor": self.cooling_factor,
            "created_at": self.created_at
        }
    
    def compute_aliveness_from_metrics(self) -> AlivenessState:
        viability = (
            self.constraint_influence * 0.30 +
            (1.0 - self.cumulative_entropy) * 0.25 +
            self.authority_retention * 0.20 +
            self.participation_activity * 0.10 +
            self.metabolic_rate * 0.10 +
            self.cooling_factor * 0.05
        )
        if viability >= 0.70:
            return AlivenessState.LIVING
        elif viability >= 0.55:
            return AlivenessState.STABLE
        elif viability >= 0.40:
            return AlivenessState.WEAKENED
        elif viability >= 0.25:
            return AlivenessState.INERT
        elif viability >= 0.12:
            return AlivenessState.CRITICAL
        else:
            return AlivenessState.DEAD
    
    def is_alive(self) -> bool:
        return self.aliveness not in [AlivenessState.DEAD]
    
    def is_necessary(self) -> bool:
        return (self.constraint_influence > 0.20 and 
                self.cumulative_entropy < 0.70 and 
                self.authority_retention > 0.25 and
                self.cooling_factor > 0.40)
    
    def can_recover(self) -> bool:
        return (self.constraint_influence > 0.10 and 
                self.cumulative_entropy < 0.85 and 
                self.reinforcement_count < 20)
    
    def freeze(self) -> dict:
        return {
            "node_id": self.node_id,
            "parent_id": self.parent_id,
            "parent_hash": self.parent_hash,
            "node_hash": self.node_hash,
            "semantic_commitment": self.semantic_commitment,
            "semantic_origin_hash": self.semantic_origin_hash,
            "semantic_invariants": sorted(self.semantic_invariants),
            "constraint_dependency": sorted(self.constraint_dependency),
            "effective_constraint_authority": sorted(self.effective_constraint_authority),
            "derivation_type": self.derivation_type.value,
            "source_text": self.source_text,
            "contextual_modifiers": self.contextual_modifiers,
            "constraint_influence": self.constraint_influence,
            "cumulative_entropy": self.cumulative_entropy,
            "authority_retention": self.authority_retention,
            "reinforcement_count": self.reinforcement_count,
            "cooling_factor": self.cooling_factor,
            "created_at": self.created_at
        }
    
    @classmethod
    def from_frozen(cls, data: dict) -> 'InterpretiveNode':
        from ..core.enums import DerivationType, AlivenessState, DependencyType
        
        return cls(
            node_id=data["node_id"],
            semantic_origin_hash=data["semantic_origin_hash"],
            parent_id=data.get("parent_id"),
            parent_hash=data.get("parent_hash"),
            node_hash=data["node_hash"],
            semantic_commitment=data["semantic_commitment"],
            source_text=data["source_text"],
            derivation_type=DerivationType(data["derivation_type"]),
            contextual_modifiers=data.get("contextual_modifiers", []),
            semantic_invariants=data["semantic_invariants"],
            constraint_dependency=data["constraint_dependency"],
            constraint_dependency_types=[DependencyType.ORIGIN],
            aliveness=AlivenessState.STABLE,
            effective_constraint_authority=data["effective_constraint_authority"],
            constraint_influence=data.get("constraint_influence", 1.0),
            cumulative_entropy=data.get("cumulative_entropy", 0.0),
            authority_retention=data.get("authority_retention", 1.0),
            reinforcement_count=data.get("reinforcement_count", 0),
            cooling_factor=data.get("cooling_factor", 1.0),
            created_at=data.get("created_at")
        )
