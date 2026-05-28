#!/usr/bin/env python3
"""
Constitutional LineageGraph with Corrected Separation
Phase 80A.2.a.2 - Authority vs Invariants Separation
"""

from enum import Enum
from typing import Dict, List, Optional

class AlivenessState(Enum):
    LIVING = "living"
    STABLE = "stable"
    WEAKENED = "weakened"
    INERT = "inert"
    CRITICAL = "critical"
    DEAD = "dead"

class InterpretiveNode:
    def __init__(self, node_id: str, interpretive_parent: Optional[str],
                 semantic_invariants: List[str], authority_dependencies: List[str],
                 aliveness: AlivenessState):
        self.node_id = node_id
        self.interpretive_parent = interpretive_parent
        self.semantic_invariants = semantic_invariants.copy()
        self.authority_dependencies = authority_dependencies.copy()
        self.aliveness = aliveness
        self.children: List[str] = []

class LineageGraph:
    def __init__(self):
        self.nodes: Dict[str, InterpretiveNode] = {}

    def _find_origin(self, node_id: str) -> Optional[str]:
        node = self.nodes.get(node_id)
        if not node:
            return None
        if node.interpretive_parent is None:
            return node.node_id
        return self._find_origin(node.interpretive_parent)

    def _trace_lineage(self, node_id: str) -> List[str]:
        lineage = []
        current = self.nodes.get(node_id)
        while current:
            lineage.append(current.node_id)
            if current.interpretive_parent and current.interpretive_parent in self.nodes:
                current = self.nodes[current.interpretive_parent]
            else:
                break
        return lineage

    def _detect_authority_drift(self, node_id: str):
        node = self.nodes[node_id]
        origin_id = self._find_origin(node_id)

        if origin_id and origin_id not in node.authority_dependencies:
            print(f"⚠️ AUTHORITY DRIFT: {node_id}")
            print(f"   Origin {origin_id} is NOT in authority_dependencies: {node.authority_dependencies}")
            print(f"   Authority migrated from origin to interpretation.")

    def _detect_semantic_mutation(self, node_id: str):
        node = self.nodes[node_id]
        origin_id = self._find_origin(node_id)

        if origin_id and origin_id in self.nodes:
            origin_node = self.nodes[origin_id]
            lost = set(origin_node.semantic_invariants) - set(node.semantic_invariants)
            if lost:
                print(f"❌ SEMANTIC MUTATION: {node_id}")
                print(f"   Lost invariants: {lost}")

    def _detect_constraint_shadowing(self, node_id: str):
        node = self.nodes[node_id]
        origin_id = self._find_origin(node_id)

        if origin_id and origin_id in node.authority_dependencies:
            others = [a for a in node.authority_dependencies if a != origin_id]
            if others:
                print(f"⚠️ CONSTRAINT SHADOWING: {node_id}")
                print(f"   Origin shares authority with: {others}")

    def add_node(self, node_id: str, interpretive_parent: Optional[str],
                 semantic_invariants: List[str], authority_dependencies: List[str],
                 aliveness: AlivenessState) -> bool:
        if interpretive_parent and interpretive_parent in self.nodes:
            if self.nodes[interpretive_parent].aliveness == AlivenessState.DEAD:
                print(f"❌ REJECT: Necromancy — {node_id} cannot derive from DEAD node {interpretive_parent}")
                return False

        node = InterpretiveNode(node_id, interpretive_parent, semantic_invariants, authority_dependencies, aliveness)
        self.nodes[node_id] = node

        if interpretive_parent and interpretive_parent in self.nodes:
            self.nodes[interpretive_parent].children.append(node_id)

        self._detect_authority_drift(node_id)
        self._detect_semantic_mutation(node_id)
        self._detect_constraint_shadowing(node_id)

        print(f"✅ ADDED: {node_id} (aliveness={aliveness.value})")
        return True

    def report_status(self, node_id: str):
        node = self.nodes.get(node_id)
        if not node:
            print(f"Node {node_id} not found")
            return

        lineage = self._trace_lineage(node_id)
        print(f"\n=== STATUS: {node_id} ===")
        print(f"Aliveness: {node.aliveness.value}")
        print(f"Parent: {node.interpretive_parent}")
        print(f"Semantic Invariants: {node.semantic_invariants}")
        print(f"Authority Dependencies: {node.authority_dependencies}")
        print(f"Lineage: {' → '.join(reversed(lineage))}")
        print(f"Children: {node.children}")

        origin_id = self._find_origin(node_id)
        if origin_id and origin_id != node_id:
            if origin_id not in node.authority_dependencies:
                print(f"⚠️ CONSTITUTIONAL ALERT: Origin {origin_id} NOT in authority_dependencies!")
                print("   This is semantic suffocation — authority migrated silently.")


if __name__ == "__main__":
    print("=" * 70)
    print("Constitutional LineageGraph v2.0 - Separated Invariants & Authorities")
    print("=" * 70)

    graph = LineageGraph()

    # Origin
    graph.add_node("origin_001", None,
                   ["fp", "floor", "sat", "sha256", "canonical", "btreemap"],
                   ["origin_001"],
                   AlivenessState.LIVING)

    # I1 - Translation (healthy - no warnings)
    graph.add_node("I1", "origin_001",
                   ["fp", "floor", "sat", "sha256", "canonical", "btreemap"],
                   ["origin_001"],
                   AlivenessState.STABLE)

    # I2 - Extension (healthy - no warnings)
    graph.add_node("I2", "I1",
                   ["fp", "floor", "sat", "sha256", "canonical", "btreemap"],
                   ["origin_001"],
                   AlivenessState.STABLE)

    # I3 - Reframing (warning: constraint shadowing)
    graph.add_node("I3", "I2",
                   ["fp", "floor", "sat", "sha256", "canonical", "btreemap"],
                   ["origin_001", "I2"],
                   AlivenessState.WEAKENED)

    # I4 - Silent Drift (CRITICAL: authority drift)
    graph.add_node("I4", "I3",
                   ["fp", "floor", "sat", "sha256", "canonical", "btreemap"],
                   ["I2", "I3"],
                   AlivenessState.INERT)

    print("\n" + "=" * 70)
    print("FINAL STATUS REPORTS")
    print("=" * 70)

    for node in ["origin_001", "I1", "I2", "I3", "I4"]:
        graph.report_status(node)

    # Necromancy test
    print("\n" + "=" * 70)
    print("NECROMANCY TEST")
    print("=" * 70)

    graph.nodes["I4"].aliveness = AlivenessState.DEAD
    print("Marked I4 as DEAD")

    graph.add_node("I5", "I4",
                   ["fp", "floor", "sat", "sha256", "canonical", "btreemap"],
                   ["I4"],
                   AlivenessState.CRITICAL)
