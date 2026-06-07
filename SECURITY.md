# AmunChain — Security Policy

## Reporting a Vulnerability

AmunChain treats security vulnerabilities as constitutional threats.

If you discover a security vulnerability, please **do not** open a
public issue. Instead, report it privately to:

**security@amungold.global**

We will respond within 72 hours with an acknowledgment and a timeline
for resolution.

## Scope

The following are in scope for security reports:

- Cryptographic flaws in the constitutional hashing or signing layers
- Determinism violations in the replay engine
- Sparse Merkle Tree proof forgery or bypass
- Capability enforcement bypass in the execution kernel
- Block hash collision or preimage attacks
- Lineage forgery or genesis identity spoofing
- Remote code execution via untrusted input vectors

## Out of Scope

- Social engineering attacks
- Physical security of nodes
- Denial of service via resource exhaustion (unless it violates
  constitutional invariants)
- Issues in dependencies that have not been confirmed exploitable
  within AmunChain

## Disclosure Policy

- Vulnerability reports will be acknowledged within 72 hours.
- A fix will be developed and tested within 30 days for critical issues.
- A public advisory will be published after the fix is released.
- Credit will be given to the reporter unless anonymity is requested.

## Constitutional Invariants

AmunChain is built on constitutional invariants that must never be
violated. Any vulnerability that allows:

- Forging a constitutional witness
- Bypassing capability enforcement
- Producing non-deterministic execution
- Breaking replay equivalence
- Spoofing genesis identity

is considered **critical** and will be treated with the highest priority.

---

Copyright (c) 2026 Amungold Global
