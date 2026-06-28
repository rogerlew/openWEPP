# Implementation Evidence

Evidence class: Static + Ran.

Production changes:

- Added `SnowMeltModel::CoeOpenSublimationStageBV1` with selector id
  `coe_open_sublimation_stage_b_v1`.
- Preserved no-env default `coe_liquid_holding_capacity_v1`; Stage B is
  explicit opt-in only through `OPENWEPP_SNOWDENSITY1038_MELT_MODEL`.
- Added Stage B sublimation branch that reuses Stage A vapor accounting and
  changes only the sublimation surface vapor pressure to a bounded active
  surface-layer temperature gate.
- Active surface-layer depth is bounded by `0.25 m` Marks/SNOBAL authority and
  current snow depth; no fixture/site inputs are consumed.
- Sublimation remains vapor export, excluded from routed liquid, and bounded by
  available SWE.

Diagnostic tool:

- `tools/snowfreeze_observed/sublimation_stage_b_unlock.py` runs four profiles:
  current default, Stage A legacy phase, partition + Stage A, and Stage B.
- It writes the cross-SNOTEL matrix and trace proof to
  `artifacts/sublimation-stage-b-unlock.{json,md}`.

Real-run disposition:

- Stage A legacy phase scored `20/153` versus current default `15/179`.
- Partition + Stage A scored `19/168`.
- Stage B reduced aggregate sublimation magnitude but scored `15/178`, one
  point below current default, with three worse robust cells.
