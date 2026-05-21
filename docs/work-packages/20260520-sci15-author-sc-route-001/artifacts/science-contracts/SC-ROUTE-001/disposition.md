# Disposition: SC-ROUTE-001

Status: complete
Date: 2026-05-20 UTC
Canonical contract: `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
Reviewed commit SHA: `2692ac59f67bed4094df62c5883f3826abe9f468`
Review snapshot: `a6b49652a11e191bfcb01e40baaa4de392c3cd07bd4e8e1fd9530e8229a3afd0`
Post-fix snapshot: `58a15974682aa1f0f2cef8eef68e95f7be4a0ee4de785e9c6bdf1319ee6a1c87`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `A-001` | `agent_a` | `medium` | `accepted` | Strengthened outlet-peak method invariant and guard text to require exactly one selected method and to reject mixed/implicit fallback behavior. | `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md:106`, `:124` | Restores deterministic/traceable `qpo` method semantics. |
| `A-002` | `agent_a` | `low` | `accepted` | Added explicit alias-map coverage for `durrof` in the event-duration row. | `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md:143` | Restores full symbol-continuity coverage for externally relevant variables. |
| `B-001` | `agent_b` | `medium` | `accepted` | Added Chapter-13 applicability-limits authority anchor, governance invariant/guard, and explicit non-promotable gap for runtime applicability guards. | `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md:69`, `:113`, `:131`, `:222` | Makes authority bounds and promotion constraints explicit. |
| `B-002` | `agent_b` | `low` | `accepted` | Promoted `roff <= 0.001 m^3` threshold branch into invariant and runtime guard mapping, with corresponding invalid-state coverage. | `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md:107`, `:125`, `:169` | Ensures threshold-gated peak/duration behavior is enforcement-visible. |

Final disposition note:
- All reported findings were accepted and addressed in the post-fix snapshot.
- Contract lifecycle remains `in_review` because non-promotable gaps
  (`GAP-ROUTE-002`, `GAP-ROUTE-003`, `GAP-ROUTE-005`) remain open.
