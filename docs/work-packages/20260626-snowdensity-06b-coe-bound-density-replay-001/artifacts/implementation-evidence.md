# Implementation Evidence

Evidence mode: Static + Ran.

## Static

- Amended `SC-SNOWFREEZE-001` to v85 with `INV-SNOWFREEZE-059`,
  `OBL-SNOWFREEZE-P-034`, and the SNOWDENSITY-06B addendum.
- Added diagnostic-only `openwepp-snowbench coe-bound-density`.
- Added `run_coe_bound_density_snowbench` and report types under
  `crates/openwepp-runner/src/hillslope/snowbench_coe_density.rs`.
- Added `tools/snowfreeze_observed/coe_bound_density_adjudication.py`.
- Added `tests/integration/snowdensity06b_coe_bound_density_replay.rs`.
- Updated the physics-bulk confinement guard to include the new diagnostic
  module and adjudication script only.

## Ran

- `cargo test --test snowdensity06b_coe_bound_density_replay -- --nocapture`
  passed.
- `cargo test --test snowdensity03_physics_bulk_offline_contract -- --nocapture`
  passed.
- `cargo test -p openwepp-runner snowbench -- --nocapture` passed.
- `cargo build -p openwepp-runner --bin openwepp-snowbench` passed.
- `.venv/bin/python tools/snowfreeze_observed/coe_bound_density_adjudication.py --output-dir target/snowdensity06b_coe_bound_density`
  passed.

## Result

Best candidate:
`coe_bound_density_compaction_v1_coe_shortwave_albedo_v1`.

- Whole robust profile vs openWEPP/legacy as-built: fail `9 -> 5`, score
  `84 -> 110`.
- Density-cell profile vs openWEPP/legacy as-built: fail `9 -> 5`, score
  `16 -> 41`.
- Maximum daily CoE SWE identity residual:
  `4.440892098500626e-16 m`.
- Maximum unbounded SWE residual before fixed-boundary normalization:
  `0.1285 m`.

Disposition: offline CoE-bound profile gate cleared; no runtime/default
activation authorized by this package.
