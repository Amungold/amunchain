from .core.axioms import ConstitutionalSemanticAxioms
from .runtime.dependency_graph import ConstitutionalDependencyGraph
from .runtime.replay_engine import SemanticReplayEngine
from .runtime.semantic_entropy_monitor import SemanticEntropyMonitor
from .runtime.admissibility_engine import AdmissibilityEngine
from .consensus.semantic_gate import SemanticConsensusGate
from .data.corpus import AmunChainCorpus

def main():
    print("\n" + "=" * 60)
    print("PHASE 80A - Constitutional Semantic Runtime")
    print("=" * 60)
    
    axioms = ConstitutionalSemanticAxioms.list_all()
    print(f"\nLoaded {len(axioms)} axioms")
    
    print("\nBuilding Constitutional Dependency Graph...")
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
    
    graph.print_report()
    
    print("\n" + "=" * 60)
    print("Semantic Replay Analysis")
    print("=" * 60)
    
    replay = SemanticReplayEngine(graph)
    health = replay.get_semantic_health_report("I4")
    print(f"\nI4 Health:")
    print(f"  Drift velocity: {health['drift_velocity']:.3f}")
    print(f"  Suffocated: {health['is_suffocated']}")
    print(f"  Total steps: {health['total_steps']}")
    if health['drift_onset']:
        print(f"  Drift onset: {health['drift_onset']['drop_percentage']:.1f}% drop")
    
    print("\n" + "=" * 60)
    print("Authority Migration Detection")
    print("=" * 60)
    
    gate = SemanticConsensusGate(graph)
    for nid in ["origin_001", "I1", "I2", "I3", "I4"]:
        migration = graph.detect_authority_migration(nid)
        if migration and migration.get('migration_detected'):
            print(f"{nid}: Authority migrated to {migration['migrated_to']}")
        
        weight = gate.get_consensus_weight(nid)
        print(f"{nid}: Consensus weight = {weight:.2f}")
    
    print("\n" + "=" * 60)
    print("Snapshot Creation")
    print("=" * 60)
    
    snapshot = gate.create_state_snapshot(1)
    print(f"Snapshot ID: {snapshot.snapshot_id[:16]}...")
    print(f"Merkle root: {snapshot.merkle_root[:16]}...")
    print(f"Node count: {snapshot.node_count}")
    
    print("\n" + "=" * 60)
    print("PHASE 80A OPERATIONAL")
    print("=" * 60)

if __name__ == "__main__":
    main()
