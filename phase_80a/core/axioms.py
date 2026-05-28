from typing import Dict, List

class ConstitutionalSemanticAxioms:
    AXIOMS = {
        "CSA-1": {
            "name": "Semantic Attributability",
            "statement": "All constitutional meaning must remain attributable to an identifiable semantic origin.",
            "invalidating_condition": "A semantic origin becomes unidentifiable or an interpretation exists with no attributable ancestry."
        },
        "CSA-2": {
            "name": "Interpretive Lineage Preservation",
            "statement": "Interpretation must preserve a complete, inspectable lineage to its semantic ancestry.",
            "invalidating_condition": "An interpretation has no interpretive_parent but is not a primordial origin, or the lineage chain has a missing link."
        },
        "CSA-3": {
            "name": "Contextual Extension Non-Erasure",
            "statement": "Context may extend constitutional meaning, but must never erase or replace the semantic origin.",
            "invalidating_condition": "A contextual_modifier removes or overwrites origin semantics, or original meaning becomes unrecoverable."
        },
        "CSA-4": {
            "name": "Drift Inspectability",
            "statement": "Drift accumulation must remain inspectable at every point along the interpretive lineage.",
            "invalidating_condition": "Drift occurs without being recorded, or semantic_delta becomes unmeasurable at any lineage point."
        },
        "CSA-5": {
            "name": "Semantic Continuity Integrity",
            "statement": "Semantic continuity is constitutionally invalidated when interpretive evolution can no longer be derivationally attributed to its constitutional origin.",
            "invalidating_condition": "An interpretation claims constitutional authority but cannot be derivationally attributed to origin."
        },
        "CSA-6": {
            "name": "Semantic Conservation",
            "statement": "Interpretive evolution must preserve the constitutional semantic invariants of its origin, not only its lineage.",
            "invalidating_condition": "Any core semantic invariant is lost, even with full lineage intact."
        },
        "CSA-7": {
            "name": "Semantic Authenticity",
            "statement": "Interpretive lineage must remain derivationally authentic and must never be synthetically fabricated.",
            "invalidating_condition": "A lineage is artificially reconstructed without lawful semantic evolution."
        },
        "CSA-8": {
            "name": "Semantic Aliveness",
            "statement": "Constitutional meaning must remain semantically alive across interpretive evolution.",
            "invalidating_condition": "Interpretive lineage remains structurally intact while constitutional semantic influence collapses."
        }
    }
    
    @classmethod
    def list_all(cls) -> Dict:
        return cls.AXIOMS
    
    @classmethod
    def get_axiom(cls, axiom_id: str) -> Dict:
        return cls.AXIOMS.get(axiom_id, {})
    
    @classmethod
    def validate(cls, node, context: Dict) -> List[str]:
        violations = []
        
        if not hasattr(node, 'semantic_origin_hash') or not node.semantic_origin_hash:
            violations.append("CSA-1: No semantic origin")
        
        if hasattr(node, 'parent_id') and node.parent_id:
            if not hasattr(node, 'lineage_status') or node.lineage_status != "complete":
                violations.append("CSA-2: Incomplete lineage")
        
        if context.get('is_replacement', False):
            violations.append("CSA-3: Context replaced origin")
        
        if context.get('lost_invariants'):
            violations.append(f"CSA-6: Lost invariants: {context['lost_invariants']}")
        
        if hasattr(node, 'aliveness') and node.aliveness.value == "dead":
            if context.get('is_used_for_derivation', False):
                violations.append("CSA-8: Semantic necromancy")
        
        return violations
