# Disposition — SC-INFILE-IRRIGATION-FIXEDDATE-001

Evidence mode: `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `FDIR-A-001` | `review_agent_a` | high | `amended_closed_with_hold` | Added mode-complete ordering behavior: strict rejects ordering anomalies (`FDIR-E-010`), compat allows legacy warning branch (`FDIR-W-006`) with guard linkage. | `docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:173`, `:182`, `:203`, `:210`, `:220` | Governance evidence for legacy migration remains in `FDIR-GAP-003`. |
| `FDIR-A-002` | `review_agent_a` | medium | `amended_closed` | Replaced abstract boundary statements with concrete boundary surfaces and field mappings. | `docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:188`, `:190`, `:191`, `:192`, `:193`, `:194` | Boundary mapping now executable for parser/scheduler/observability interfaces. |
| `FDIR-A-003` | `review_agent_a` | medium | `amended_closed_with_hold` | Added explicit `iryr_interpretation_mode` field + propagation + cross-file constraint and retained unresolved authority as HOLD. | `docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:97`, `:124`, `:184`, `:246` | `iryr` semantics remain unresolved pending governance (`FDIR-GAP-002`). |
| `FDIR-B-001` | `review_agent_b` | high | `amended_closed` | Added explicit strict/compat guard for contour/non-cropland furrow policy (`G-FDIR-013`) mapping to `FDIR-E-009`/`FDIR-W-005`. | `docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:183`, `:230` | Satisfies invariant-to-guard requirement for strict branch. |
| `FDIR-B-002` | `review_agent_b` | medium | `amended_closed_with_hold` | Carried datver-floor authority conflict into explicit HOLD gap and marked policy provisional until closure. | `docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:212`, `:248`; `docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:43`, `:201` | Unresolved authority conflict is explicit; no false completion claim. |

## Status
- High-severity findings closed in this pass: `FDIR-A-001`, `FDIR-B-001`.
- Open governance HOLDs (not unresolved reviewer findings): `FDIR-GAP-002`, `FDIR-GAP-003`, `FDIR-GAP-004`.
