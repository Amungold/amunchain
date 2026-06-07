#!/bin/bash
set -euo pipefail

RELEASE_DIR="${1:-}"

if [ -z "${RELEASE_DIR}" ]; then
    echo "Usage: $0 <release-directory>"
    exit 1
fi

if [ ! -d "${RELEASE_DIR}" ]; then
    echo "FAILED: release directory not found"
    exit 1
fi

echo ""
echo "=================================================="
echo "VERIFYING CONSTITUTIONAL RELEASE"
echo "=================================================="

if [ ! -f "${RELEASE_DIR}/RELEASE_MANIFEST.txt" ]; then
    echo "FAILED: missing RELEASE_MANIFEST.txt"
    exit 1
fi

echo "PASS: manifest found"

if [ ! -f "${RELEASE_DIR}/FREEZE_CERTIFICATE.txt" ]; then
    echo "FAILED: missing FREEZE_CERTIFICATE.txt"
    exit 1
fi

echo "PASS: freeze certificate found"

if [ ! -f "${RELEASE_DIR}/SHA256SUMS" ]; then
    echo "FAILED: missing SHA256SUMS"
    exit 1
fi

echo "Verifying checksums..."

(
cd "${RELEASE_DIR}"
sha256sum -c SHA256SUMS
)

echo ""
echo "=================================================="
echo "RELEASE VERIFIED"
echo "=================================================="
