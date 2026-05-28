from typing import List, Dict, Optional, Tuple
from .types import ConstitutionalState, FractureType, ConstitutionalMetrics
from .graph import AuthorityLegitimacyGraph
from .transition import ConstitutionalTransitionEngine

class FractureDetector:
    def __init__(self, graph: AuthorityLegitimacyGraph):
        self.graph = graph
        self.transition_engine = ConstitutionalTransitionEngine()
        self.node_states: Dict[str, ConstitutionalState] = {}
    
    def detect_anomalies(self, node_id: str, metrics: ConstitutionalMetrics) -> List[Tuple[FractureType, float]]:
        anomalies = []
        
        if metrics.authority_retention < 0.4:
            anomalies.append((FractureType.SHADOW_AUTHORITY, 1.0 - metrics.authority_retention))
        
        if metrics.origin_distance > 0.7:
            anomalies.append((FractureType.ORIGIN_SEVERANCE, metrics.origin_distance))
        
        if metrics.consensus_weight < 0.2 and metrics.cumulative_entropy > 0.6:
            anomalies.append((FractureType.PARASITIC_LINEAGE, (1.0 - metrics.consensus_weight) * metrics.cumulative_entropy))
        
        if metrics.cumulative_entropy > 0.8:
            anomalies.append((FractureType.DOCTRINAL_COLLAPSE, metrics.cumulative_entropy))
        
        authority_pressure = self.graph.compute_authority_pressure(node_id)
        if authority_pressure > 0.7:
            anomalies.append((FractureType.EMERGENT_AUTHORITY, authority_pressure))
        
        return sorted(anomalies, key=lambda x: x[1], reverse=True)
    
    def update_state(self, node_id: str, metrics: ConstitutionalMetrics) -> Tuple[ConstitutionalState, Optional[FractureType]]:
        current_state = self.node_states.get(node_id, ConstitutionalState.ORTHODOX)
        
        new_state, fracture = self.transition_engine.compute_next_state(metrics, current_state)
        
        self.node_states[node_id] = new_state
        
        return new_state, fracture
    
    def detect_collective_divergence(self, node_metrics: Dict[str, ConstitutionalMetrics]) -> List[str]:
        divergent_nodes = []
        
        for node_id, metrics in node_metrics.items():
            state, _ = self.update_state(node_id, metrics)
            if state in [ConstitutionalState.SCHISMATIC, ConstitutionalState.SHADOW, ConstitutionalState.EXILED]:
                divergent_nodes.append(node_id)
        
        if len(divergent_nodes) > len(node_metrics) * 0.3:
            return divergent_nodes
        
        return divergent_nodes
