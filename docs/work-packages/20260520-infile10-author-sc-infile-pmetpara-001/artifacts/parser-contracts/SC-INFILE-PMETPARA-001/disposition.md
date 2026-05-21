# Disposition — SC-INFILE-PMETPARA-001

Evidence mode: `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `PMET-A-001` | `review_agent_a` | high | `amended_closed` | Added explicit optional-surface state in field/propagation/boundary (`sidecar_present`, `iflget`) for missing-sidecar branch (`iflget=1`). | `docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:66`, `:67`, `:82`, `:83`, `:145`, `:174` | Branch is now explicit and guard-linked. |
| `PMET-B-001` | `review_agent_b` | high | `amended_closed` | Added same explicit provenance surfaces and guard linkage for sidecar absence semantics. | `docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:66`, `:67`, `:145`, `:174`; `docs/specifications/wepp-input-files/specs/pmetpara.spec.md:94` | Closes missing-surface contract visibility gap. |
| `PMET-A-002` | `review_agent_a` | high | `amended_closed` | Corrected lookup fallback mutability/ownership: `fallback_first_row_used` moved to runtime lookup module and marked mutable. | `docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:85`, `:92` | Aligns lookup-time behavior with runtime state semantics. |
| `PMET-A-003` | `review_agent_a` | medium | `amended_closed` | Rewrote cross-file constraints to concrete coupled surfaces (`normalized_crop_key`, management symbol coupling, explicit `iflget` coupling). | `docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:132`, `:133`, `:134`, `:135` | Constraints are now auditable and executable. |
| `PMET-A-004` | `review_agent_a` | medium | `amended_closed_with_hold` | Added explicit provisional failure/warning surface for non-canonical `actlnam` tokenization via taxonomy + guard. | `docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:122`, `:126`, `:160`, `:175`, `:190` | Under-specification remains tracked by `PMET-GAP-003`. |
| `PMET-B-002` | `review_agent_b` | medium | `amended_closed` | Added guard linkage for missing-sidecar compat warning (`PMET-W-001`) via `G-PMET-009`. | `docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:123`, `:174` | Invariant-to-warning path now explicit. |

## Status
- High-severity findings closed in this pass: `PMET-A-001`, `PMET-A-002`, `PMET-B-001`.
- Open governance HOLDs (not unresolved reviewer findings): `PMET-GAP-001`, `PMET-GAP-002`, `PMET-GAP-003`.
