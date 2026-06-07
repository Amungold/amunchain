#!/bin/bash
set -euo pipefail

# ============================================================
# Constitutional Release Script v1
# ============================================================

TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)

VERSION=$(
grep '^version' crates/amun-constitutional/Cargo.toml \
| head -1 \
| cut -d'"' -f2
)

if [ -z "${VERSION}" ]; then
    echo "FAILED: unable to determine version"
    exit 1
fi

RELEASE_DIR="releases/amunchain-${VERSION}"

echo ""
echo "=================================================="
echo "AMUNCHAIN CONSTITUTIONAL RELEASE"
echo "=================================================="
echo "Version: ${VERSION}"
echo "Timestamp: ${TIMESTAMP}"
echo ""

echo "[1/8] Constitutional verification"
cargo check --workspace --all-targets
echo "PASS: cargo check"

echo "[2/8] Full test suite"
cargo test --workspace
echo "PASS: tests"

echo "[3/8] License audit"
cargo deny check licenses
echo "PASS: licenses"

echo "[4/8] Dependency audit"
cargo udeps --workspace
echo "PASS: udeps"

echo "[5/8] Deterministic release build"
cargo build --release --locked
echo "PASS: release build"

echo "[6/8] Generating specification hash"

SPEC_HASH=$(
find crates docs constitution \
-type f \
| sort \
| xargs cat \
| sha256sum \
| cut -d' ' -f1
)

echo "Specification Hash: ${SPEC_HASH}"

echo "[7/8] Creating release artifacts"

mkdir -p "${RELEASE_DIR}"

cat > "${RELEASE_DIR}/RELEASE_MANIFEST.txt" << MANIFEST
AMUNCHAIN CONSTITUTIONAL RELEASE

Version: ${VERSION}
Timestamp: ${TIMESTAMP}

Specification Hash:
${SPEC_HASH}

Verification:
- cargo check: PASSED
- tests: PASSED
- licenses: PASSED
- udeps: PASSED
- release build: PASSED
MANIFEST

cat > "${RELEASE_DIR}/FREEZE_CERTIFICATE.txt" << FREEZE
FREEZE CERTIFICATE V1

Civilization: AmunChain
Version: ${VERSION}

Frozen Properties:
- serialization
- replay semantics
- constitutional invariants
- domain separation
- proof structure
FREEZE

(
cd "${RELEASE_DIR}"
find . -type f ! -name SHA256SUMS | sort | xargs sha256sum > SHA256SUMS
)

echo "[8/8] Release complete"

echo ""
echo "=================================================="
echo "RELEASE COMPLETE"
echo "=================================================="
echo "Release directory: ${RELEASE_DIR}"
echo "Specification hash: ${SPEC_HASH}"
echo ""

ls -la "${RELEASE_DIR}"
