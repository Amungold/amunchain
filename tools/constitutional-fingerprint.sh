#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."

echo "# AmunChain Constitutional Fingerprint"
echo "# Generated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
echo "# Any change to constitutional files MUST update this fingerprint."
echo ""

FILES=$(find docs/architecture tools .github/workflows -type f \
    \( -name "*.md" -o -name "*.sh" -o -name "*.yml" -o -name "*.json" \) \
    ! -name "CONSTITUTIONAL_FINGERPRINT.txt" 2>/dev/null | sort)

for f in $FILES; do
    [ -f "$f" ] || continue
    hash=$(sha256sum "$f" | cut -d' ' -f1)
    echo "${hash}  ${f}"
done
