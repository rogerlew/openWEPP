# Disposition: SC-PLANT-001

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`

Canonical contract under disposition:
- path: `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- commit_sha: `907a206f6e5857ca410665dd81a3830692e35181`
- reviewed_snapshot_pre_fix: `1a1b42d0d505304e3cc5e93a2b49d41bac3afd9d67d0208e72e8f4c05702917e`
- reviewed_snapshot_post_fix: `fe68d679386055269e56964c0df057392f9515d677e9de7bcf59c7e5e331a4bc`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| A-001 | agent_a | high | accepted | Added claim-level evidence tags to invalid states, obligations, and boundary disposition claims. | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:117`, `:126`, `:133`, `:140` | Closed in amendment pass. |
| A-002 | agent_a | medium | accepted | Added missing symbols (`DeltaBp`, `DeltaBi`, `Rdx`, `CRITVM`, `gi`, `RGCMIN`) to variables/units table. | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:84` | Closed in amendment pass. |
| A-003 | agent_a | medium | accepted | Added dedicated phenology anchor and rewired invariant authority reference. | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:52`, `:95` | Closed in amendment pass. |
| A-004 | agent_a | low | accepted | Added explicit `Promotability` field/labels to all gap rows. | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:161` | Closed in amendment pass. |
| B-001 | agent_b | high | amended | Replaced overly global cropland phenology invariant semantics with scoped cropland invariant and added rangeland-specific growth invariant. | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:95`, `:103` | Reviewer proposed reject-as-written; author action amended structure. |
| B-002 | agent_b | high | accepted | Added explicit promotability labels including `non-promotable` for cross-contract dependency gap. | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:166` | Closed in amendment pass. |
| B-003 | agent_b | medium | accepted | Added missing claim-level evidence tags across normative sections. | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:117`, `:142` | Co-resolved with A-001. |
| B-004 | agent_b | medium | accepted | Added missing symbol definitions and aligned tolerance symbol list with declared variables. | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:84`, `:156` | Co-resolved with A-002. |
| B-005 | agent_b | low | accepted | Normalized evidence-mode casing to `static` across metadata/body. | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md:16`, `:26` | Closed in amendment pass. |

Disposition summary:
- High-severity findings: 3 total, all closed via accepted/amended actions.
- Final promotion state for this revision: `HOLD` (open `GAP-PLANT-004` is explicitly `non-promotable`).
