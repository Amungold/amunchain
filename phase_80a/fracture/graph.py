from dataclasses import dataclass, field
from typing import Dict, List, Set, Optional
import math

@dataclass
class AuthorityEdge:
    from_authority: str
    to_authority: str
    legitimacy_score: float
    inheritance_strength: float
    constitutional_distance: int
    entropy_leakage: float
    
    def effective_strength(self) -> float:
        return self.legitimacy_score * self.inheritance_strength * (1.0 - self.entropy_leakage)

class AuthorityLegitimacyGraph:
    def __init__(self):
        self.nodes: Set[str] = set()
        self.edges: Dict[str, List[AuthorityEdge]] = {}
        self.node_weights: Dict[str, float] = {}
    
    def add_node(self, node_id: str, initial_weight: float = 1.0):
        self.nodes.add(node_id)
        self.node_weights[node_id] = initial_weight
        if node_id not in self.edges:
            self.edges[node_id] = []
    
    def add_edge(self, edge: AuthorityEdge):
        if edge.from_authority not in self.nodes:
            self.add_node(edge.from_authority)
        if edge.to_authority not in self.nodes:
            self.add_node(edge.to_authority)
        self.edges[edge.from_authority].append(edge)
    
    def compute_semantic_distance(self, node_a: str, node_b: str) -> float:
        if node_a not in self.nodes or node_b not in self.nodes:
            return float('inf')
        
        # Simplified: later add weighted path computation
        if node_a == node_b:
            return 0.0
        
        # Check direct edge
        for edge in self.edges.get(node_a, []):
            if edge.to_authority == node_b:
                return 1.0 / (1.0 + edge.effective_strength())
        
        return 1.0  # Default distance
    
    def compute_authority_pressure(self, node_id: str) -> float:
        if node_id not in self.nodes:
            return 0.0
        
        pressure = 0.0
        for edge in self.edges.get(node_id, []):
            pressure += edge.effective_strength()
        
        return min(1.0, pressure)
    
    def compute_constitutional_tension(self, node_id: str) -> float:
        if node_id not in self.nodes:
            return 0.0
        
        incoming = 0.0
        outgoing = 0.0
        
        for edge in self.edges.get(node_id, []):
            outgoing += edge.effective_strength()
        
        for src, edges in self.edges.items():
            for edge in edges:
                if edge.to_authority == node_id:
                    incoming += edge.effective_strength()
        
        if incoming + outgoing == 0:
            return 0.0
        
        return abs(incoming - outgoing) / (incoming + outgoing)
    
    def get_dominance_center(self) -> Optional[str]:
        if not self.nodes:
            return None
        return max(self.nodes, key=lambda n: self.node_weights.get(n, 0))
