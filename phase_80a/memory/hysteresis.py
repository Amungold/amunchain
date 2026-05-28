from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple
from datetime import datetime
import math
from ..math.ops import to_fixed, from_fixed, ONE, HALF, ZERO, F_add, F_sub, F_mul, F_div, F_min, F_max
from ..math.fixed import FixedNumber

@dataclass
class StateTransition:
    from_state: str
    to_state: str
    timestamp: str
    transition_cost: int
    stability_impact: int

class ConstitutionalHysteresis:
    MIN_RESIDENCY_SECONDS = 300
    DEFAULT_COOLDOWN = 600
    
    TRANSITION_COSTS = {
        ("orthodox", "drifting"): to_fixed(0.1),
        ("drifting", "orthodox"): to_fixed(0.2),
        ("drifting", "schismatic"): to_fixed(0.3),
        ("schismatic", "drifting"): to_fixed(0.4),
        ("schismatic", "shadow"): to_fixed(0.5),
        ("shadow", "schismatic"): to_fixed(0.4),
        ("schismatic", "exiled"): to_fixed(0.6),
        ("exiled", "dead"): to_fixed(0.1),
        ("any", "dead"): to_fixed(0.2),
    }
    
    def __init__(self):
        self.state_history: Dict[str, List[Tuple[str, str]]] = {}
        self.cooldown_remaining: Dict[str, float] = {}
    
    def record_transition(self, node_id: str, from_state: str, to_state: str) -> float:
        timestamp = datetime.utcnow().isoformat() + "Z"
        
        if node_id not in self.state_history:
            self.state_history[node_id] = []
        self.state_history[node_id].append((to_state, timestamp))
        
        cost_key = (from_state, to_state)
        cost = from_fixed(self.TRANSITION_COSTS.get(cost_key, to_fixed(0.3)))
        
        if to_state == "dead":
            cost = 0.1
        
        self.cooldown_remaining[node_id] = self.DEFAULT_COOLDOWN
        
        return cost
    
    def is_transition_allowed(self, node_id: str, from_state: str, to_state: str, 
                               current_legitimacy: float) -> Tuple[bool, str]:
        if node_id in self.cooldown_remaining and self.cooldown_remaining[node_id] > 0:
            return False, f"Cooldown active: {self.cooldown_remaining[node_id]:.0f}s remaining"
        
        history = self.state_history.get(node_id, [])
        if history:
            last_state, last_time = history[-1]
            
            try:
                last_dt = datetime.fromisoformat(last_time.replace('Z', '+00:00'))
                now = datetime.now(last_dt.tzinfo)
                seconds_since_transition = (now - last_dt).total_seconds()
                
                if seconds_since_transition < self.MIN_RESIDENCY_SECONDS:
                    return False, f"Minimum residency not met: {seconds_since_transition:.0f}s < {self.MIN_RESIDENCY_SECONDS}s"
            except:
                pass
        
        if current_legitimacy < 0.15 and to_state not in ["exiled", "dead"]:
            return False, f"Legitimacy too low ({current_legitimacy:.3f}) for transition to {to_state}"
        
        if from_state == to_state:
            return False, "Same state transition"
        
        return True, "Allowed"
    
    def update_cooldown(self, dt_seconds: float):
        for node_id in list(self.cooldown_remaining.keys()):
            self.cooldown_remaining[node_id] -= dt_seconds
            if self.cooldown_remaining[node_id] <= 0:
                del self.cooldown_remaining[node_id]
    
    def get_stability(self, node_id: str) -> float:
        history = self.state_history.get(node_id, [])
        if len(history) < 2:
            return 1.0
        
        transitions = 0
        for i in range(1, len(history)):
            if history[i][0] != history[i-1][0]:
                transitions += 1
        
        if transitions == 0:
            return 1.0
        
        return max(0.0, 1.0 - (transitions / len(history)))

@dataclass
class ConstitutionalAttractor:
    node_id: str
    state: str
    strength: int
    basin_radius: int
    member_nodes: List[str]
    emerged_at: str

class ConstitutionalMemory:
    def __init__(self):
        self.attractors: Dict[str, ConstitutionalAttractor] = {}
        self.persistent_weights: Dict[str, int] = {}
        self.memory_decay_rate = to_fixed(0.01)
    
    def update_persistent_weight(self, node_id: str, delta: int):
        current = self.persistent_weights.get(node_id, ZERO)
        new = F_min(ONE, F_max(ZERO, F_add(current, delta)))
        self.persistent_weights[node_id] = new
    
    def compute_temporal_persistence(self, node_id: str, age_hours: float) -> int:
        base = self.persistent_weights.get(node_id, ZERO)
        age_fixed = to_fixed(age_hours)
        decay = F_exp(F_mul(F_neg(age_fixed), self.memory_decay_rate))
        return F_mul(base, decay)
    
    def register_attractor(self, node_id: str, state: str, strength: int, 
                           basin_radius: int, member_nodes: List[str]) -> str:
        attractor = ConstitutionalAttractor(
            node_id=node_id,
            state=state,
            strength=strength,
            basin_radius=basin_radius,
            member_nodes=member_nodes,
            emerged_at=datetime.utcnow().isoformat() + "Z"
        )
        self.attractors[node_id] = attractor
        return node_id
    
    def get_memory_stats(self) -> dict:
        return {
            "active_attractors": len(self.attractors),
            "nodes_with_memory": len(self.persistent_weights),
        }

@dataclass
class ConstitutionalMemoryVerdict:
    node_id: str
    persistence_score: int
    attractor_state: Optional[str]
    stability_score: int
