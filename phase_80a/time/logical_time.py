from dataclasses import dataclass
from typing import Dict, Optional

@dataclass
class ConstitutionalTick:
    epoch: int
    block_height: int
    tick_number: int
    parent_tick_hash: Optional[str] = None

class LogicalTimeManager:
    def __init__(self):
        self.current_epoch = 0
        self.current_tick = 0
        self.tick_history: Dict[int, ConstitutionalTick] = {}
    
    def advance_tick(self, block_height: int, parent_hash: Optional[str] = None) -> ConstitutionalTick:
        self.current_tick += 1
        
        tick = ConstitutionalTick(
            epoch=self.current_epoch,
            block_height=block_height,
            tick_number=self.current_tick,
            parent_tick_hash=parent_hash
        )
        
        self.tick_history[self.current_tick] = tick
        return tick
    
    def advance_epoch(self) -> int:
        self.current_epoch += 1
        self.current_tick = 0
        return self.current_epoch
    
    def get_current_tick(self) -> ConstitutionalTick:
        return self.tick_history.get(self.current_tick, ConstitutionalTick(
            epoch=self.current_epoch,
            block_height=0,
            tick_number=self.current_tick
        ))
    
    def get_temporal_distance(self, tick_a: int, tick_b: int) -> int:
        return abs(tick_a - tick_b)

class ConstitutionalTimer:
    def __init__(self, time_manager: LogicalTimeManager):
        self.time_manager = time_manager
        self.timers: Dict[str, int] = {}
    
    def set_timer(self, timer_id: str, duration_ticks: int):
        current_tick = self.time_manager.current_tick
        self.timers[timer_id] = current_tick + duration_ticks
    
    def is_expired(self, timer_id: str) -> bool:
        current_tick = self.time_manager.current_tick
        expiry_tick = self.timers.get(timer_id, 0)
        return current_tick >= expiry_tick
    
    def get_remaining_ticks(self, timer_id: str) -> int:
        current_tick = self.time_manager.current_tick
        expiry_tick = self.timers.get(timer_id, 0)
        return max(0, expiry_tick - current_tick)
    
    def clear_timer(self, timer_id: str):
        if timer_id in self.timers:
            del self.timers[timer_id]
