from typing import List, Dict, Optional
from .dependency_graph import ConstitutionalDependencyGraph

class SemanticEntropyMonitor:
    def __init__(self, graph: ConstitutionalDependencyGraph):
        self.graph = graph
        self.entropy_history: List[Dict] = []
    
    def compute_constraint_entropy(self, node, origin_id: str) -> float:
        if origin_id not in node.effective_constraint_authority:
            return min(0.95, node.cumulative_entropy + 0.20)
        return node.cumulative_entropy * node.cooling_factor
    
    def compute_authority_entropy(self, node, origin_id: str) -> float:
        if origin_id in node.effective_constraint_authority:
            return node.cumulative_entropy * 0.40
        else:
            return min(0.95, node.cumulative_entropy + 0.30)
    
    def compute_semantic_temperature(self, node) -> float:
        temperature = (
            node.constraint_influence * 0.30 +
            (1.0 - node.cumulative_entropy) * 0.25 +
            node.authority_retention * 0.20 +
            node.participation_activity * 0.10 +
            node.metabolic_rate * 0.10 +
            node.cooling_factor * 0.05
        )
        return max(0.0, min(1.0, temperature))
    
    def compute_recovery_potential(self, node_id: str) -> float:
        node = self.graph.nodes.get(node_id)
        if not node:
            return 0.0
        
        if not node.can_recover():
            return 0.0
        
        recovery = (1.0 - node.cumulative_entropy) * 0.4
        recovery += node.cooling_factor * 0.3
        recovery += node.authority_retention * 0.3
        
        return min(0.95, recovery)
    
    def compute_semantic_heat_death_distance(self, node_id: str) -> float:
        node = self.graph.nodes.get(node_id)
        if not node:
            return float('inf')
        
        current_temp = self.compute_semantic_temperature(node)
        if current_temp <= 0.05:
            return 0.0
        
        if node.constraint_influence >= 0.95:
            return float('inf')
        
        decay_rate = 1.0 - (node.constraint_influence * node.cooling_factor)
        if decay_rate <= 0.01:
            return float('inf')
        
        return (current_temp - 0.05) / decay_rate
    
    def get_entropy_report(self, node_id: str) -> Dict:
        node = self.graph.nodes.get(node_id)
        if not node:
            return {"error": "Node not found"}
        
        origin_id = self.graph._find_origin(node_id)
        
        return {
            "node_id": node_id,
            "origin_id": origin_id,
            "constraint_entropy": self.compute_constraint_entropy(node, origin_id) if origin_id else node.cumulative_entropy,
            "authority_entropy": self.compute_authority_entropy(node, origin_id) if origin_id else node.cumulative_entropy,
            "semantic_temperature": self.compute_semantic_temperature(node),
            "recovery_potential": self.compute_recovery_potential(node_id),
            "heat_death_distance": self.compute_semantic_heat_death_distance(node_id),
            "aliveness": node.aliveness.value,
            "is_critical": node.aliveness.value in ["critical", "dead"],
            "can_recover": node.can_recover(),
            "entropy_level": self._classify_entropy(node, origin_id) if origin_id else "unknown"
        }
    
    def _classify_entropy(self, node, origin_id: str) -> str:
        constraint_entropy = self.compute_constraint_entropy(node, origin_id)
        
        if constraint_entropy < 0.20:
            return "low"
        elif constraint_entropy < 0.45:
            return "medium"
        elif constraint_entropy < 0.70:
            return "high"
        else:
            return "critical"
    
    def monitor_drift_accumulation(self, node_id: str) -> List[Dict]:
        lineage = self.graph.trace_lineage(node_id)
        history = []
        
        for nid in lineage:
            node = self.graph.nodes.get(nid)
            if node:
                origin_id = self.graph._find_origin(nid)
                history.append({
                    "node_id": nid,
                    "constraint_entropy": self.compute_constraint_entropy(node, origin_id) if origin_id else node.cumulative_entropy,
                    "semantic_temperature": self.compute_semantic_temperature(node),
                    "influence": node.constraint_influence,
                    "cooling_factor": node.cooling_factor,
                    "aliveness": node.aliveness.value
                })
        
        return history
