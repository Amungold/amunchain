from ..runtime.dependency_graph import ConstitutionalDependencyGraph
from ..data.corpus import AmunChainCorpus
from ..core.enums import AlivenessState, DerivationType, DependencyType
from ..models.interpretive_node import InterpretiveNode

def test_suffocation():
    print("\nTEST: Semantic Suffocation Detection")
    print("-" * 40)
    
    graph = ConstitutionalDependencyGraph()
    origin = AmunChainCorpus.origin()
    i1 = AmunChainCorpus.translation(parent_node=origin)
    i2 = AmunChainCorpus.extension(parent_node=i1)
    i3 = AmunChainCorpus.reframing(parent_node=i2)
    i4 = AmunChainCorpus.drift(parent_node=i3)
    
    graph.add_node(origin)
    graph.add_node(i1)
    graph.add_node(i2)
    graph.add_node(i3)
    graph.add_node(i4)
    
    status = graph.get_status("I4")
    if status and "origin_001" not in status['constraint_dependency']:
        print("PASSED: Semantic Suffocation detected")
        return True
    
    print("FAILED: Suffocation not detected")
    return False

def test_necromancy():
    print("\nTEST: Necromancy Prevention")
    print("-" * 40)
    
    graph = ConstitutionalDependencyGraph()
    origin = AmunChainCorpus.origin()
    i1 = AmunChainCorpus.translation(parent_node=origin)
    i2 = AmunChainCorpus.extension(parent_node=i1)
    i3 = AmunChainCorpus.reframing(parent_node=i2)
    i4 = AmunChainCorpus.drift(parent_node=i3)
    
    graph.add_node(origin)
    graph.add_node(i1)
    graph.add_node(i2)
    graph.add_node(i3)
    graph.add_node(i4)
    
    graph.nodes["I4"].aliveness = AlivenessState.DEAD
    
    i5 = InterpretiveNode.create(
        node_id="I5",
        semantic_origin_hash=AmunChainCorpus.ORIGIN_HASH,
        parent_id="I4",
        source_text="Derivation from dead node",
        derivation_type=DerivationType.DRIFT,
        contextual_modifiers=[],
        semantic_invariants=AmunChainCorpus.ORIGIN_INVARIANTS.copy(),
        constraint_dependency=["I4"],
        constraint_dependency_types=[DependencyType.DERIVED],
        aliveness=AlivenessState.CRITICAL,
        parent_node=i4
    )
    
    success, violations = graph.add_node(i5)
    if not success:
        print("PASSED: Necromancy blocked")
        return True
    
    print("FAILED: Necromancy not blocked")
    return False

if __name__ == "__main__":
    test_suffocation()
    test_necromancy()
