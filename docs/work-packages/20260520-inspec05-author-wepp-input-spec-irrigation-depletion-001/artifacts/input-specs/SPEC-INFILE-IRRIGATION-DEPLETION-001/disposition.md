# Disposition — SPEC-INFILE-IRRIGATION-DEPLETION-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `IRDEP-A-001` | `review_agent_a.md` | medium | amend | Added provenance-tag column and per-row provenance values in gap/conflict register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:242` | Satisfies required conflict provenance structure. |
| `IRDEP-A-002` | `review_agent_a.md` | medium | amend | Added explicit strict-mode typed outcome for omitted `datver` header and explicit strict/compat behavior for legacy sprinkler-nozzle omission branch. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:157` | Covers both no-datver and pre-94.21 sprinkler shape behavior. |
| `IRDEP-A-003` | `review_agent_a.md` | medium | amend | Added typed strict/compat outcomes for continuation/initialization ordering violations. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:162` | Ordering behavior is now deterministic in parser contracts. |
| `IRDEP-B1` | `review_agent_b.md` | medium | amend | Resolved by same provenance-tagging update as `IRDEP-A-001`. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:242` | Duplicate finding closure. |
| `IRDEP-B2` | `review_agent_b.md` | medium | amend | Resolved by explicit strict/compat typed outcomes for no-datver and pre-94.21 nozzle omission branches. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:157` | Legacy compatibility branches now map to guardable outcomes. |
| `IRDEP-B3` | `review_agent_b.md` | low | amend | Reclassified provenance-only gap to non-blocking note (`IRDEP-NOTE-001`). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:247` | Promotion blockers now track correctness-impact gaps only. |

## Unresolved / HOLD blockers
- `IRDEP-GAP-001`: compatibility-matrix ratification for no-datver/legacy-version branches is unresolved (`[DIRECT][E-US-03]`, `[DIRECT][E-WF-01]`, `[DIRECT][E-WF-07]`).
- `IRDEP-GAP-002`: strict-vs-compat policy for legacy in-place normalization (`irdmin`, `depsrg`) is unresolved (`[DIRECT][E-WF-03]`, `[DIRECT][E-WF-08]`).
- `IRDEP-GAP-003`: continuation-stream data-model/ingestion closure remains unresolved (`[DIRECT][E-US-04]`, `[DIRECT][E-WF-05]`).
