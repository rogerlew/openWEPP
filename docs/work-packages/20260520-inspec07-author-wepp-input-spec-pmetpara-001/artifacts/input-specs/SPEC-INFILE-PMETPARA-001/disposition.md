# Disposition — SPEC-INFILE-PMETPARA-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `PMET-A-001` | `review_agent_a.md` | medium | amend | Added provenance-tag column and row-level provenance tags in gap/conflict register. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:185` | Required conflict-authority tagging is now explicit. |
| `PMET-A-002` | `review_agent_a.md` | medium | amend | Added deterministic crop-key policy covering normalization, width handling, strict/compat truncation behavior, and explicit warnings/errors. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:77` | Removes ambiguity in fallback-triggering key semantics. |
| `PMET-A-003` | `review_agent_a.md` | medium | amend | Added explicit typed rejection for datver-prefixed variant (`FormatVersionLineUnsupportedError`). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:127` | Rejection branch now maps to concrete error surface. |
| `PMET-B1` | `review_agent_b.md` | medium | amend | Resolved by same provenance-tagging update as `PMET-A-001`. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:185` | Duplicate finding closure. |
| `PMET-B2` | `review_agent_b.md` | medium | amend | Added typed strict/compat outcomes for overlength crop keys and truncation warnings. | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:130` | Makes string-width compatibility handling guardable and deterministic. |
| `PMET-B3` | `review_agent_b.md` | low | amend | Reclassified provenance-only `wepppyo3` gap to non-blocking note (`PMET-NOTE-001`). | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:190` | Clarifies promotion blockers vs provenance completeness. |

## Unresolved / HOLD blockers
- `PMET-GAP-001`: strict-vs-compat fallback policy for crop-name miss remains open (`[DIRECT][E-US-02]`, `[DIRECT][E-WF-03]`).
- `PMET-GAP-002`: canonical string-length/truncation policy for full row payload remains unresolved (`[DIRECT][E-WF-04]`, `[DIRECT][E-WP-01]`, `[DIRECT][E-WP-03]`).
- `PMET-GAP-003`: delimiter/quoting policy for `actlnam` remains unresolved (`[DIRECT][E-US-02]`, `[DIRECT][E-WP-01]`, `[DIRECT][E-WF-06]`).
