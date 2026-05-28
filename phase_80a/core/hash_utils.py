import hashlib
import json
from typing import Any, Dict, List, Optional

DOMAIN_SEPARATOR_LEAF = b'\x00'
DOMAIN_SEPARATOR_NODE = b'\x01'

def canonicalize(obj: Any) -> str:
    if isinstance(obj, dict):
        return json.dumps(obj, sort_keys=True, separators=(',', ':'), ensure_ascii=False)
    elif hasattr(obj, 'to_dict'):
        return json.dumps(obj.to_dict(), sort_keys=True, separators=(',', ':'), ensure_ascii=False)
    else:
        return json.dumps(obj, sort_keys=True, separators=(',', ':'), ensure_ascii=False)

def sha256_hash(data: str) -> str:
    return hashlib.sha256(data.encode('utf-8')).hexdigest()

def compute_node_hash(node) -> str:
    canonical_data = {
        "node_id": node.node_id,
        "parent_id": node.parent_id,
        "semantic_origin_hash": node.semantic_origin_hash,
        "semantic_invariants": sorted(node.semantic_invariants),
        "constraint_dependency": sorted(node.constraint_dependency),
        "effective_constraint_authority": sorted(node.effective_constraint_authority),
        "derivation_type": node.derivation_type.value,
        "source_text": node.source_text,
        "contextual_modifiers": sorted(node.contextual_modifiers)
    }
    return sha256_hash(canonicalize(canonical_data))

def compute_parent_hash(parent_id: str, parent_hash: str) -> str:
    chain_data = f"{parent_id}:{parent_hash}"
    return sha256_hash(chain_data)

def compute_semantic_commitment(node) -> str:
    commitment_data = {
        "semantic_invariants": sorted(node.semantic_invariants),
        "effective_constraint_authority": sorted(node.effective_constraint_authority),
        "semantic_origin_hash": node.semantic_origin_hash,
        "derivation_type": node.derivation_type.value,
        "contextual_modifiers": sorted(node.contextual_modifiers)
    }
    return sha256_hash(canonicalize(commitment_data))

def verify_node_integrity(node, stored_hash: str) -> bool:
    return compute_node_hash(node) == stored_hash

def verify_lineage_integrity(node, parent_node) -> bool:
    expected_parent_hash = compute_parent_hash(parent_node.node_id, parent_node.node_hash)
    return expected_parent_hash == node.parent_hash

def compute_merkle_root(hashes: List[str]) -> str:
    if not hashes:
        return sha256_hash("empty")
    
    leaf_hashes = []
    for h in hashes:
        leaf_hashes.append(hashlib.sha256(DOMAIN_SEPARATOR_LEAF + h.encode()).hexdigest())
    
    while len(leaf_hashes) > 1:
        next_level = []
        for i in range(0, len(leaf_hashes), 2):
            if i + 1 < len(leaf_hashes):
                combined = leaf_hashes[i] + leaf_hashes[i + 1]
            else:
                combined = leaf_hashes[i] + leaf_hashes[i]
            node_hash = hashlib.sha256(DOMAIN_SEPARATOR_NODE + combined.encode()).hexdigest()
            next_level.append(node_hash)
        leaf_hashes = next_level
    
    return leaf_hashes[0]

def compute_snapshot_id(block_height: int, merkle_root: str, parent_hash: Optional[str] = None) -> str:
    content = {
        "block_height": block_height,
        "merkle_root": merkle_root,
        "parent_hash": parent_hash
    }
    return sha256_hash(canonicalize(content))
