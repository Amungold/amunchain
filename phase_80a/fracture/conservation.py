from dataclasses import dataclass, field
from typing import Dict, List, Optional
import hashlib
from ..math.ops import (
    to_fixed, from_fixed, to_fixed_int, from_fixed_int,
    F_add, F_sub, F_mul, F_div,
    ZERO, ONE
)

@dataclass
class LegitimacyTransaction:
    from_node: str
    to_node: str
    amount: int
    transaction_type: str
    tick_number: int
    proof_hash: Optional[str] = None

class GenesisSeal:
    def __init__(self, chain_id: str, initial_supply: int, genesis_hash: str):
        self.chain_id = chain_id
        self.initial_supply = initial_supply
        self.genesis_hash = genesis_hash
        self.sealed = False
    
    def seal(self, validator_signatures: List[str]) -> str:
        if self.sealed:
            return self.genesis_hash
        combined = f"{self.chain_id}:{self.initial_supply}:{''.join(sorted(validator_signatures))}"
        self.genesis_hash = hashlib.sha256(combined.encode()).hexdigest()
        self.sealed = True
        return self.genesis_hash
    
    def verify(self, validator_signatures: List[str]) -> bool:
        combined = f"{self.chain_id}:{self.initial_supply}:{''.join(sorted(validator_signatures))}"
        expected_hash = hashlib.sha256(combined.encode()).hexdigest()
        return expected_hash == self.genesis_hash

class LegitimacyConservationEngine:
    def __init__(self, initial_total_legitimacy: int = ONE):
        self.total_legitimacy = initial_total_legitimacy
        self.committed: Dict[str, int] = {}
        self.transactions: List[LegitimacyTransaction] = []
        self.entropy_account: Dict[str, int] = {}
        self.current_tick = 0
        self.genesis_seal: Optional[GenesisSeal] = None
        self.is_sealed = False
    
    def set_genesis(self, chain_id: str, initial_supply: int, validator_signatures: List[str]) -> bool:
        if self.is_sealed:
            return False
        
        self.genesis_seal = GenesisSeal(chain_id, to_fixed_int(initial_supply), "")
        genesis_hash = self.genesis_seal.seal(validator_signatures)
        self.total_legitimacy = to_fixed_int(initial_supply)
        self.is_sealed = True
        return True
    
    def verify_genesis(self, validator_signatures: List[str]) -> bool:
        if not self.genesis_seal:
            return False
        return self.genesis_seal.verify(validator_signatures)
    
    def set_current_tick(self, tick: int):
        self.current_tick = tick
    
    def mint_legitimacy(self, node_id: str, amount: int, proof: str) -> bool:
        """Minting only allowed before sealing or with constitutional proof."""
        if not self.is_sealed:
            self.total_legitimacy = F_add(self.total_legitimacy, amount)
            self.committed[node_id] = F_add(self.committed.get(node_id, ZERO), amount)
            return True
        return False
    
    def allocate_legitimacy(self, node_id: str, amount: int) -> bool:
        available = F_sub(self.total_legitimacy, self._total_committed())
        if amount > available:
            return False
        
        self.committed[node_id] = F_add(self.committed.get(node_id, ZERO), amount)
        return True
    
    def transfer_legitimacy(self, from_node: str, to_node: str, amount: int) -> bool:
        from_balance = self.committed.get(from_node, ZERO)
        if amount > from_balance:
            return False
        
        self.committed[from_node] = F_sub(from_balance, amount)
        self.committed[to_node] = F_add(self.committed.get(to_node, ZERO), amount)
        
        self.transactions.append(LegitimacyTransaction(
            from_node=from_node,
            to_node=to_node,
            amount=amount,
            transaction_type="transfer",
            tick_number=self.current_tick
        ))
        
        return True
    
    def burn_legitimacy(self, node_id: str, amount: int, reason: str) -> bool:
        from_balance = self.committed.get(node_id, ZERO)
        if amount > from_balance:
            return False
        
        self.committed[node_id] = F_sub(from_balance, amount)
        self.total_legitimacy = F_sub(self.total_legitimacy, amount)
        
        self.transactions.append(LegitimacyTransaction(
            from_node=node_id,
            to_node="__burn__",
            amount=amount,
            transaction_type=f"burn:{reason}",
            tick_number=self.current_tick
        ))
        
        return True
    
    def _total_committed(self) -> int:
        total = ZERO
        for v in self.committed.values():
            total = F_add(total, v)
        return total
    
    def get_balance(self, node_id: str) -> int:
        return self.committed.get(node_id, ZERO)
    
    def get_total_legitimacy(self) -> int:
        return self.total_legitimacy
    
    def get_conservation_report(self) -> dict:
        total_committed = self._total_committed()
        return {
            "total_legitimacy": from_fixed(self.total_legitimacy),
            "committed_legitimacy": from_fixed(total_committed),
            "available_legitimacy": from_fixed(F_sub(self.total_legitimacy, total_committed)),
            "conserved": total_committed <= self.total_legitimacy,
            "transaction_count": len(self.transactions),
            "active_nodes": len(self.committed),
            "genesis_sealed": self.is_sealed
        }
