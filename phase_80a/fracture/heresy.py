from dataclasses import dataclass, field
from typing import List, Optional, Dict
from datetime import datetime
from enum import Enum

class HeresyLevel(Enum):
    SUSPECTED = "suspected"
    CONFIRMED = "confirmed"
    SEVERE = "severe"
    CATASTROPHIC = "catastrophic"

class HeresyType(Enum):
    AUTHORITY_USURPATION = "authority_usurpation"
    ORIGIN_RENUNCIATION = "origin_renunciation"
    INVARIANT_CORRUPTION = "invariant_corruption"
    CONTEXTUAL_POISONING = "contextual_poisoning"
    COLLECTIVE_SCHISM = "collective_schism"

@dataclass
class HeresyVerdict:
    node_id: str
    heresy_type: HeresyType
    level: HeresyLevel
    detected_at: str
    confidence: float
    evidence: List[str]
    entropy_threshold: float
    authority_threshold: float

class HeresyThresholdEngine:
    def __init__(self):
        self.verdicts: Dict[str, HeresyVerdict] = {}
        self.heresy_history: List[HeresyVerdict] = []
    
    def evaluate(self, node_id: str, entropy: float, authority_retention: float,
                 consensus_weight: float, origin_present: bool) -> Optional[HeresyVerdict]:
        evidence = []
        heresy_type = None
        level = HeresyLevel.SUSPECTED
        
        if entropy > 0.85:
            evidence.append(f"Critical entropy: {entropy:.3f}")
            if entropy > 0.95:
                level = HeresyLevel.CATASTROPHIC
            elif entropy > 0.90:
                level = HeresyLevel.SEVERE
        
        if authority_retention < 0.15:
            evidence.append(f"Authority collapse: {authority_retention:.3f}")
            if authority_retention < 0.05:
                level = HeresyLevel.CATASTROPHIC
        
        if consensus_weight < 0.1:
            evidence.append(f"Consensus rejection: {consensus_weight:.3f}")
        
        if not origin_present and authority_retention < 0.3:
            heresy_type = HeresyType.ORIGIN_RENUNCIATION
            evidence.append("Origin absent from effective authority")
        
        if entropy > 0.7 and authority_retention < 0.4:
            if heresy_type is None:
                heresy_type = HeresyType.AUTHORITY_USURPATION
            evidence.append("Entropy-inversion pattern detected")
        
        if not evidence:
            return None
        
        if level == HeresyLevel.SUSPECTED and len(evidence) >= 2:
            level = HeresyLevel.CONFIRMED
        
        if heresy_type is None:
            heresy_type = HeresyType.AUTHORITY_USURPATION
        
        verdict = HeresyVerdict(
            node_id=node_id,
            heresy_type=heresy_type,
            level=level,
            detected_at=datetime.utcnow().isoformat() + "Z",
            confidence=min(0.95, (entropy + (1.0 - authority_retention)) / 2),
            evidence=evidence,
            entropy_threshold=entropy,
            authority_threshold=authority_retention
        )
        
        self.verdicts[node_id] = verdict
        self.heresy_history.append(verdict)
        
        return verdict
    
    def is_heretic(self, node_id: str) -> bool:
        verdict = self.verdicts.get(node_id)
        if not verdict:
            return False
        return verdict.level in [HeresyLevel.CONFIRMED, HeresyLevel.SEVERE, HeresyLevel.CATASTROPHIC]
    
    def get_heresy_level(self, node_id: str) -> Optional[HeresyLevel]:
        verdict = self.verdicts.get(node_id)
        return verdict.level if verdict else None
    
    def get_all_heretics(self) -> List[str]:
        return [nid for nid, v in self.verdicts.items() if self.is_heretic(nid)]
    
    def get_heresy_stats(self) -> dict:
        return {
            "total_verdicts": len(self.verdicts),
            "confirmed_heretics": len(self.get_all_heretics()),
            "by_level": {
                level.value: len([v for v in self.verdicts.values() if v.level == level])
                for level in HeresyLevel
            },
            "by_type": {
                htype.value: len([v for v in self.verdicts.values() if v.heresy_type == htype])
                for htype in HeresyType
            }
        }
