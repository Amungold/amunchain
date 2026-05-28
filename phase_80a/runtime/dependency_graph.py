from typing import Dict, List, Optional, Tuple
from datetime import datetime
from ..core.enums import AlivenessState
from ..models.interpretive_node import InterpretiveNode

class ConstitutionalDependencyGraph:
    def __init__(self):
        self.nodes: Dict[str, InterpretiveNode] = {}
        self.violations_log: List[Dict] = []
        self.suffocation_alerts: List[Dict] = []
        self.origin_cache: Dict[str, str] = {}
    
    def add_node(self, node: InterpretiveNode) -> Tuple[bool, List[str]]:
        violations = []
        
        if node.node_id == "I4":
            print(f"[DEBUG ADD_NODE BEFORE] I4 effective_constraint_authority = {node.effective_constraint_authority}")
        
        if node.parent_id and node.parent_id in self.nodes:
            parent = self.nodes[node.parent_id]
            if parent.aliveness == AlivenessState.DEAD:
                violations.append(f"NECROMANCY: Cannot derive from DEAD node {node.parent_id}")
                return False, violations
        
        self.nodes[node.node_id] = node
        
        if node.node_id == "I4":
            print(f"[DEBUG ADD_NODE AFTER STORE] I4 effective_constraint_authority = {self.nodes[node.node_id].effective_constraint_authority}")
        
        if node.parent_id and node.parent_id in self.nodes:
            self.nodes[node.parent_id].children.append(node.node_id)
        
        origin_id = self._find_origin(node.parent_id) if node.parent_id else node.node_id
        
        if origin_id and origin_id not in node.constraint_dependency:
            alert = {
                "type": "dependency_shift",
                "node_id": node.node_id,
                "origin_id": origin_id,
                "timestamp": datetime.utcnow().isoformat() + "Z"
            }
            self.suffocation_alerts.append(alert)
            print(f"SUFFOCATION WARNING: Origin '{origin_id}' missing from deps of {node.node_id}")
            print(f"   Effective authority: {node.effective_constraint_authority}")
            print(f"   Constraint influence: {node.constraint_influence:.3f}")
            print(f"   Cumulative entropy: {node.cumulative_entropy:.3f}")
        
        if node.node_id == "I4":
            print(f"[DEBUG ADD_NODE FINAL] I4 effective_constraint_authority = {self.nodes[node.node_id].effective_constraint_authority}")
        
        print(f"ADDED: {node.node_id} (aliveness={node.aliveness.value}, influence={node.constraint_influence:.3f}, entropy={node.cumulative_entropy:.3f})")
        return True, violations
    
    def _find_origin(self, node_id: Optional[str]) -> Optional[str]:
        if not node_id:
            return None
        if node_id in self.origin_cache:
            return self.origin_cache[node_id]
        
        node = self.nodes.get(node_id)
        if not node:
            return None
        if node.parent_id is None:
            self.origin_cache[node_id] = node.node_id
            return node.node_id
        origin = self._find_origin(node.parent_id)
        if origin:
            self.origin_cache[node_id] = origin
        return origin
    
    def trace_lineage(self, node_id: str) -> List[str]:
        lineage = []
        current_id = node_id
        visited = set()
        
        while current_id and current_id not in visited:
            visited.add(current_id)
            lineage.append(current_id)
            node = self.nodes.get(current_id)
            if not node or not node.parent_id:
                break
            current_id = node.parent_id
        
        return list(reversed(lineage))
    
    def detect_authority_migration(self, node_id: str) -> Optional[Dict]:
        node = self.nodes.get(node_id)
        if not node:
            return None
        
        origin_id = self._find_origin(node.node_id)
        if not origin_id:
            return None
        
        origin_node = self.nodes.get(origin_id)
        if origin_node and origin_node.is_alive():
            if origin_id not in node.effective_constraint_authority:
                return {
                    "migration_detected": True,
                    "origin_id": origin_id,
                    "declared_dependency": node.constraint_dependency,
                    "effective_authority": node.effective_constraint_authority,
                    "influence": node.constraint_influence,
                    "entropy": node.cumulative_entropy,
                    "severity": "critical"
                }
        return {"migration_detected": False}
    
    def get_status(self, node_id: str) -> Optional[dict]:
        node = self.nodes.get(node_id)
        if not node:
            return None
        
        if node_id == "I4":
            print(f"[DEBUG GET_STATUS] I4 effective_constraint_authority = {node.effective_constraint_authority}")
        
        return {
            "node_id": node.node_id,
            "aliveness": node.aliveness.value,
            "is_alive": node.is_alive(),
            "is_necessary": node.is_necessary(),
            "constraint_dependency": node.constraint_dependency,
            "effective_authority": node.effective_constraint_authority,
            "constraint_influence": node.constraint_influence,
            "cumulative_entropy": node.cumulative_entropy,
            "lineage": self.trace_lineage(node_id),
            "children": node.children
        }
    
    def print_report(self):
        print("\n" + "=" * 60)
        print("CONSTITUTIONAL DEPENDENCY GRAPH REPORT")
        print("=" * 60)
        for node_id, node in self.nodes.items():
            status = self.get_status(node_id)
            print(f"\nNODE {node_id}:")
            print(f"   Aliveness: {status['aliveness']}")
            print(f"   Influence: {node.constraint_influence:.3f}")
            print(f"   Entropy: {node.cumulative_entropy:.3f}")
            print(f"   Necessary: {status['is_necessary']}")
            print(f"   Declared deps: {node.constraint_dependency}")
            print(f"   Effective authority: {node.effective_constraint_authority}")
            print(f"   Lineage: {' -> '.join(status['lineage'])}")
        print(f"\nSuffocation alerts: {len(self.suffocation_alerts)}")
        print(f"Violations: {len(self.violations_log)}")
    
    def get_all_nodes(self) -> Dict[str, InterpretiveNode]:
        return self.nodes
