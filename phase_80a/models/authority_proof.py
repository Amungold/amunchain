from dataclasses import dataclass, field
from typing import List, Optional
from datetime import datetime
from ..core.hash_utils import sha256_hash, canonicalize

@dataclass
class AuthorityTransitionProof:
    from_authority: List[str]
    to_authority: List[str]
    reason: str
    approved_by: Optional[List[str]]
    transition_hash: str
    created_at: str
    signature: Optional[str] = None
    
    @classmethod
    def create(cls, from_authority: List[str], to_authority: List[str],
               reason: str, approved_by: Optional[List[str]] = None) -> 'AuthorityTransitionProof':
        created_at = datetime.utcnow().isoformat() + "Z"
        proof_data = {
            "from": sorted(from_authority),
            "to": sorted(to_authority),
            "reason": reason,
            "approved_by": sorted(approved_by) if approved_by else [],
            "created_at": created_at
        }
        transition_hash = sha256_hash(canonicalize(proof_data))
        
        return cls(
            from_authority=from_authority,
            to_authority=to_authority,
            reason=reason,
            approved_by=approved_by,
            transition_hash=transition_hash,
            created_at=created_at
        )
    
    def to_dict(self) -> dict:
        return {
            "from": self.from_authority,
            "to": self.to_authority,
            "reason": self.reason,
            "approved_by": self.approved_by,
            "transition_hash": self.transition_hash,
            "created_at": self.created_at,
            "signature": self.signature
        }
    
    def verify(self) -> bool:
        proof_data = {
            "from": sorted(self.from_authority),
            "to": sorted(self.to_authority),
            "reason": self.reason,
            "approved_by": sorted(self.approved_by) if self.approved_by else [],
            "created_at": self.created_at
        }
        computed = sha256_hash(canonicalize(proof_data))
        return computed == self.transition_hash
    
    def sign(self, private_key) -> str:
        self.signature = sha256_hash(self.transition_hash + ":signed")
        return self.signature
    
    def verify_signature(self) -> bool:
        if not self.signature:
            return False
        expected = sha256_hash(self.transition_hash + ":signed")
        return self.signature == expected
