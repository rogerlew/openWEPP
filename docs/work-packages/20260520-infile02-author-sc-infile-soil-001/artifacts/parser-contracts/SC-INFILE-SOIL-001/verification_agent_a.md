# Verification Agent A — SC-INFILE-SOIL-001

Evidence: Static

## Per-Finding Verification

| finding_id | source | severity | disposition decision | verdict | evidence (file:line) | verification note |
| --- | --- | --- | --- | --- | --- | --- |
| `SOL-A-001` | `review_agent_a.md` | high | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:97`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:103` | Extended 9002+/9005 fields (`theta_r/theta_s/alpha/npar/ks/Rosetta wp/fc`) now have explicit per-field rows. |
| `SOL-A-002` | `review_agent_a.md` | high | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:174` | Boundary-export mapping section is now explicitly present. |
| `SOL-A-003` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:172` | Cross-file rule now explicitly disambiguates mode-scoped topology authority (`ntemp==nofe` vs `ntemp==nchan`). |
| `SOL-B1` | `review_agent_b.md` | high | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:97`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:103` | Duplicate high-severity field-coverage finding closed by same table expansion. |
| `SOL-B2` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:124`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:127` | Propagation map now includes explicit rows for extended hydraulic/pedotransfer and policy fields. |
| `SOL-B3` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:174` | Duplicate boundary-export requirement closed by same section addition. |

## Package Verdict
PASS-WITH-NOTES

## Remaining High-Severity Open Items
- None from reviewed findings.

Notes:
- Contract remains `HOLD` due to unresolved gap register items (`SOL-GAP-001..003`) outside this closure gate.
