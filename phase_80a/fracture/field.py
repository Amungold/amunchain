from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple
from collections import deque
from ..math.ops import (
    to_fixed, from_fixed, to_fixed_int,
    F_add, F_sub, F_mul, F_div, F_sqrt,
    ZERO, ONE, HALF
)

@dataclass
class FieldNode:
    node_id: str
    position_x: int
    position_y: int
    authority_strength: int
    entropy_level: int
    legitimacy_balance: int

@dataclass
class PropagationEvent:
    from_node: str
    to_node: str
    influence: int
    arrival_tick: int
    processed: bool = False

class ConstitutionalFieldTopology:
    """
    Field topology with causal propagation delays, double-buffer updates,
    and conservation of field influence.
    """
    
    def __init__(self, field_radius: int = 10, attenuation_factor: float = 0.7):
        self.nodes: Dict[str, FieldNode] = {}
        self.current_buffer: Dict[str, int] = {}
        self.next_buffer: Dict[str, int] = {}
        self.propagation_queue: deque = deque()
        self.field_radius = field_radius
        self.attenuation_factor = to_fixed(attenuation_factor)
        self.current_tick = 0
        self.total_field_energy = ZERO
    
    def set_current_tick(self, tick: int):
        self.current_tick = tick
    
    def add_node(self, node_id: str, pos_x: int, pos_y: int, 
                 authority: int, entropy: int, legitimacy: int):
        self.nodes[node_id] = FieldNode(
            node_id=node_id,
            position_x=pos_x,
            position_y=pos_y,
            authority_strength=authority,
            entropy_level=entropy,
            legitimacy_balance=legitimacy
        )
        self.current_buffer[node_id] = legitimacy
        self.next_buffer[node_id] = legitimacy
        self.total_field_energy = F_add(self.total_field_energy, legitimacy)
    
    def _distance_squared(self, node_a: FieldNode, node_b: FieldNode) -> int:
        dx = node_a.position_x - node_b.position_x
        dy = node_a.position_y - node_b.position_y
        return dx * dx + dy * dy
    
    def _fixed_distance(self, node_a: FieldNode, node_b: FieldNode) -> int:
        dist_sq = self._distance_squared(node_a, node_b)
        return F_sqrt(to_fixed_int(dist_sq))
    
    def _compute_field_influence(self, source: FieldNode, target: FieldNode) -> int:
        dist_sq = self._distance_squared(source, target)
        if dist_sq > self.field_radius * self.field_radius:
            return ZERO
        
        strength = source.authority_strength
        
        if dist_sq > 0:
            # Inverse square law in fixed-point
            inv_dist = F_div(to_fixed_int(1), self._fixed_distance(source, target))
            decay = F_mul(inv_dist, inv_dist)
        else:
            decay = ONE
        
        entropy_penalty = F_sub(ONE, F_mul(source.entropy_level, HALF))
        
        influence = F_mul(strength, decay)
        influence = F_mul(influence, entropy_penalty)
        influence = F_mul(influence, self.attenuation_factor)
        
        return influence
    
    def schedule_propagation(self, from_node_id: str, to_node_id: str, delay_ticks: int = 3):
        source = self.nodes.get(from_node_id)
        target = self.nodes.get(to_node_id)
        if not source or not target:
            return
        
        influence = self._compute_field_influence(source, target)
        if influence == ZERO:
            return
        
        event = PropagationEvent(
            from_node=from_node_id,
            to_node=to_node_id,
            influence=influence,
            arrival_tick=self.current_tick + delay_ticks
        )
        self.propagation_queue.append(event)
    
    def process_propagation_events(self) -> Dict[str, int]:
        """
        Process all due propagation events.
        Returns updated legitimacy values.
        """
        results = {}
        
        # Process events whose arrival time has come
        remaining = deque()
        while self.propagation_queue:
            event = self.propagation_queue.popleft()
            if event.arrival_tick <= self.current_tick and not event.processed:
                if event.to_node in self.next_buffer:
                    # Conserve total field energy - transfer, not create
                    if self.next_buffer[event.to_node] >= event.influence:
                        self.next_buffer[event.to_node] = F_sub(self.next_buffer[event.to_node], event.influence)
                    results[event.to_node] = self.next_buffer[event.to_node]
                event.processed = True
            else:
                remaining.append(event)
        
        self.propagation_queue = remaining
        return results
    
    def compute_local_legitimacy(self, node_id: str) -> int:
        """Compute legitimacy from local field only (no global propagation)."""
        target = self.nodes.get(node_id)
        if not target:
            return ZERO
        
        total_field = ZERO
        for source in self.nodes.values():
            if source.node_id == node_id:
                continue
            
            influence = self._compute_field_influence(source, target)
            if influence > ZERO:
                total_field = F_add(total_field, influence)
        
        # Add self-legitimacy from buffer
        self_legitimacy = self.current_buffer.get(node_id, ZERO)
        total_field = F_add(total_field, self_legitimacy)
        
        return F_min(ONE, total_field)
    
    def double_buffer_update(self) -> Dict[str, int]:
        """
        Double-buffered field update. All computes from current buffer,
        all writes go to next buffer. Order-independent.
        """
        # Compute all updates from current buffer
        for node_id in self.nodes:
            new_val = self.compute_local_legitimacy(node_id)
            self.next_buffer[node_id] = new_val
        
        # Compute total energy change (should be conserved)
        new_total = ZERO
        for v in self.next_buffer.values():
            new_total = F_add(new_total, v)
        
        # Normalize to conserve total field energy if needed
        if new_total != self.total_field_energy and new_total > ZERO:
            ratio = F_div(self.total_field_energy, new_total)
            for node_id in self.next_buffer:
                self.next_buffer[node_id] = F_mul(self.next_buffer[node_id], ratio)
        
        # Swap buffers
        self.current_buffer, self.next_buffer = self.next_buffer, self.current_buffer
        self.next_buffer = {k: v for k, v in self.current_buffer.items()}
        
        # Update total energy
        self.total_field_energy = self._compute_total_energy()
        
        return dict(self.current_buffer)
    
    def _compute_total_energy(self) -> int:
        total = ZERO
        for v in self.current_buffer.values():
            total = F_add(total, v)
        return total
    
    def step(self, ticks: int = 1) -> Dict[str, int]:
        """Advance the field by multiple ticks."""
        results = {}
        for _ in range(ticks):
            self.current_tick += 1
            self.process_propagation_events()
            step_results = self.double_buffer_update()
            results.update(step_results)
        return results
    
    def get_legitimacy(self, node_id: str) -> int:
        return self.current_buffer.get(node_id, ZERO)
    
    def schedule_all_propagations(self, delay_ticks: int = 3):
        """Schedule propagation from all nodes to all others."""
        for source_id in self.nodes:
            for target_id in self.nodes:
                if source_id != target_id:
                    self.schedule_propagation(source_id, target_id, delay_ticks)
    
    def get_topology_stats(self) -> dict:
        return {
            "total_nodes": len(self.nodes),
            "field_radius": self.field_radius,
            "total_field_energy": from_fixed(self.total_field_energy),
            "pending_events": len(self.propagation_queue),
            "current_tick": self.current_tick,
            "average_legitimacy": from_fixed(self._compute_total_energy() // max(1, len(self.nodes)))
        }
