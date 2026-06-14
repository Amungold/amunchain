# Evidence Manifest — [GATE_ID]
Gate: [NV-XX] | Title: [Title] | Status: [RUNNING/PASS/CERTIFIED] | Date: [YYYY-MM-DD] | Baseline: 90f4993

## Evidence Files
| # | File | Hash (sha256) |
|---|------|----------------|
| 1 | genesis.json | [FILL] |
| 2 | genesis.sha256 | [FILL] |
| 3 | validator_1_roots.txt | [FILL] |
| 4 | validator_2_roots.txt | [FILL] |
| 5 | validator_3_roots.txt | [FILL] |
| 6 | validator_4_roots.txt | [FILL] |
| 7 | reproduce.sh | [FILL] |
| 8 | ../../Cargo.lock | [FILL] |

## Integrity Verification
sha256sum -c MANIFEST.sha256

## MANIFEST.sha256 Generation
sha256sum \
  genesis.json \
  genesis.sha256 \
  validator_1_roots.txt \
  validator_2_roots.txt \
  validator_3_roots.txt \
  validator_4_roots.txt \
  reproduce.sh \
  ../../Cargo.lock \
  > MANIFEST.sha256
