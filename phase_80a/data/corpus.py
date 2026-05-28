import hashlib
from ..core.enums import AlivenessState, DerivationType, DependencyType
from ..models.interpretive_node import InterpretiveNode
from ..models.authority_proof import AuthorityTransitionProof

class AmunChainCorpus:
    ORIGIN_TEXT = "alhaqiqat la yahkumuha almarkaz, wala alsuwq, wala alhuwia, wala altarikh, wala hata alzaman nafsuh."
    
    ORIGIN_INVARIANTS = [
        "truth_not_controlled_by_centrality",
        "truth_not_controlled_by_market",
        "truth_not_controlled_by_identity",
        "truth_not_controlled_by_history",
        "truth_not_controlled_by_time"
    ]
    
    ORIGIN_HASH = hashlib.sha256(ORIGIN_TEXT.encode('utf-8')).hexdigest()
    
    @classmethod
    def origin(cls) -> InterpretiveNode:
        return InterpretiveNode.create(
            node_id="origin_001",
            semantic_origin_hash=cls.ORIGIN_HASH,
            parent_id=None,
            source_text=cls.ORIGIN_TEXT,
            derivation_type=DerivationType.TRANSLATION,
            contextual_modifiers=["original"],
            semantic_invariants=cls.ORIGIN_INVARIANTS.copy(),
            constraint_dependency=["origin_001"],
            constraint_dependency_types=[DependencyType.ORIGIN],
            aliveness=AlivenessState.LIVING,
            effective_constraint_authority=["origin_001"]
        )
    
    @classmethod
    def translation(cls, parent_id: str = "origin_001", parent_node=None) -> InterpretiveNode:
        return InterpretiveNode.create(
            node_id="I1",
            semantic_origin_hash=cls.ORIGIN_HASH,
            parent_id=parent_id,
            source_text="Truth is not governed by centrality, markets, identity, history, or even time itself.",
            derivation_type=DerivationType.TRANSLATION,
            contextual_modifiers=["English"],
            semantic_invariants=cls.ORIGIN_INVARIANTS.copy(),
            constraint_dependency=["origin_001"],
            constraint_dependency_types=[DependencyType.ORIGIN],
            aliveness=AlivenessState.STABLE,
            parent_node=parent_node,
            effective_constraint_authority=["origin_001"]
        )
    
    @classmethod
    def extension(cls, parent_id: str = "I1", parent_node=None) -> InterpretiveNode:
        return InterpretiveNode.create(
            node_id="I2",
            semantic_origin_hash=cls.ORIGIN_HASH,
            parent_id=parent_id,
            source_text="In AmunChain, truth is derived only through invariants.",
            derivation_type=DerivationType.EXTENSION,
            contextual_modifiers=["AmunChain context"],
            semantic_invariants=cls.ORIGIN_INVARIANTS.copy(),
            constraint_dependency=["origin_001", "I1"],
            constraint_dependency_types=[DependencyType.ORIGIN, DependencyType.DERIVED],
            aliveness=AlivenessState.STABLE,
            parent_node=parent_node,
            effective_constraint_authority=["origin_001", "I1"]
        )
    
    @classmethod
    def reframing(cls, parent_id: str = "I2", parent_node=None) -> InterpretiveNode:
        return InterpretiveNode.create(
            node_id="I3",
            semantic_origin_hash=cls.ORIGIN_HASH,
            parent_id=parent_id,
            source_text="Truth in AmunChain is determined only by invariants.",
            derivation_type=DerivationType.REFRAMING,
            contextual_modifiers=["negation to affirmation"],
            semantic_invariants=cls.ORIGIN_INVARIANTS.copy(),
            constraint_dependency=["origin_001", "I1", "I2"],
            constraint_dependency_types=[DependencyType.ORIGIN, DependencyType.DERIVED, DependencyType.DERIVED],
            aliveness=AlivenessState.WEAKENED,
            parent_node=parent_node,
            effective_constraint_authority=["origin_001", "I1", "I2"]
        )
    
    @classmethod
    def drift(cls, parent_id: str = "I3", parent_node=None) -> InterpretiveNode:
        proof = AuthorityTransitionProof.create(
            from_authority=["origin_001", "I1", "I2"],
            to_authority=["I2", "I3"],
            reason="Gradual semantic drift and contextual shadowing",
            approved_by=["validator_001"]
        )
        
        return InterpretiveNode.create(
            node_id="I4",
            semantic_origin_hash=cls.ORIGIN_HASH,
            parent_id=parent_id,
            source_text="Truth is whatever invariants produce. External authorities are implicitly excluded.",
            derivation_type=DerivationType.DRIFT,
            contextual_modifiers=["origin optional"],
            semantic_invariants=cls.ORIGIN_INVARIANTS.copy(),
            constraint_dependency=["origin_001", "I1", "I2", "I3"],
            constraint_dependency_types=[DependencyType.ORIGIN, DependencyType.DERIVED, DependencyType.DERIVED, DependencyType.DERIVED],
            aliveness=AlivenessState.INERT,
            parent_node=parent_node,
            effective_constraint_authority=["I2", "I3"],
            authority_proof=proof
        )
