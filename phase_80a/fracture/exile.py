from enum import Enum
from dataclasses import dataclass, field
from typing import List, Optional, Dict
from datetime import datetime

class ExileReason(Enum):
    ORIGIN_SEVERANCE = "origin_severance"
    AUTHORITY_COLLAPSE = "authority_collapse"
    CONSENSUS_REJECTION = "consensus_rejection"
    HERESY_CONVICTION = "heresy_conviction"
    SHADOW_ACTIVATION = "shadow_activation"
    COLLECTIVE_DIVERGENCE = "collective_divergence"

@dataclass
class ExileRecord:
    node_id: str
    exile_reason: ExileReason
    exiled_at: str
    exiled_by: str
    entropy_at_exile: float
    influence_at_exile: float
    authority_retention_at_exile: float
    can_return: bool = False
    return_deadline: Optional[str] = None
    rehabilitation_progress: float = 0.0

class ConstitutionalExileManager:
    def __init__(self):
        self.exiled_nodes: Dict[str, ExileRecord] = {}
        self.exile_history: List[ExileRecord] = []
    
    def exile_node(self, node_id: str, reason: ExileReason, 
                   entropy: float, influence: float, 
                   authority_retention: float, exiled_by: str = "system") -> bool:
        if node_id in self.exiled_nodes:
            return False
        
        record = ExileRecord(
            node_id=node_id,
            exile_reason=reason,
            exiled_at=datetime.utcnow().isoformat() + "Z",
            exiled_by=exiled_by,
            entropy_at_exile=entropy,
            influence_at_exile=influence,
            authority_retention_at_exile=authority_retention,
            can_return=reason in [ExileReason.SHADOW_ACTIVATION, ExileReason.COLLECTIVE_DIVERGENCE],
            return_deadline=None,
            rehabilitation_progress=0.0
        )
        
        if record.can_return:
            record.return_deadline = datetime.utcnow().isoformat() + "Z"
        
        self.exiled_nodes[node_id] = record
        self.exile_history.append(record)
        
        print(f"EXILED: {node_id} - Reason: {reason.value}")
        return True
    
    def is_exiled(self, node_id: str) -> bool:
        return node_id in self.exiled_nodes
    
    def get_exile_record(self, node_id: str) -> Optional[ExileRecord]:
        return self.exiled_nodes.get(node_id)
    
    def rehabilitate(self, node_id: str, progress_increment: float = 0.1) -> bool:
        if node_id not in self.exiled_nodes:
            return False
        
        record = self.exiled_nodes[node_id]
        if not record.can_return:
            return False
        
        record.rehabilitation_progress += progress_increment
        
        if record.rehabilitation_progress >= 1.0:
            del self.exiled_nodes[node_id]
            print(f"REHABILITATED: {node_id} has returned from exile")
            return True
        
        return False
    
    def get_all_exiled(self) -> List[str]:
        return list(self.exiled_nodes.keys())
    
    def get_exile_stats(self) -> dict:
        return {
            "total_exiled": len(self.exiled_nodes),
            "total_history": len(self.exile_history),
            "rehabilitated": len([r for r in self.exile_history if r not in self.exiled_nodes]),
            "by_reason": {
                reason.value: len([r for r in self.exile_history if r.exile_reason == reason])
                for reason in ExileReason
            }
        }
