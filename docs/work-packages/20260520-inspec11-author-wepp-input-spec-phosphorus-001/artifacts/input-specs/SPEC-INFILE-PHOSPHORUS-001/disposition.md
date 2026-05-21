# Disposition — SPEC-INFILE-PHOSPHORUS-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `PHOS-A-001` | `review_agent_a.md` | medium | amend | Added row-level provenance tags to gap/conflict register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/phosphorus.spec.md:183` | Conflict provenance is now explicit and auditable. |
| `PHOS-A-002` | `review_agent_a.md` | medium | amend | Added explicit strict/compat typed outcomes for header-string mismatch policy conflict. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/phosphorus.spec.md:125` | Header acceptance behavior is now deterministic by mode. |
| `PHOS-A-003` | `review_agent_a.md` | low | amend | Reclassified provenance/ownership item to non-blocking note (`PHOS-NOTE-002`). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/phosphorus.spec.md:189` | Keeps HOLD focused on correctness-impact blockers. |
| `PHOS-B1` | `review_agent_b.md` | medium | amend | Resolved by same provenance-tagging update as `PHOS-A-001`. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/phosphorus.spec.md:183` | Duplicate finding closure. |
| `PHOS-B2` | `review_agent_b.md` | medium | amend | Resolved by mode-gated typed header mismatch outcomes. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/phosphorus.spec.md:125` | Removes implementation ambiguity between legacy and validator behavior. |
| `PHOS-B3` | `review_agent_b.md` | low | amend | Resolved by non-blocking note classification for provenance-only ownership tracking. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/phosphorus.spec.md:189` | Promotion criteria clarity improved. |

## Unresolved / HOLD blockers
- `PHOS-GAP-001`: no authoritative usersum format section for `phosphorus.txt` (`[DIRECT][E-US-01]`).
- `PHOS-GAP-002`: concentration bounds policy remains unresolved beyond type/finite validity (`[DIRECT][E-WF-01]`, `[DIRECT][E-WP-01]`).
- `PHOS-GAP-003`: hillslope applicability semantics remain under-specified vs watershed-documented consumption (`[DIRECT][E-WF-04]`, `[DIRECT][E-WF-05]`, `[DIRECT][E-WF-07]`).
