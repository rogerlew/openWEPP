# Contract Implementation Evidence

Evidence mode: Static.

- `SC-SNOWFREEZE-001` was amended from `contract_version: 92` to
  `contract_version: 94`.
- Added `coe_winter_thaw_state_loss_v1` to the `snow_melt_model` variable row.
- Qualified `INV-SNOWFREEZE-002` so the legacy density gate remains authoritative
  for default/CoE-default behavior, with only the explicit opt-in
  `INV-SNOWFREEZE-066` exception.
- Added `INV-SNOWFREEZE-066`, authorizing only the positive-thaw state-loss
  branch delta and preserving CoE melt terms, radiation, canopy, phase,
  density constants, rain mechanics, frost, fixtures, and public schemas.
- Added v94 review-disposition authority requiring conservation/routing proof
  and coupled direct-production WAT snow-control evidence before any fix or
  activation claim.
- Authorized only package-bound `OPENWEPP_SNOWDENSITY1037_MELT_MODEL`
  diagnostic WAT reruns; parser/runfile/user CLI activation remains forbidden.
- Added invalid-state isolation text rejecting default activation, parser/
  runfile/user activation, albedo-state dependency, tuning, and closure without
  paired event-window improvement.
- Added `OBL-SNOWFREEZE-P-041`.
- Added the `SNOWDENSITY-10.3.7 Opt-In Winter-Thaw State-Loss Addendum`.
- Added revision-history rows for v93 and v94.

Contract-first sequencing was preserved: contract text and contract-derived
tests were added before the Rust selector/branch implementation.
