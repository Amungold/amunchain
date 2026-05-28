from enum import Enum
from dataclasses import dataclass
from typing import List, Optional

class ConstitutionalState(Enum):
    ORTHODOX = "orthodox"
    DRIFTING = "drifting"
    SCHISMATIC = "schismatic"
    SHADOW = "shadow"
    EXILED = "exiled"
    DEAD = "dead"

class FractureType(Enum):
    SHADOW_AUTHORITY = "shadow_authority"
    ORIGIN_SEVERANCE = "origin_severance"
    PARASITIC_LINEAGE = "parasitic_lineage"
    DOCTRINAL_COLLAPSE = "doctrinal_collapse"
    COLLECTIVE_DRIFT = "collective_drift"
    EMERGENT_AUTHORITY = "emergent_authority"

@dataclass
class ConstitutionalMetrics:
    constraint_influence: float
    cumulative_entropy: float
    authority_retention: float
    consensus_weight: float
    semantic_temperature: float
    lineage_depth: int
    origin_distance: float

@dataclass
class ConstitutionalTransition:
    from_state: ConstitutionalState
    to_state: ConstitutionalState
    trigger: FractureType
    confidence: float
    metrics_snapshot: ConstitutionalMetrics
