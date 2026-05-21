# Verification Agent A — SC-INFILE-IRRIGATION-FIXEDDATE-001

Evidence: Static

## Per-Finding Verification

| finding_id | source | severity | disposition decision | verdict | evidence (file:line) | verification note |
| --- | --- | --- | --- | --- | --- | --- |
| `FDIR-A-001` | `review_agent_a.md` | high | `amended_closed_with_hold` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:182`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:203`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:220`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:247` | Ordering behavior is now mode-complete (strict reject + compatibility warning/provenance), with migration evidence explicitly kept as HOLD. |
| `FDIR-A-002` | `review_agent_a.md` | medium | `amended_closed` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:190`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:191`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:192`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:193`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:194` | Boundary export mapping now uses concrete boundary surfaces and field mappings. |
| `FDIR-A-003` | `review_agent_a.md` | medium | `amended_closed_with_hold` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:97`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:124`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:184`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:246` | `iryr` interpretation is now explicitly represented and guard-linked while unresolved authority remains declared HOLD. |
| `FDIR-B-001` | `review_agent_b.md` | high | `amended_closed` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:183`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:230` | Strict and compatibility contour/non-cropland furrow policy now has explicit guard mapping (`G-FDIR-013`). |
| `FDIR-B-002` | `review_agent_b.md` | medium | `amended_closed_with_hold` | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:36`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:212`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:248`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:43` | Datver-floor authority conflict is now explicitly carried as HOLD and marked provisional in the contract policy path. |

## Package Verdict
PASS-WITH-NOTES

## Remaining High-Severity Open Items
- None from reviewed findings.

Notes:
- Contract remains `HOLD` due to explicit governance gaps (`FDIR-GAP-002`, `FDIR-GAP-003`, `FDIR-GAP-004`) outside this finding-closure gate.
