# Pre-Implementation Contract Gate

Status: `BLOCKED / production edits prohibited`

Evidence class: Static authority reconciliation on 2026-08-06.

| Gate | Result | Evidence |
| --- | --- | --- |
| Resolved-snow complete carrier and cold-content-first phase chronology | PASS | `SC-SNOWENERGY-001#INV-SNOWENERGY-029/030`; predecessor complete-carrier shadow evidence. |
| Infiltration-before-runoff liquid mass route | PASS | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-018`; `SC-RUNOFFPART-001`; pinned `grna.for`/`watbal_hourly.for`. |
| Direct transfer of snow-computed terminal excess prohibited | PASS | `INV-SNOWENERGY-029`; terminal branch table; `GAP-SNOWFREEZE-006`. |
| Implicit/error-controlled terminal solver | BLOCKED | No invariant or pinned routine defines the state/error norm, tolerance, event bracket, convergence, or typed failure. |
| Combined melt/sublimation/deposition exhaustion chronology | BLOCKED | No current invariant localizes the earliest event or recomputes vapor flux after the surface transition. |
| Complete post-meltout land-surface energy carrier | BLOCKED | `tmpadj` is frost surface-temperature authority only; no closed receiving-surface component ledger exists. |
| Same-substep snow-free vapor mass plus latent energy | BLOCKED | `SC-EVAP-001` is daily/final-hour water withdrawal authority, not event-local surface exchange. |
| Snow-free precipitation heat | BLOCKED | Existing precipitation-advected heat authority is snow-carrier-specific. |
| Unfrozen-soil and surface-water enthalpy recipients | BLOCKED | `SC-SOIL-001`, `SC-WATBAL-001`, and `SC-RUNOFFPART-001` own constitutive/water mass state, not these energy stores. |
| Persistent coupled shadow ownership | BLOCKED | No parallel soil/water/frost/cover state is admitted; `INV-SNOWFREEZE-091` rejects a second snow mass state. |
| Seasonal/restart claim | BLOCKED | Cannot be supported without the blocked coupled-state ownership and recipients. |

## Gate Verdict

`NO-GO`. Contract-derived tests and production code are later phases and were
not started. Current authority must first be expanded by two independently
reviewed authority-admission increments:

1. `SNOW-POST-MELTOUT-LAND-SURFACE-ENERGY-AUTHORITY`, defining the receiving
   regime selector, component equations, event chronology, energy recipients,
   signs, tolerances, and independently reconstructable ledgers; and
2. `SNOW-STAGE3-COUPLED-SHADOW-STATE-AUTHORITY`, defining parallel-state
   ownership, reconciling `INV-SNOWFREEZE-091`, and bounding soil, frost,
   surface-water, vegetation/residue, restart, and noninterference semantics.

Until the land-surface-energy authority gate passes, CoE remains authoritative
and the current Stage 3 shadow must stop at the existing resolved-mass
boundary. After that gate passes, an event-local terminal implementation may
proceed without making persistence, restart, Snowbird, or seasonal claims. The
coupled-shadow-state authority gate is additionally required before any such
claim or cross-interval shadow state is admitted.
