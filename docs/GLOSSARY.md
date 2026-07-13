# AmunChain Glossary

## Constitutional Terms

**Legal Owner:** The authority responsible for defining policy for a domain.

**Operational Owner:** The single runtime component authorized to mutate a domain.

**Reader:** A component allowed to observe a domain. Readers SHALL NEVER mutate data.

**Projection:** Persistent representation derived from authoritative runtime state. SHALL NEVER become authoritative.

**Runtime Cache:** Temporary performance optimization. SHALL NEVER become authoritative.

**Initialization Source:** Bootstrap information loaded during startup.

**Recovery Source:** Persistent data used to restore runtime state after restart.

**Constitutional Domain:** A bounded area of architectural responsibility with exactly one Operational Owner.

## Domain Terms

**Identity Domain:** Constitutional Domain owning validator identity certificates signing keys voting power view and authority registry cache.

**Consensus Domain:** Constitutional Domain owning canonical chain state current height current view QC state history root and commit decisions.

**Persistence Domain:** Constitutional Domain owning persistent storage for finalized blocks and recovery state.

**Session Domain:** Constitutional Domain owning authenticated peer sessions and secure transport binding.

**Governance:** The Legal Owner of voting power distribution and constitutional amendments.

**Authority Registry:** A runtime cache derived from the Identity Domain. Never an authoritative source.

**Canonical Chain State:** The authoritative runtime state of the blockchain including height history root QC state and current view.

---
Copyright 2026 AmunChain Constitutional Assembly
