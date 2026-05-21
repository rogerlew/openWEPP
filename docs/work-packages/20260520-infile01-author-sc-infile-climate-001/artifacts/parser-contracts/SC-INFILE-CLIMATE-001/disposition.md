# Disposition — SC-INFILE-CLIMATE-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `CLI-A-001` | `review_agent_a.md` | `high` | `amend` | Added explicit boundary-export mapping section covering metadata, daily forcing, and breakpoint payload boundaries with name/unit continuity requirements. | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:158` | High-severity closure complete. |
| `CLI-A-002` | `review_agent_a.md` | `medium` | `amend` | Added `generator_cmd` to grammar-linked field table and propagation map as preserved optional metadata with explicit runtime/export treatment. | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:87`; `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:107` | Source-vs-simulation handling is now explicit. |
| `CLI-A-003` | `review_agent_a.md` | `medium` | `amend` | Added explicit strict/compat breakpoint cardinality policy (`nbrkpt > 50`), typed error class, and guard linkage. | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:147`; `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:172`; `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:194` | Provisional policy codified; residual portability benchmarking remains a HOLD gap (`CLI-GAP-003`). |
| `CLI-B1` | `review_agent_b.md` | `medium` | `amend` | Closed by same boundary-export section added for `CLI-A-001`. | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:158` | Duplicate finding resolved. |
| `CLI-B2` | `review_agent_b.md` | `medium` | `amend` | Updated propagation phases to reflect lifecycle usage (`init,daily`, `init,event`, `init,daily,event`) instead of all-`init`. | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:108`; `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:112` | Phase granularity now matches downstream consumers. |
| `CLI-B3` | `review_agent_b.md` | `low` | `amend` | Replaced broad file-only legacy anchors with line-local evidence anchors in the Evidence section. | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:20` | Traceability improved for verification passes. |

## Unresolved / HOLD
- `CLI-GAP-001`, `CLI-GAP-002`, and `CLI-GAP-003` remain explicit HOLD items in the canonical contract and are not closed in this gate.
