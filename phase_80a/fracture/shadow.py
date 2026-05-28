from dataclasses import dataclass, field
from typing import List, Dict, Optional, Set
from datetime import datetime

@dataclass
class ShadowAuthorityRecord:
    node_id: str
    emerged_at: str
    parent_shadow: Optional[str]
    member_nodes: List[str]
    authority_strength: float
    origin_distance: int
    entropy_level: float

class ShadowAuthorityTracker:
    def __init__(self):
        self.shadow_authorities: Dict[str, ShadowAuthorityRecord] = {}
        self.shadow_membership: Dict[str, str] = {}
    
    def register_shadow(self, node_id: str, parent_shadow: Optional[str],
                        authority_strength: float, origin_distance: int,
                        entropy_level: float) -> str:
        record = ShadowAuthorityRecord(
            node_id=node_id,
            emerged_at=datetime.utcnow().isoformat() + "Z",
            parent_shadow=parent_shadow,
            member_nodes=[node_id],
            authority_strength=authority_strength,
            origin_distance=origin_distance,
            entropy_level=entropy_level
        )
        
        self.shadow_authorities[node_id] = record
        self.shadow_membership[node_id] = node_id
        
        print(f"SHADOW AUTHORITY EMERGED: {node_id} (strength={authority_strength:.3f})")
        
        return node_id
    
    def join_shadow(self, node_id: str, shadow_id: str) -> bool:
        if shadow_id not in self.shadow_authorities:
            return False
        
        record = self.shadow_authorities[shadow_id]
        record.member_nodes.append(node_id)
        self.shadow_membership[node_id] = shadow_id
        
        return True
    
    def get_shadow_for_node(self, node_id: str) -> Optional[ShadowAuthorityRecord]:
        shadow_id = self.shadow_membership.get(node_id)
        if shadow_id:
            return self.shadow_authorities.get(shadow_id)
        return None
    
    def get_all_shadows(self) -> List[ShadowAuthorityRecord]:
        return list(self.shadow_authorities.values())
    
    def compute_shadow_consensus_weight(self, shadow_id: str) -> float:
        record = self.shadow_authorities.get(shadow_id)
        if not record:
            return 0.0
        
        base_weight = record.authority_strength
        size_bonus = min(0.3, len(record.member_nodes) * 0.05)
        distance_penalty = max(0.0, 1.0 - (record.origin_distance / 10.0))
        entropy_penalty = 1.0 - record.entropy_level
        
        return base_weight * distance_penalty * entropy_penalty + size_bonus
    
    def get_shadow_stats(self) -> dict:
        return {
            "total_shadows": len(self.shadow_authorities),
            "total_members": len(self.shadow_membership),
            "average_strength": sum(r.authority_strength for r in self.shadow_authorities.values()) / max(1, len(self.shadow_authorities)),
            "largest_shadow": max(self.shadow_authorities.values(), key=lambda r: len(r.member_nodes)) if self.shadow_authorities else None
        }
