# Disposition: SC-SNOWFREEZE-001

Status: complete (cycle-1)
Date: 2026-05-20 UTC
Evidence mode: `Static`

Canonical contract under disposition:
- path: `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- commit_sha: `4c6c504eb79619a3d602943021590e3b25113194` (baseline `HEAD`)
- reviewed_snapshot_pre_fix: `722f5e1863d00a1901c2fcec559dc681126d7d5c787e7be67633d2c9341d5b5c`
- reviewed_snapshot_post_fix: `17f6306c474c9abbb95dc45a53856170eef5df4747c831509e6ece0d2cb65254`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| A-001 | agent_a | high | accepted | Re-scoped drift variables/aliases as inactive lineage provenance only and clarified runtime-vs-governance separation in invariants and gaps. | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:75`, `:107`, `:141`, `:198`, `:219` | Closed; drift-governance ambiguity removed. |
| A-002 | agent_a | high | accepted | Added canonical symbols/units for frost and thaw depth outputs and propagated through invariants/obligations/alias map. | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:93`, `:94`, `:110`, `:149`, `:176`, `:186` | Closed; co-resolves `B-002`. |
| A-003 | agent_a | medium | accepted | Introduced explicit pre-hour melt-bound symbol `Dsavail` and aligned invariant/guard/tolerance semantics to branch timing. | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:71`, `:104`, `:118`, `:168`, `:207` | Closed; co-resolves `B-001` and `B-005`. |
| A-004 | agent_a | medium | accepted | Fixed `InfCap_frz` to required exported boundary unit (`m s^-1`) and clarified conversion requirement for non-SI internals. | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:98`, `:152` | Closed; co-resolves `B-003`. |
| A-005 | agent_a | low | accepted | Added explicit evidence tags to scientific-scope and degenerate-state claims. | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:36`, `:45`, `:158`, `:162` | Closed; co-resolves `B-006`. |
| B-001 | agent_b | high | accepted | Same amendment as A-003: clarified melt-bound symbol/timing and post-branch semantics. | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:71`, `:104`, `:118`, `:168`, `:207` | Closed in shared fix with A-003. |
| B-002 | agent_b | high | accepted | Same amendment as A-002: added `Dfrost`/`Dthaw` symbols and full boundary propagation. | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:93`, `:94`, `:110`, `:149`, `:176`, `:186` | Closed in shared fix with A-002. |
| B-003 | agent_b | medium | accepted | Same amendment as A-004: strict `InfCap_frz` unit declaration and boundary conversion requirement. | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:98`, `:152` | Closed in shared fix with A-004. |
| B-004 | agent_b | medium | accepted | Replaced product-based zero-depth tolerance rule with explicit conditional zero-depth/zero-density closure check. | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:210` | Closed. |
| B-005 | agent_b | medium | accepted | Harmonized melt invariant, guard map, invalid state, and boundary disposition language to remove clamp/hard-error conflict. | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:104`, `:118`, `:168`, `:193` | Closed in shared fix with A-003. |
| B-006 | agent_b | low | accepted | Same amendment as A-005: evidence tags added to non-trivial scope/degenerate claims. | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:36`, `:45`, `:158`, `:162` | Closed in shared fix with A-005. |

Disposition summary:
- High-severity findings: 4 total, all closed.
- Medium-severity findings: 5 total, all closed.
- Low-severity findings: 2 total, all closed.
- Final promotion state for this revision: `HOLD` (non-promotable gaps remain open in contract gap register).
