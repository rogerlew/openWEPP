# Implementation Evidence

Evidence mode: Static + Ran.

## Static

- Amended `SC-SNOWFREEZE-001` to v84 with `INV-SNOWFREEZE-058`,
  `OBL-SNOWFREEZE-P-033`, and the SNOWDENSITY-06 addendum.
- Added offline `PhysicsBulkVariant::DensityCompactionV1`.
- Exposed named PTM/POC/liquid-water compaction constants in
  `PhysicsBulkConstants` and `physics_bulk_summary.json`.
- Updated `apply_time_compaction` to consume named constants instead of hidden
  literals.
- Preserved baseline candidate melt constants for `density_compaction_v1`:
  `positive_degree_melt_kg_m2_per_c_hour`, `solar_melt_efficiency`, and
  `subfreezing_cold_content_relaxation_per_hour`.
- Extended `physics_bulk_adjudication.py` with density/densification robust-cell
  summaries for `long_term_cold_season_bulk_density`,
  `seasonal_densification_trajectory`, `seasonal_depth_swe_slope`, and
  `cross_cutting_bias_sign_consistency`.

## Ran

- `cargo test -p openwepp-runner snowbench_physics_bulk -- --nocapture` passed.
- `cargo test --test snowdensity03_physics_bulk_offline_contract -- --nocapture`
  passed.
- `cargo test --test snowdensity06_density_compaction -- --nocapture` passed.
- `cargo build -p openwepp-runner --bin openwepp-snowbench` passed.
- `.venv/bin/python tools/snowfreeze_observed/physics_bulk_adjudication.py --output-dir target/snowdensity06_adjudication_density_only --variant density_compaction_v1`
  passed.
- `rg -n "qwet|frzftp" crates || true` returned no matches.

## Result

`density_compaction_v1` is a density-cell improvement, not a whole-rubric or
runtime promotion candidate:

- Legacy/as-built density cells: fail `9`, score `16`.
- `density_compaction_v1` density cells: fail `7`, score `22`.
- Legacy/as-built whole robust profile: fail `9`, score `84`.
- `density_compaction_v1` whole robust profile: fail `18`, score `46`.

Disposition: complete the density-compaction gate, no runtime/default
activation. Next package should replay the density update against fixed CoE
melt/liquid/SWE-loss operands instead of the old physics-bulk degree-day melt
surrogate.
