# Disposition — SC-INFILE-FROST-001

Evidence mode: `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `FROST-A-001` | `review_agent_a` | high | `amended_closed_with_hold` | Encoded unresolved prefix/version policy in applicability matrix, taxonomy (`FROST-E-006`), and guard (`G-FROST-008`) with provisional reject behavior. | `docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:40`, `:137`, `:184`, `:198` | Governance uncertainty remains explicit in `FROST-GAP-002`. |
| `FROST-A-002` | `review_agent_a` | high | `amended_closed_with_hold` | Reframed `kfactor(1..3)` semantics as unresolved slot indices (not deterministic class labels) across field, propagation, and cross-file sections. | `docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:75`, `:76`, `:77`, `:94`, `:95`, `:96`, `:147`, `:197`; `docs/specifications/wepp-input-files/specs/frost.spec.md:97` | Class-label mapping remains `HOLD` in `FROST-GAP-001`. |
| `FROST-A-003` | `review_agent_a` | medium | `amended_closed` | Added field-level clamp/default provenance (`legacy_clamp_fields`) and closure hooks for per-field observability. | `docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:81`, `:100`, `:117`, `:124` | Compatibility normalization is now field-auditable. |
| `FROST-B-001` | `review_agent_b` | high | `amended_closed` | Resolved grammar/policy inconsistency by making strict vs compatibility grammar explicit (`strict_frost_file` requires line2). | `docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:49`, `:50`, `:51`, `:58`; `docs/specifications/wepp-input-files/specs/frost.spec.md:49`, `:50`, `:51` | Contract now has executable mode-consistent grammar. |
| `FROST-B-002` | `review_agent_b` | medium | `amended_closed` | Updated guard path for missing line2 to explicit strict syntax failure vs compat default-warning branch. | `docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:133`, `:181` | Strict missing-line2 no longer routed through runtime-warning style semantics. |

## Status
- High-severity findings closed in this pass: `FROST-A-001`, `FROST-A-002`, `FROST-B-001`.
- Open governance HOLDs (not unresolved reviewer findings): `FROST-GAP-001`, `FROST-GAP-002`.
