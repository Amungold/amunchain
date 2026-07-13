# Dependency Rules

## Layering (Top to Bottom)
- Layer 6 (Test) can depend on anything
- Layer 5 (Network) can depend on Layers 0-4
- Layer 4 (Storage) can depend on Layers 0-3
- Layer 3 (Execution) can depend on Layers 0-2
- Layer 2 (Constitution) can depend on Layers 0-1
- Layer 1 (Consensus) can depend on Layer 0
- Layer 0 (Core) cannot depend on anything

## Forbidden Dependencies
- Execution → Network (execution must not know about network)
- Storage → Constitution (storage must not know about constitution)
- Network → Consensus (network must not know about consensus details)

## Allowed Dependencies
- Core ← Crypto ← Consensus ← Constitution ← Execution ← Storage ← Network ← Test
