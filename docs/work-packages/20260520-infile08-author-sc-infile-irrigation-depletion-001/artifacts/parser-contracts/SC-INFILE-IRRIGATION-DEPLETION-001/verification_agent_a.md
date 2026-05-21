# Verification Agent A — SC-INFILE-IRRIGATION-DEPLETION-001

Evidence: Static

## Per-Finding Verification

| finding_id | source | severity | disposition decision | verdict | evidence (file:line) | verification note |
| --- | --- | --- | --- | --- | --- | --- |
| `IRD-A-001` | `review_agent_a.md` | high | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:35`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:204`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:223`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:44` | Datver policy is now concrete and executable (strict canonical + bounded compat windows by system type). |
| `IRD-A-002` | `review_agent_a.md` | high | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:103`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:127` | Period-stream propagation is now explicit at symbol level across sprinkler/furrow/date fields plus derived metadata. |
| `IRD-A-003` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:167`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:203`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:223` | Compatibility warning outcomes are now typed and guard-linked (`IRD-W-001/002/003/006`). |
| `IRD-A-004` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:143`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:150`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:224` | `irbeg==0` sentinel transitions now have explicit derivation/closure and typed compatibility observability. |
| `IRD-B-001` | `review_agent_b.md` | high | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:97`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:183`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:201`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:225`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:165` | Furrow disable behavior for contour/non-cropland is now explicitly mode-gated (strict error vs compatibility disable-warning). |
| `IRD-B-002` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:93`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:123` | `initialization_complete` now has explicit propagation mapping. |
| `IRD-B-003` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:95`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:125`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:192` | `continuation_order_key` is now explicitly represented in field spec, propagation, and boundary export. |

## Package Verdict
PASS-WITH-NOTES

## Remaining High-Severity Open Items
- None from reviewed findings.

Notes:
- Contract remains `HOLD` due to explicit gap-register items (`IRD-GAP-*`) outside this finding-closure gate.
