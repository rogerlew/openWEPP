# Disposition: SC-EVAP-001

Status: complete
Date: 2026-05-20 UTC
Canonical contract: `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
Reviewed commit SHA: `2692ac59f67bed4094df62c5883f3826abe9f468`
Review snapshot: `021ef71dd6f82b70a2057e5c12abfb459fe5df54468fc6603ab07663bfe21922`
Post-fix snapshot: `3f598f7920ffcc6afb4fd4d3e1005d04529290f8baf1f3b00584cf09005465c0`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `A-001` | `agent_a` | `high` | `accepted` | Added evidence tags to scope claims, converted Allowed Degenerate States to evidence-tagged table, and added evidence tags to tolerance narrative/table. | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md:37`, `:137`, `:181`, `:184` | Restores claim-level provenance annotation required by procedure. |
| `A-002` | `agent_a` | `medium` | `accepted` | Normalized document evidence-mode casing to `Static` in metadata and body header. | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md:16`, `:26` | Aligns document-level evidence tokens with procedure convention. |
| `A-003` | `agent_a` | `medium` | `accepted` | Added `Θc` coverage to Symbol Alias Map. | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md:129` | Restores symbol continuity for critical threshold variable. |
| `A-004` | `agent_a` | `medium` | `accepted` | Reworked snow-precedence anchor to explicit Chapter-5 provenance and standardized chapter source paths. | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md:57`, `:65`, `:96` | Removes ambiguous anchor naming/path inconsistency. |
| `B-001` | `agent_b` | `high` | `accepted` | Added missing `Θc` alias coverage in canonical symbol map. | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md:129` | Same corrective action as `A-003`; finding closed by shared amendment. |
| `B-002` | `agent_b` | `medium` | `accepted` | Added explicit aggregate `ET` symbol row to Variables table and Alias Map for closure boundary semantics. | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md:84`, `:131` | Completes externally relevant symbol coverage for `INV-EVAP-009` and consumer obligations. |
| `B-003` | `agent_b` | `medium` | `accepted` | Added claim-level evidence tagging for degenerate-state and tolerance claims. | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md:137`, `:181`, `:184` | Satisfies evidence-tagging requirement for non-trivial assertions. |
| `B-004` | `agent_b` | `low` | `accepted` | Standardized all Chapter-5 authority-source paths and replaced ambiguous snow anchor naming. | `docs/specifications/science-contracts/contracts/SC-EVAP-001.md:57`, `:65` | Improves citation hygiene and audit reproducibility. |

Final disposition note:
- All reported findings were accepted and addressed in the post-fix snapshot.
- Contract lifecycle remains `in_review` because non-promotable cross-contract
  gaps (`GAP-EVAP-002`, `GAP-EVAP-003`) are still open.
