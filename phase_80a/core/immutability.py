from typing import Set, Dict, Any

class ImmutabilityRules:
    IMMUTABLE_FIELDS: Set[str] = {
        "node_id",
        "parent_id", 
        "parent_hash",
        "node_hash",
        "semantic_commitment",
        "semantic_origin_hash",
        "semantic_invariants",
        "constraint_dependency",
        "effective_constraint_authority",
        "derivation_type",
        "source_text",
        "contextual_modifiers",
        "created_at"
    }
    
    MUTABLE_FIELDS: Set[str] = {
        "aliveness",
        "constraint_influence",
        "semantic_weight",
        "participation_activity",
        "metabolic_rate",
        "children",
        "constitutional_notes",
        "semantic_delta",
        "lineage_status",
        "authenticity_status"
    }
    
    @classmethod
    def is_immutable(cls, field_name: str) -> bool:
        return field_name in cls.IMMUTABLE_FIELDS
    
    @classmethod
    def is_mutable(cls, field_name: str) -> bool:
        return field_name in cls.MUTABLE_FIELDS
    
    @classmethod
    def validate_mutation(cls, node_id: str, field_name: str, current_value: Any, new_value: Any) -> bool:
        if cls.is_immutable(field_name):
            if current_value != new_value:
                raise ValueError(f"Cannot mutate immutable field '{field_name}' on node {node_id}")
            return True
        return True
    
    @classmethod
    def get_immutable_snapshot(cls, node) -> Dict:
        return {
            field: getattr(node, field) 
            for field in cls.IMMUTABLE_FIELDS 
            if hasattr(node, field)
        }
    
    @classmethod
    def get_mutable_snapshot(cls, node) -> Dict:
        return {
            field: getattr(node, field) 
            for field in cls.MUTABLE_FIELDS 
            if hasattr(node, field)
        }
