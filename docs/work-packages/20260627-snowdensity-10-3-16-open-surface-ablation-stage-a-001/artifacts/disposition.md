# Disposition

Evidence mode: Static/Ran.

Status: `NON-PROMOTION-STAGE-A-GATE-NOT-MET`

The Stage A opt-in sublimation candidate is implemented and tested, but it is
not promotion-eligible. It reduced the targeted open-surface cap-limited
over-persistence tail from `30` to `27`, but it worsened the open-surface
under-persistence tail from `54` to `57`. That fails the package's current-scope
bidirectional guardrail.

## Gate Summary

| Gate | Result | Disposition |
|---|---:|---|
| Real coupled direct-production WAT/trace run | PASS | Candidate selector reached `30317` trace rows. |
| Open cap-limited tail reduction | PASS | `30 -> 27`. |
| Under-persistence non-worsening | FAIL | `54 -> 57`. |
| Sublimation magnitude range | PASS | total `0.586351 m`; max daily-lane `0.004834 m`. |
| Whole-model snow-state conservation | PASS | max residual `5.551e-17 m`. |
| No default/schema/cap/fixture/frost drift | PASS | Boundary flags false in diagnostic artifact. |

## Decision

- Do not activate `coe_open_sublimation_stage_a_v1`.
- Keep the activated default bundle:
  `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`.
- Keep `legacy_coe` / `legacy_wepp` rollback selectors intact.
- Keep Stage A available only as an explicit package-bound diagnostic selector.
- Carry the open-surface ablation defect forward, but not as "more vapor loss."
  Any follow-up must preserve the same under-persistence guardrail and likely
  needs the Stage B surface cold-content / surface-temperature mechanism or
  another independently authorized open-exposure process.
