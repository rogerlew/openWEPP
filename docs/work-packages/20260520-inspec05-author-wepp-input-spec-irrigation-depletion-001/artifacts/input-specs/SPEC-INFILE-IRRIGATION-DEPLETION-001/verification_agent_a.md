# Verification Agent A — SPEC-INFILE-IRRIGATION-DEPLETION-001

Evidence: Static

## Per-Finding Verification

| finding_id | source | severity | disposition decision | verdict | evidence (file:line) | verification note |
|---|---|---|---|---|---|---|
| `IRDEP-A-001` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:242` | Gap/conflict register now includes row-level provenance tags. |
| `IRDEP-A-002` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:157`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:158` | Strict-mode no-datver rejection and explicit pre-94.21 sprinkler-nozzle strict/compat behavior are now typed. |
| `IRDEP-A-003` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:162` | Continuation ordering violations now map to explicit strict/compat outcomes. |
| `IRDEP-B1` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:242` | Duplicate provenance-tag finding closed by same register update. |
| `IRDEP-B2` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:157`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:158` | Legacy compatibility branches now have deterministic guardable outcomes. |
| `IRDEP-B3` | `review_agent_b.md` | low | amend | closed | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:247` | Provenance-only item reclassified to non-blocking note (`IRDEP-NOTE-001`). |

## Package Verdict
PASS

## Remaining High-Severity Open Items
- None from reviewed findings.

Notes:
- HOLD blockers remain in Section 10 (`IRDEP-GAP-001..003`) but are not unresolved high-severity review findings in this gate.
