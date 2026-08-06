# Review Disposition

Status: complete / all six findings accepted

| Finding | Source | Disposition | Rationale / required change |
| --- | --- | --- | --- |
| Terminal integrator under-specified | Review A finding 1 | Accepted | Require an implicit or error-controlled terminal solve, contract tolerances, event bracketing, convergence failure, and warming/cooling flux reevaluation. Replace `exact` with `localized within contract tolerance`. |
| Receiving surface over-specified | Review A finding 2 | Accepted | Replace fixed `snow_free_wet_soil` with a typed snow-free land-surface regime selected from actual cover, frost, infiltration, and ponding state. Add soil, runoff-partition, evaporation, and pinned baseline authority. |
| Snow-only persistence cannot support seasonal claims | Review A finding 3 | Accepted | Require coupled surface/soil thermal-water-frost shadow state for restart/Snowbird claims; otherwise limit evidence to event-local diagnostics. |
| Terminal liquid and infiltration-first routing omitted | Review A finding 4 | Accepted | Name terminal retained-store release, surface-liquid supply, infiltration, soil storage, ponding/overflow, evaporation, and residual runoff. Distinguish snow drainage from hillslope runoff. |
| Vapor/phase event chronology incomplete | Review A finding 5 | Accepted | Localize the earliest combined melt/sublimation exhaustion event; recompute post-event vapor flux and define simultaneous precipitation/deposition and reappearance chronology. |
| Authority hold occurs after code | Review A finding 6 | Accepted | Make complete receiving-surface and coupled-state authority a Phase-1 go/no-go gate. No production edit may precede its pass; otherwise close `HOLD` or prospectively split an authority successor. |

The reviewer agreed with the proposal's rejection of direct transfer of
snow-computed terminal excess into soil. That decision is retained.
