# Disposition: SC-RUNOFFPART-001

Status: complete
Date: 2026-05-20 UTC
Canonical contract: `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
Reviewed commit SHA: `4c6c504eb79619a3d602943021590e3b25113194`
Review snapshot: `6d936f8fc19fa8064cc1fda506391b819f9a94d9c9f2fbf4b646c42c02a630de`
Post-fix snapshot: `37f151ec1ccd7653a08900745cdef26475e6b26935d19e120494af769be5036c`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `A-001` | `agent_a` | `medium` | `accepted` | Added explicit alias-map row for canonical symbol `De`. | `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md:156` | Restores variable-to-alias continuity for all externally relevant runoff outputs. |
| `A-002` | `agent_a` | `medium` | `accepted` | Added explicit event-closure term definition (`Rtot + Qin = Ftot + Qv + ΔSdep + εevt`) and tied residual to tolerance ID. | `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md:92`, `:116`, `:223` | Clarifies normative accounting boundary for independent implementations. |
| `B-001` | `agent_b` | `medium` | `accepted` | Added normative multi-OFE branch outcome table covering cases 1-4 and required `Qj` outcomes. | `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md:172` | Reduces case-classification ambiguity in runon/runoff implementation paths. |
| `B-002` | `agent_b` | `low` | `accepted` | Split generic rate tolerance into explicit `fi/vi` and `qp` tolerances with units. | `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md:225`, `:226` | Improves comparator/governance clarity for rate-domain checks. |

Final disposition note:
- All reported findings were accepted and addressed in the post-fix snapshot.
- Contract lifecycle remains `in_review` because non-promotable gaps
  (`GAP-RUNOFFPART-002`, `GAP-RUNOFFPART-003`, `GAP-RUNOFFPART-004`) remain
  open and explicitly tracked.
