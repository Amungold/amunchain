# Build Reproducibility

## Overview

AmunChain requires reproducible builds.

Independent builders should produce identical constitutional artifacts.

---

# Toolchain

Required:

- Rust 1.85+
- cargo

---

# Locked Dependencies

Always build with:

```bash
cargo build --locked
```

---

# Frozen Dependency Graph

Cargo.lock is constitutional infrastructure.

Silent dependency drift is forbidden.

---

# Deterministic Release Profile

Recommended:

```toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

---

# Risks

Potential reproducibility risks:

- compiler drift
- dependency mutation
- feature drift
- environment leakage

---

# Constitutional Principle

Reproducible builds are constitutional infrastructure.
