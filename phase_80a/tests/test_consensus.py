from ..runtime.dependency_graph import ConstitutionalDependencyGraph
from ..consensus.semantic_gate import SemanticConsensusGate
from ..consensus.validator import ConstitutionalValidator
from ..data.corpus import AmunChainCorpus

def test_semantic_gate():
    print("\nTEST: Semantic Consensus Gate")
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
    
    gate = SemanticConsensusGate(graph)
    
    valid, reasons = gate.validate_for_consensus("origin_001")
    if valid:
        print("  Origin validation: PASSED")
    else:
        print(f"  Origin validation: FAILED - {reasons}")
        return False
    
    valid, reasons = gate.validate_for_consensus("I4")
    if not valid:
        print(f"  I4 rejection: PASSED - {reasons}")
    else:
        print("  I4 should be rejected but was accepted")
        return False
    
    weight = gate.get_consensus_weight("origin_001")
    print(f"  Origin weight: {weight:.2f}")
    
    print("\nPASSED: Semantic gate tests")
    return True

def test_validator():
    print("\nTEST: Constitutional Validator")
    print("-" * 40)
    
    graph = ConstitutionalDependencyGraph()
    origin = AmunChainCorpus.origin()
    graph.add_node(origin)
    
    gate = SemanticConsensusGate(graph)
    validator = ConstitutionalValidator("validator_001", gate)
    
    status = validator.report_status()
    print(f"  Validator: {status['validator_id']}, role={status['role']}")
    
    result = validator.validate_interpretation(origin)
    if result['valid']:
        print("  Interpretation validation: PASSED")
    else:
        print(f"  Interpretation validation: FAILED - {result['reason']}")
        return False
    
    print("\nPASSED: Validator tests")
    return True

if __name__ == "__main__":
    test_semantic_gate()
    test_validator()
