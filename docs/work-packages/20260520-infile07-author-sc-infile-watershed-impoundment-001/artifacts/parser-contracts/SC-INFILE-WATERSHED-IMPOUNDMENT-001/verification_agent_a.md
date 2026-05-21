# Verification Agent A — SC-INFILE-WATERSHED-IMPOUNDMENT-001

Evidence: Static

## Per-Finding Verification

| finding_id | source | severity | disposition decision | verdict | evidence (file:line) | verification note |
| --- | --- | --- | --- | --- | --- | --- |
| `IMP-A-001` | `review_agent_a.md` | high | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:77`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:191`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:372`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:153` | `strdes` branch comment records are now present in contract field/progression/boundary surfaces and in the paired spec field dictionary. |
| `IMP-A-002` | `review_agent_a.md` | high | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:206`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:231`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:319` | Contract now includes a normative per-symbol propagation table in full required propagation-map shape (source/parser/runtime/owner/phase/mutability/consumers/guards), resolving grouped-only propagation deficiency. |
| `IMP-A-003` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:354`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:410` | Ordering invariant now has explicit error + guard (`IMP-E-009`, `G-IMP-017`). |
| `IMP-A-004` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:355`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:387`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:412` | Compatibility warning outcomes for no-datver acceptance and surplus truncation are explicit and guard-linked (`IMP-W-001/002`, `G-IMP-018/019`). |
| `IMP-B-001` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:77`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:213`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:153` | Duplicate source-fidelity concern for branch comments is resolved. |
| `IMP-B-002` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:183`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:200`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:319` | Derived `structure_enabled_flags` now has explicit propagation mapping in both the core map and per-symbol table. |
| `IMP-B-003` | `review_agent_b.md` | low | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:356`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:387`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:412` | Surplus-record truncation is now explicitly observable as warning-class output, closing auditability gap. |

## Package Verdict
PASS-WITH-NOTES

## Remaining High-Severity Open Items
- None from reviewed findings.

Notes:
- Contract remains `HOLD` due to explicit gap-register items (`IMP-GAP-*`) that are outside this review-finding closure gate.
