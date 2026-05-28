.PHONY: all check test audit clean

all: check test

# ============================================================
# Quick Checks
# ============================================================
check:
	cargo check --workspace --all-targets --all-features

clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all
	cargo fmt --all --check

# ============================================================
# Testing
# ============================================================
test:
	cargo test --workspace --all-features

test-release:
	cargo test --workspace --all-features --release

miri:
	bash scripts/miri-test.sh

# ============================================================
# Audit
# ============================================================
audit:
	cargo audit

deny:
	cargo deny check

udeps:
	cargo +nightly udeps --workspace --all-targets

outdated:
	cargo outdated

# ============================================================
# Full Audit Pipeline
# ============================================================
audit-full: fmt clippy test audit deny udeps
	@echo "✅ Full audit complete"

# ============================================================
# Cleanup
# ============================================================
clean:
	cargo clean

doc:
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
