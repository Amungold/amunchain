from typing import List, Dict, Optional
from .dependency_graph import ConstitutionalDependencyGraph

class SemanticReplayEngine:
    def __init__(self, graph: ConstitutionalDependencyGraph):
        self.graph = graph
        self.timeline: List[Dict] = []
    
    def build_timeline(self, node_id: str) -> List[Dict]:
        lineage = self.graph.trace_lineage(node_id)
        timeline = []
        
        for nid in lineage:
            node = self.graph.nodes.get(nid)
            if node:
                timeline.append({
                    "node_id": nid,
                    "timestamp": node.created_at,
                    "aliveness": node.aliveness.value,
                    "constraint_influence": node.constraint_influence,
                    "semantic_weight": node.semantic_weight,
                    "participation_activity": node.participation_activity,
                    "metabolic_rate": node.metabolic_rate,
                    "constraint_dependency": node.constraint_dependency,
                    "effective_authority": node.effective_constraint_authority
                })
        
        self.timeline = timeline
        return timeline
    
    def detect_drift_onset(self, node_id: str) -> Optional[Dict]:
        timeline = self.build_timeline(node_id)
        if len(timeline) < 2:
            return None
        
        for i in range(1, len(timeline)):
            prev = timeline[i-1]
            curr = timeline[i]
            
            if curr["constraint_influence"] < prev["constraint_influence"] * 0.9:
                return {
                    "onset_node": curr["node_id"],
                    "onset_time": curr["timestamp"],
                    "previous_influence": prev["constraint_influence"],
                    "current_influence": curr["constraint_influence"],
                    "drop_percentage": (1 - curr["constraint_influence"] / prev["constraint_influence"]) * 100
                }
        return None
    
    def detect_authority_migration_time(self, node_id: str) -> Optional[Dict]:
        timeline = self.build_timeline(node_id)
        origin_id = self.graph._find_origin(node_id)
        
        for step in timeline:
            if origin_id not in step["constraint_dependency"]:
                return {
                    "migration_node": step["node_id"],
                    "migration_time": step["timestamp"],
                    "new_authority": step["effective_authority"],
                    "origin_influence": step["constraint_influence"]
                }
        return None
    
    def compute_drift_velocity(self, node_id: str) -> float:
        timeline = self.build_timeline(node_id)
        if len(timeline) < 2:
            return 0.0
        
        start_influence = timeline[0]["constraint_influence"]
        end_influence = timeline[-1]["constraint_influence"]
        steps = len(timeline) - 1
        
        if steps == 0:
            return 0.0
        
        return (start_influence - end_influence) / steps
    
    def get_semantic_health_report(self, node_id: str) -> Dict:
        timeline = self.build_timeline(node_id)
        
        return {
            "node_id": node_id,
            "total_steps": len(timeline),
            "initial_influence": timeline[0]["constraint_influence"] if timeline else None,
            "final_influence": timeline[-1]["constraint_influence"] if timeline else None,
            "drift_velocity": self.compute_drift_velocity(node_id),
            "drift_onset": self.detect_drift_onset(node_id),
            "authority_migration": self.detect_authority_migration_time(node_id),
            "is_suffocated": timeline[-1]["constraint_influence"] < 0.1 if timeline else False
        }
    
    def replay_from_checkpoint(self, snapshot) -> List[Dict]:
        return self.build_timeline(snapshot.node_id) if hasattr(snapshot, 'node_id') else []
