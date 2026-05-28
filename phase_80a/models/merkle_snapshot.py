from dataclasses import dataclass, field
from typing import List, Dict, Optional
from datetime import datetime
from ..core.hash_utils import sha256_hash, compute_merkle_root, canonicalize, compute_snapshot_id

@dataclass
class MerkleSnapshot:
    snapshot_id: str
    block_height: int
    merkle_root: str
    node_count: int
    node_hashes: List[str]
    created_at: str
    parent_snapshot_hash: Optional[str] = None
    signature: Optional[str] = None
    
    @classmethod
    def create(cls, block_height: int, node_hashes: List[str], 
               parent_snapshot_hash: Optional[str] = None) -> 'MerkleSnapshot':
        created_at = datetime.utcnow().isoformat() + "Z"
        merkle_root = compute_merkle_root(sorted(node_hashes))
        snapshot_id = compute_snapshot_id(block_height, merkle_root, parent_snapshot_hash)
        
        return cls(
            snapshot_id=snapshot_id,
            block_height=block_height,
            merkle_root=merkle_root,
            node_count=len(node_hashes),
            node_hashes=sorted(node_hashes),
            created_at=created_at,
            parent_snapshot_hash=parent_snapshot_hash
        )
    
    def to_dict(self) -> dict:
        return {
            "snapshot_id": self.snapshot_id,
            "block_height": self.block_height,
            "merkle_root": self.merkle_root,
            "node_count": self.node_count,
            "node_hashes": self.node_hashes[:100],
            "created_at": self.created_at,
            "parent_snapshot_hash": self.parent_snapshot_hash,
            "signature": self.signature
        }
    
    def verify(self, node_hashes: List[str]) -> bool:
        computed_root = compute_merkle_root(sorted(node_hashes))
        return computed_root == self.merkle_root
    
    def verify_chain(self, previous_snapshot: 'MerkleSnapshot') -> bool:
        if not self.parent_snapshot_hash:
            return previous_snapshot is None
        if not previous_snapshot:
            return False
        expected_hash = sha256_hash(canonicalize(previous_snapshot.to_dict()))
        return self.parent_snapshot_hash == expected_hash
    
    def sign(self, private_key) -> str:
        self.signature = sha256_hash(self.snapshot_id + ":signed")
        return self.signature
    
    def verify_signature(self) -> bool:
        if not self.signature:
            return False
        expected = sha256_hash(self.snapshot_id + ":signed")
        return self.signature == expected
