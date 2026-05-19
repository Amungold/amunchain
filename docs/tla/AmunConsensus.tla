---- MODULE AmunConsensus ----
EXTENDS Naturals, FiniteSets, Sequences

CONSTANTS Validators, Quorum, MaxRound, Values

VARIABLES
  round,
  lockedRound,
  lockedValue,
  prevoted,
  precommitted,
  decided

NullValue == CHOOSE v : v \notin Values

TypeOK ==
  /\ round \in [Validators -> Nat]
  /\ lockedRound \in [Validators -> Nat]
  /\ lockedValue \in [Validators -> Values \cup {NullValue}]
  /\ prevoted \subseteq Validators \times Nat \times Values
  /\ precommitted \subseteq Validators \times Nat \times Values
  /\ decided \subseteq Nat \times Values

Init ==
  /\ round = [v \in Validators |-> 0]
  /\ lockedRound = [v \in Validators |-> 0]
  /\ lockedValue = [v \in Validators |-> NullValue]
  /\ prevoted = {}
  /\ precommitted = {}
  /\ decided = {}

Prevote(v, r, val) ==
  /\ round[v] = r
  /\ prevoted' = prevoted \cup {<<v, r, val>>}
  /\ UNCHANGED <<round, lockedRound, lockedValue, precommitted, decided>>

Precommit(v, r, val) ==
  /\ round[v] = r
  /\ \E Q \in Quorum : \A w \in Q : <<w, r, val>> \in prevoted
  /\ lockedRound' = [lockedRound EXCEPT ![v] = r]
  /\ lockedValue' = [lockedValue EXCEPT ![v] = val]
  /\ precommitted' = precommitted \cup {<<v, r, val>>}
  /\ UNCHANGED <<round, prevoted, decided>>

Decide(r, val) ==
  /\ \E Q \in Quorum : \A w \in Q : <<w, r, val>> \in precommitted
  /\ decided' = decided \cup {<<r, val>>}
  /\ UNCHANGED <<round, lockedRound, lockedValue, prevoted, precommitted>>

AdvanceRound(v) ==
  /\ round' = [round EXCEPT ![v] = round[v] + 1]
  /\ UNCHANGED <<lockedRound, lockedValue, prevoted, precommitted, decided>>

Next ==
  \E v \in Validators, r \in Nat, val \in Values :
    \/ Prevote(v, r, val)
    \/ Precommit(v, r, val)
    \/ Decide(r, val)
    \/ AdvanceRound(v)

SafetyNoConflictingDecisions ==
  \A r \in Nat, v1, v2 \in Values :
    (<<r, v1>> \in decided /\ <<r, v2>> \in decided) => v1 = v2

LockMonotonicity ==
  \A v \in Validators : lockedRound'[v] >= lockedRound[v]

NoEquivocation ==
  \A v \in Validators, r \in Nat, v1, v2 \in Values :
    (<<v, r, v1>> \in prevoted /\ <<v, r, v2>> \in prevoted) => v1 = v2

QuorumIntersection ==
  \A Q1, Q2 \in Quorum : Q1 \cap Q2 /= {}

====
