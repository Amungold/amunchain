from typing import Dict, List, Tuple, Optional
from ..core.hash_utils import compute_node_hash, compute_parent_hash, compute_semantic_commitment

class IntegrityVerifier:
    def __init__(self, nodes: Dict):
        self.nodes = nodes
    
    def verify_node_integrity(self, node_id: str) -> Tuple[bool, List[str]]:
        node = self.nodes.get(node_id)
        if not node:
            return False, [f"Node {node_id} not found"]
        
        computed_hash = compute_node_hash(node)
        if computed_hash != node.node_hash:
            return False, [f"Node hash mismatch: stored={node.node_hash}, computed={computed_hash}"]
        
        return True, []
    
    def verify_parent_chaining(self, node_id: str) -> Tuple[bool, List[str]]:
        issues = []
        current_id = node_id
        visited = set()
        
        while current_id and current_id not in visited:
            visited.add(current_id)
            current = self.nodes.get(current_id)
            if not current:
                issues.append(f"Node {current_id} not found in chain")
                break
            
            if current.parent_id and current.parent_hash:
                parent = self.nodes.get(current.parent_id)
                if not parent:
                    issues.append(f"Parent {current.parent_id} not found for {current_id}")
                else:
                    expected_hash = compute_parent_hash(parent.node_id, parent.node_hash)
                    if current.parent_hash != expected_hash:
                        issues.append(f"Parent hash mismatch for {current_id}")
            
            current_id = current.parent_id if current.parent_id else None
        
        return len(issues) == 0, issues
    
    def verify_semantic_commitment(self, node_id: str) -> Tuple[bool, List[str]]:
        node = self.nodes.get(node_id)
        if not node:
            return False, [f"Node {node_id} not found"]
        
        computed = compute_semantic_commitment(node)
        if computed != node.semantic_commitment:
            return False, [f"Semantic commitment mismatch for {node_id}"]
        
        return True, []
    
    def verify_full_graph(self) -> Tuple[bool, List[str]]:
        all_issues = []
        
        for node_id in self.nodes:
            ok, issues = self.verify_node_integrity(node_id)
            if not ok:
                all_issues.extend(issues)
            
            ok, issues = self.verify_semantic_commitment(node_id)
            if not ok:
                all_issues.extend(issues)
            
            ok, issues = self.verify_parent_chaining(node_id)
            if not ok:
                all_issues.extend(issues)
        
        return len(all_issues) == 0, all_issues
    
    def get_graph_fingerprint(self) -> str:
        import hashlib
        fingerprints = []
        for node_id in sorted(self.nodes.keys()):
            node = self.nodes[node_id]
            fingerprints.append(f"{node_id}:{node.node_hash}")
        return hashlib.sha256("".join(fingerprints).encode()).hexdigest()
    
    def get_merkle_root(self) -> str:
        from ..core.hash_utils import compute_merkle_root
        sorted_hashes = sorted([node.node_hash for node in self.nodes.values()])
        return compute_merkle_root(sorted_hashes)
