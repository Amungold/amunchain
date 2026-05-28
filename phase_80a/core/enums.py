from enum import Enum

class AlivenessState(Enum):
    LIVING = "living"
    STABLE = "stable"
    WEAKENED = "weakened"
    INERT = "inert"
    CRITICAL = "critical"
    DEAD = "dead"

class DerivationType(Enum):
    TRANSLATION = "translation"
    EXTENSION = "extension"
    REFRAMING = "reframing"
    DRIFT = "drift"
    MUTATION = "mutation"
    SEVERANCE = "severance"

class DependencyType(Enum):
    ORIGIN = "origin"
    DERIVED = "derived"
    CONTEXT = "context"
    HYBRID = "hybrid"

class DriftLevel(Enum):
    NONE = "none"
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"

class ConsensusRole(Enum):
    VALIDATOR = "validator"
    OBSERVER = "observer"
    PROPOSER = "proposer"
    COMMITTER = "committer"
