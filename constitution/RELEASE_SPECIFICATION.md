# Release Specification v1

## Constitutional Release Authority

A release is constitutionally valid if and only if all mandatory verification layers succeed.

Mandatory requirements:

- all tests pass
- all audit layers pass
- cargo deny reports zero violations
- cargo udeps reports zero unused dependencies
- the build is reproducible
- the specification hash matches the canonical constitutional state
- no constitutional invariant is violated

A release artifact is considered part of the constitutional record.

Any modification to constitutional semantics changes the constitutional identity of the civilization.

---

## Release Artifacts

Every constitutional release MUST generate the following artifacts:

1. Release manifest
2. Specification hash
3. Freeze certificate
4. Reproducibility proof
5. Lineage continuity proof
6. Artifact checksums
7. Build environment snapshot

Artifacts become immutable historical constitutional records.

---

## Versioning Model

AmunChain uses constitutional semantic versioning.

Format:

MAJOR.MINOR.PATCH

Rules:

- MAJOR:
  constitutional identity changes
  specification hash changes
  replay equivalence changes

- MINOR:
  backward compatible constitutional extensions

- PATCH:
  bug fixes without constitutional impact

---

## Frozen Release Profile

```toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
opt-level = "s"

The release profile itself is considered part of the reproducibility surface.

---

Constitutional Freeze

The following properties are frozen:

- canonical serialization
- domain separation
- proof semantics
- replay semantics
- endianness
- constitutional invariants
- SMT depth
- constitutional hash semantics

Changing frozen rules creates a new constitutional civilization.

---

Signing Authority

Release manifests must be signed by constitutional authority keys.

Unsigned releases are informational artifacts only and are not constitutional releases.

---

Reproducibility

A release must be reproducible from:

- Cargo.lock
- frozen release profile
- constitutional specifications
- canonical dependency graph
- deterministic toolchain

Independent validators must be able to reconstruct identical release artifacts.

---

Constitutional Identity

The specification hash defines the constitutional identity of the chain.

If the specification hash changes:

- replay equivalence changes
- constitutional identity changes
- lineage continuity must be re-established

---

Historical Permanence

Release artifacts are permanent constitutional records.

Historical releases must remain:

- verifiable
- replayable
- reproducible
- cryptographically attributable

