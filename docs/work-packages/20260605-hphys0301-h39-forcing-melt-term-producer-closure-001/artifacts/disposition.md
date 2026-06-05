# Disposition

Status: executed-hold

Evidence mode: static + ran

Static:

- Canonical authority added:
  - `SC-SNOWFREEZE-001#INV-SNOWFREEZE-032`
  - `SC-WATBAL-001#INV-WATBAL-076`
- HPHYS0301 did not modify production `crates/` code.
- The H39 first-2013 HPHYS0300 raw-rain aggregate is not valid production forcing authority because it compares baseline residual rain-on-snow evidence to openWEPP raw rain.
- The HPHYS0301 valid comparison is baseline residual rain-on-snow against openWEPP released plus post-winter rain.
- Independent Claude Code review approved the no-production-edit disposition and withdrew the HPHYS0300 "fix H39 now" recommendation.
- Remaining `RM`, `Snow-Water`, `hrmlt`, and `wmelt` residuals require a comparator-surface audit proving baseline and openWEPP cut-points are the same physical quantity in the same units before any production snow-producer edit or paired `melt.for` / `snowd.for` implementation package.

Ran:

- HPHYS0301 lineage runner passed and generated the ledger/summary/decision artifacts.
- Focused HPHYS0301 contract test passed.
- Full workspace gates passed:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

Decision:

- `hphys0301_route = h39-rain-release-lineage-reclassified-hold`.
- `production_edit_authorized = false`.
- `production_forcing_edit_authorized = false`.
- `production_snow_melt_edit_authorized = false`.
- `baseline_minus_open_raw_rain_mm = -16.476986`.
- `baseline_minus_open_released_plus_post_rain_mm = -0.237193`.

Review disposition:

- Dual review findings are resolved in `review-disposition.md`.
- HPHYS0301 remains in `HOLD` only for science continuation, not package-governance incompleteness.

Continuation:

- Scaffold the next package as a comparator-surface audit for `RM`, `Snow-Water`, and melt-term lineage before asserting another producer defect.
- The audit must prove like-for-like physical quantity and unit pairing for baseline/openWEPP cut-points, then recompute residuals before any `melt.for` / `snowd.for` producer instrumentation or implementation claim.
- Candidate follow-on symbols remain `amelt`, `bmelt`, `cmelt`, `dmelt`, `hrrain`, `hrtemp`, `tdpt`, `hrad`, `cloudC`, `vwind`, `snodpt`, `densgt`, `rain_retained`, and `rain_released`, but they are not production-defect authority until the comparator-surface gate passes.
