from typing import List, Dict, Optional
from ..core.enums import ConsensusRole, AlivenessState
from ..models.interpretive_node import InterpretiveNode
from ..models.authority_proof import AuthorityTransitionProof
from ..models.merkle_snapshot import MerkleSnapshot
from .semantic_gate import SemanticConsensusGate

class ConstitutionalValidator:
    def __init__(self, validator_id: str, gate: SemanticConsensusGate):
        self.validator_id = validator_id
        self.gate = gate
        self.role = ConsensusRole.VALIDATOR
        self.reputation_score = 1.0
    
    def validate_interpretation(self, node: InterpretiveNode) -> Dict:
        if not node.parent_id or not node.parent_hash:
            return {
                "valid": False,
                "reason": "Missing parent linkage",
                "validator": self.validator_id
            }
        
        parent = self.gate.graph.nodes.get(node.parent_id)
        if parent and not parent.is_alive():
            return {
                "valid": False,
                "reason": f"Parent {node.parent_id} is dead",
                "validator": self.validator_id
            }
        
        if parent:
            origin_id = self.gate.graph._find_origin(node.node_id)
            if origin_id:
                entropy = self.gate.entropy_monitor.compute_constraint_entropy(parent, origin_id)
                if entropy > 0.7:
                    return {
                        "valid": False,
                        "reason": f"Parent entropy too high: {entropy:.2f}",
                        "validator": self.validator_id
                    }
        
        if node.authority_proof:
            if not node.authority_proof.verify():
                return {
                    "valid": False,
                    "reason": "Invalid authority transition proof",
                    "validator": self.validator_id
                }
        
        return {
            "valid": True,
            "reason": "All checks passed",
            "validator": self.validator_id,
            "weight": self.reputation_score
        }
    
    def validate_authority_transition(self, proof: AuthorityTransitionProof) -> Dict:
        if not proof.verify():
            return {
                "valid": False,
                "reason": "Proof hash mismatch",
                "validator": self.validator_id
            }
        
        for node_id in proof.from_authority:
            if node_id not in self.gate.graph.nodes:
                return {
                    "valid": False,
                    "reason": f"From authority node {node_id} not found",
                    "validator": self.validator_id
                }
        
        for node_id in proof.to_authority:
            if node_id not in self.gate.graph.nodes:
                return {
                    "valid": False,
                    "reason": f"To authority node {node_id} not found",
                    "validator": self.validator_id
                }
        
        all_validators = [self.validator_id]
        quorum = self.gate.get_semantic_quorum(all_validators, 0.5)
        
        return {
            "valid": quorum,
            "reason": "Authority transition validated" if quorum else "Insufficient quorum",
            "validator": self.validator_id
        }
    
    def vote_on_snapshot(self, snapshot: MerkleSnapshot) -> Dict:
        if self.gate.verify_snapshot(snapshot):
            return {
                "vote": "APPROVE",
                "validator": self.validator_id,
                "weight": self.reputation_score
            }
        else:
            return {
                "vote": "REJECT",
                "validator": self.validator_id,
                "weight": self.reputation_score,
                "reason": "Snapshot verification failed"
            }
    
    def report_status(self) -> Dict:
        return {
            "validator_id": self.validator_id,
            "role": self.role.value,
            "reputation_score": self.reputation_score,
            "active": True
        }
