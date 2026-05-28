from typing import List, Dict, Optional, Tuple
from ..core.enums import AlivenessState
from ..runtime.dependency_graph import ConstitutionalDependencyGraph
from ..runtime.admissibility_engine import AdmissibilityEngine
from ..runtime.semantic_entropy_monitor import SemanticEntropyMonitor
from ..models.interpretive_node import InterpretiveNode
from ..models.merkle_snapshot import MerkleSnapshot
from ..core.hash_utils import compute_merkle_root

class SemanticConsensusGate:
    def __init__(self, graph: ConstitutionalDependencyGraph):
        self.graph = graph
        self.admissibility = AdmissibilityEngine(graph)
        self.entropy_monitor = SemanticEntropyMonitor(graph)
    
    def validate_for_consensus(self, node_id: str) -> Tuple[bool, List[str]]:
        reasons = []
        
        status = self.admissibility.get_full_admissibility_status(node_id)
        if not status['overall_admissible']:
            reasons.append(f"Admissibility failed: {status['verdict']}")
            return False, reasons
        
        node = self.graph.nodes.get(node_id)
        if not node:
            return False, ["Node not found"]
        
        if not node.is_alive():
            reasons.append(f"Node is {node.aliveness.value}")
            return False, reasons
        
        if not node.is_necessary():
            reasons.append(f"Node not constitutionally necessary (influence={node.constraint_influence:.3f}, entropy={node.cumulative_entropy:.3f}, authority_retention={node.authority_retention:.3f}, cooling={node.cooling_factor:.3f})")
            return False, reasons
        
        origin_id = self.graph._find_origin(node_id)
        if origin_id:
            entropy = self.entropy_monitor.compute_constraint_entropy(node, origin_id)
            if entropy > 0.68:
                reasons.append(f"Entropy too high: {entropy:.3f} > 0.68")
                return False, reasons
            
            temperature = self.entropy_monitor.compute_semantic_temperature(node)
            if temperature < 0.32:
                reasons.append(f"Temperature too low: {temperature:.3f} < 0.32")
                return False, reasons
            
            authority_overlap = len(set(node.effective_constraint_authority) & {origin_id}) if node.effective_constraint_authority else 0
            if authority_overlap == 0 and origin_id in node.constraint_dependency:
                reasons.append(f"Authority migration detected: origin {origin_id} declared but not effective")
                return False, reasons
        
        recovery = self.entropy_monitor.compute_recovery_potential(node_id)
        if recovery < 0.20 and node.cumulative_entropy > 0.60:
            reasons.append(f"Recovery potential too low: {recovery:.3f}")
            return False, reasons
        
        return True, []
    
    def get_consensus_weight(self, node_id: str) -> float:
        node = self.graph.nodes.get(node_id)
        if not node:
            return 0.0
        
        if not node.is_alive():
            return 0.0
        
        if not node.is_necessary():
            return 0.0
        
        base_weight = self.entropy_monitor.compute_semantic_temperature(node)
        
        origin_id = self.graph._find_origin(node_id)
        if origin_id:
            authority_overlap = len(set(node.effective_constraint_authority) & {origin_id}) if node.effective_constraint_authority else 0
            if authority_overlap == 0:
                base_weight = base_weight * 0.30
            else:
                base_weight = base_weight * (0.65 + 0.35 * node.authority_retention)
        
        entropy_penalty = 1.0 - self.entropy_monitor.compute_constraint_entropy(node, origin_id) if origin_id else 1.0
        
        recovery_bonus = self.entropy_monitor.compute_recovery_potential(node_id) * 0.15
        
        final_weight = base_weight * entropy_penalty * node.authority_retention * node.cooling_factor
        final_weight = final_weight + recovery_bonus
        
        return max(0.0, min(1.0, final_weight))
    
    def get_semantic_quorum(self, node_ids: List[str], threshold: float = 0.66) -> bool:
        total_alive_weight = 0.0
        total_possible = 0.0
        
        for nid in node_ids:
            node = self.graph.nodes.get(nid)
            if node and node.is_alive():
                weight = self.get_consensus_weight(nid)
                total_alive_weight += weight
                total_possible += 1.0
        
        if total_possible == 0:
            return False
        
        alive_ratio = total_alive_weight / total_possible
        
        return alive_ratio >= threshold
    
    def reinforce_node(self, node_id: str) -> Optional[InterpretiveNode]:
        node = self.graph.nodes.get(node_id)
        if not node:
            return None
        
        if not node.can_recover():
            print(f"Cannot reinforce {node_id}: recovery impossible")
            return None
        
        reinforced = node.reinforce()
        self.graph.nodes[node_id] = reinforced
        
        print(f"REINFORCED: {node_id} (influence={reinforced.constraint_influence:.3f}, entropy={reinforced.cumulative_entropy:.3f}, recovery_count={reinforced.reinforcement_count})")
        
        return reinforced
    
    def create_state_snapshot(self, block_height: int) -> MerkleSnapshot:
        all_hashes = [node.node_hash for node in self.graph.nodes.values()]
        return MerkleSnapshot.create(block_height, all_hashes)
    
    def verify_snapshot(self, snapshot: MerkleSnapshot) -> bool:
        current_hashes = [node.node_hash for node in self.graph.nodes.values()]
        return snapshot.verify(current_hashes)
