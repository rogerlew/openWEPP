# Conflict And Disposition Ledger

Status: complete.
Evidence mode: Static.

## Findings

- `SC-SNOWFREEZE-001` lacked the 05A melt-modernization authority before this
  package.
- SNOWDENSITY-04's `dense_slow_melt_v1` runtime-opt-in handoff conflicted with
  the operator decision to keep degree-day melt variants as negative
  benchmarks.
- WEPP Chapter 3 prose writes `amelt - bmelt + cmelt + dmelt`, while current
  openWEPP traces and `clim05_snow_runtime_kernel_contract.rs` use
  `amelt + bmelt + cmelt + dmelt`.

## Disposition

- Accepted: amend `SC-SNOWFREEZE-001` before production code.
- Accepted: bind `melt_bmelt_in` as an already-signed contribution. No formula
  code was changed.
- Accepted: reject `dense_slow_melt_v1` promotion; retain it as a negative
  benchmark only.
- Accepted: bind no-radiation-tuning guard to `SC-CLIMATE-001#INV-CLIMATE-013`.
- Deferred: shortwave source binding to SNOWDENSITY-05B.
- Deferred: albedo state/constants to SNOWDENSITY-05C.
- Deferred: opt-in production implementation and conservation reconstruction to
  SNOWDENSITY-05D.
