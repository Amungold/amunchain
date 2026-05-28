from dataclasses import dataclass, field
from typing import List, Optional
from ..core.enums import DriftLevel

@dataclass
class SemanticDelta:
    parent_id: str
    child_id: str
    added_invariants: List[str] = field(default_factory=list)
    removed_invariants: List[str] = field(default_factory=list)
    dependency_shift: List[str] = field(default_factory=list)
    contextual_changes: List[str] = field(default_factory=list)
    drift_level: DriftLevel = DriftLevel.NONE
    constraint_influence_change: float = 0.0
    semantic_weight_change: float = 0.0
    
    def to_dict(self):
        return {
            "parent_id": self.parent_id,
            "child_id": self.child_id,
            "added_invariants": self.added_invariants,
            "removed_invariants": self.removed_invariants,
            "dependency_shift": self.dependency_shift,
            "contextual_changes": self.contextual_changes,
            "drift_level": self.drift_level.value,
            "constraint_influence_change": self.constraint_influence_change,
            "semantic_weight_change": self.semantic_weight_change
        }
    
    def has_drift(self) -> bool:
        return self.drift_level != DriftLevel.NONE
    
    def is_severe(self) -> bool:
        return self.drift_level in [DriftLevel.HIGH, DriftLevel.CRITICAL]
    
    def calculate_drift_level(self) -> DriftLevel:
        total_changes = len(self.added_invariants) + len(self.removed_invariants) + len(self.dependency_shift)
        
        if total_changes == 0 and abs(self.constraint_influence_change) < 0.05:
            return DriftLevel.NONE
        elif total_changes <= 2 and abs(self.constraint_influence_change) < 0.15:
            return DriftLevel.LOW
        elif total_changes <= 4 and abs(self.constraint_influence_change) < 0.3:
            return DriftLevel.MEDIUM
        elif total_changes <= 6 or abs(self.constraint_influence_change) < 0.5:
            return DriftLevel.HIGH
        else:
            return DriftLevel.CRITICAL
