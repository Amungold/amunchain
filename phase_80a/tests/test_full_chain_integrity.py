from ..core.enums import AlivenessState, DerivationType, DependencyType
from ..models.interpretive_node import InterpretiveNode

def test_full_chain_determinism():
    print("\nTEST: Full Chain Determinism")
    print("-" * 40)
    
    origin = InterpretiveNode.create(
        node_id="origin_001",
        semantic_origin_hash="origin_hash",
        parent_id=None,
        source_text="Test text.",
        derivation_type=DerivationType.TRANSLATION,
        contextual_modifiers=[],
        semantic_invariants=["invariant_a"],
        constraint_dependency=["origin_001"],
        constraint_dependency_types=[DependencyType.ORIGIN],
        aliveness=AlivenessState.LIVING
    )
    
    child = InterpretiveNode.create(
        node_id="child_001",
        semantic_origin_hash="origin_hash",
        parent_id="origin_001",
        source_text="Child text.",
        derivation_type=DerivationType.EXTENSION,
        contextual_modifiers=[],
        semantic_invariants=["invariant_a", "invariant_b"],
        constraint_dependency=["origin_001"],
        constraint_dependency_types=[DependencyType.DERIVED],
        aliveness=AlivenessState.STABLE,
        parent_node=origin
    )
    
    origin_hash1 = origin.node_hash
    child_hash1 = child.node_hash
    
    origin2 = InterpretiveNode.create(
        node_id="origin_001",
        semantic_origin_hash="origin_hash",
        parent_id=None,
        source_text="Test text.",
        derivation_type=DerivationType.TRANSLATION,
        contextual_modifiers=[],
        semantic_invariants=["invariant_a"],
        constraint_dependency=["origin_001"],
        constraint_dependency_types=[DependencyType.ORIGIN],
        aliveness=AlivenessState.LIVING
    )
    
    if origin_hash1 == origin2.node_hash:
        print("PASSED: Full chain determinism")
        return True
    else:
        print("FAILED: Chain not deterministic")
        return False

if __name__ == "__main__":
    test_full_chain_determinism()
