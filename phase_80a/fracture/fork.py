from dataclasses import dataclass, field
from typing import List, Dict, Optional, Set
from datetime import datetime
from enum import Enum

class ForkType(Enum):
    DOCTRINAL = "doctrinal"
    AUTHORITY = "authority"
    INTERPRETIVE = "interpretive"
    CONSENSUS = "consensus"
    EXILED = "exiled"

@dataclass
class ConstitutionalFork:
    fork_id: str
    fork_type: ForkType
    emerged_at: str
    origin_node_id: str
    member_nodes: List[str]
    shadow_authority_id: Optional[str]
    legitimacy_score: float
    entropy_level: float
    is_active: bool = True

class ConstitutionalForkDetector:
    def __init__(self):
        self.forks: Dict[str, ConstitutionalFork] = {}
        self.fork_membership: Dict[str, str] = {}
    
    def detect_fork(self, node_ids: List[str], fork_type: ForkType,
                    origin_node_id: str, shadow_id: Optional[str] = None,
                    legitimacy_score: float = 0.5, entropy_level: float = 0.5) -> Optional[str]:
        if len(node_ids) < 2:
            return None
        
        import hashlib
        fork_id = hashlib.sha256(f"{origin_node_id}:{datetime.utcnow().isoformat()}".encode()).hexdigest()[:16]
        
        fork = ConstitutionalFork(
            fork_id=fork_id,
            fork_type=fork_type,
            emerged_at=datetime.utcnow().isoformat() + "Z",
            origin_node_id=origin_node_id,
            member_nodes=node_ids.copy(),
            shadow_authority_id=shadow_id,
            legitimacy_score=legitimacy_score,
            entropy_level=entropy_level,
            is_active=True
        )
        
        self.forks[fork_id] = fork
        for nid in node_ids:
            self.fork_membership[nid] = fork_id
        
        print(f"FORK DETECTED: {fork_id} - Type: {fork_type.value} - Members: {len(node_ids)}")
        
        return fork_id
    
    def get_fork_for_node(self, node_id: str) -> Optional[ConstitutionalFork]:
        fork_id = self.fork_membership.get(node_id)
        if fork_id:
            return self.forks.get(fork_id)
        return None
    
    def is_forked(self, node_id: str) -> bool:
        return node_id in self.fork_membership
    
    def get_all_forks(self) -> List[ConstitutionalFork]:
        return list(self.forks.values())
    
    def get_active_forks(self) -> List[ConstitutionalFork]:
        return [f for f in self.forks.values() if f.is_active]
    
    def get_fork_stats(self) -> dict:
        return {
            "total_forks": len(self.forks),
            "active_forks": len(self.get_active_forks()),
            "total_forked_nodes": len(self.fork_membership),
            "by_type": {
                ft.value: len([f for f in self.forks.values() if f.fork_type == ft])
                for ft in ForkType
            }
        }
