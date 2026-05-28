from typing import List, Dict, Optional, Tuple
from .dependency_graph import ConstitutionalDependencyGraph
from .semantic_entropy_monitor import SemanticEntropyMonitor

class AdmissibilityEngine:
    def __init__(self, graph: ConstitutionalDependencyGraph):
        self.graph = graph
        self.entropy_monitor = SemanticEntropyMonitor(graph)
    
    def is_structurally_admissible(self, node_id: str) -> Tuple[bool, List[str]]:
        node = self.graph.nodes.get(node_id)
        if not node:
            return False, ["Node not found"]
        
        issues = []
        
        if node.lineage_status != "complete":
            issues.append(f"Incomplete lineage: {node.lineage_status}")
        
        if node.semantic_origin_hash and node.parent_id:
            origin = self.graph._find_origin(node_id)
            if not origin:
                issues.append("Cannot trace to origin")
        
        if node.authenticity_status == "fabricated":
            issues.append("Fabricated lineage detected")
        
        return len(issues) == 0, issues
    
    def is_constitutionally_alive(self, node_id: str) -> Tuple[bool, List[str]]:
        node = self.graph.nodes.get(node_id)
        if not node:
            return False, ["Node not found"]
        
        issues = []
        
        if not node.is_alive():
            issues.append(f"Node is {node.aliveness.value}")
        
        if not node.is_necessary():
            issues.append(f"Not constitutionally necessary (influence={node.constraint_influence})")
        
        temp = self.entropy_monitor.compute_semantic_temperature(node)
        if temp < 0.3:
            issues.append(f"Semantic temperature too low: {temp:.2f}")
        
        origin_id = self.graph._find_origin(node_id)
        if origin_id:
            constraint_entropy = self.entropy_monitor.compute_constraint_entropy(node, origin_id)
            if constraint_entropy > 0.7:
                issues.append(f"Constraint entropy too high: {constraint_entropy:.2f}")
        
        return len(issues) == 0, issues
    
    def is_semantically_authentic(self, node_id: str) -> Tuple[bool, List[str]]:
        node = self.graph.nodes.get(node_id)
        if not node:
            return False, ["Node not found"]
        
        issues = []
        
        if node.authenticity_status != "authentic":
            issues.append(f"Not authentic: {node.authenticity_status}")
        
        if node.derivation_type.value in ["mutation", "severance"]:
            issues.append(f"Forbidden derivation type: {node.derivation_type.value}")
        
        if node.semantic_delta and node.semantic_delta.is_severe():
            issues.append(f"Severe drift detected: {node.semantic_delta.drift_level.value}")
        
        return len(issues) == 0, issues
    
    def get_full_admissibility_status(self, node_id: str) -> Dict:
        struct_ok, struct_issues = self.is_structurally_admissible(node_id)
        alive_ok, alive_issues = self.is_constitutionally_alive(node_id)
        authentic_ok, authentic_issues = self.is_semantically_authentic(node_id)
        
        overall = struct_ok and alive_ok and authentic_ok
        
        return {
            "node_id": node_id,
            "overall_admissible": overall,
            "structurally_admissible": struct_ok,
            "structurally_issues": struct_issues,
            "constitutionally_alive": alive_ok,
            "aliveness_issues": alive_issues,
            "semantically_authentic": authentic_ok,
            "authenticity_issues": authentic_issues,
            "verdict": "VALID" if overall else "REJECTED"
        }
    
    def can_derive_from(self, parent_id: str, child_id: str) -> Tuple[bool, List[str]]:
        parent = self.graph.nodes.get(parent_id)
        if not parent:
            return False, [f"Parent {parent_id} not found"]
        
        issues = []
        
        if not parent.is_alive():
            issues.append(f"Parent is {parent.aliveness.value}")
        
        if not parent.is_necessary():
            issues.append(f"Parent not constitutionally necessary (influence={parent.constraint_influence})")
        
        if parent.authenticity_status != "authentic":
            issues.append(f"Parent authenticity: {parent.authenticity_status}")
        
        temp = self.entropy_monitor.compute_semantic_temperature(parent)
        if temp < 0.4:
            issues.append(f"Parent semantic temperature too low: {temp:.2f}")
        
        return len(issues) == 0, issues
