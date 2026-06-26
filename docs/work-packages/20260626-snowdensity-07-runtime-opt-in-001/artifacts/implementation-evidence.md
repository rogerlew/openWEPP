# Implementation Evidence

Evidence class: Static + Ran.

## Contract

- `SC-SNOWFREEZE-001` is v86.
- Added `INV-SNOWFREEZE-060`, `OBL-SNOWFREEZE-P-035`,
  `snow_density_model`, and CoE boundary carry variables.
- Contract says `legacy_wepp` remains default/rollback and
  `physics_bulk_density_compaction_v1` is typed opt-in only.

## Runtime

- Added `09_snow_density.rs` with:
  - `SnowDensityModel::LegacyWepp`
  - `SnowDensityModel::PhysicsBulkDensityCompactionV1`
  - CoE-bound density runtime update
  - SWE identity residual reporting
  - fail-closed finite/domain guards
- Wired typed active snow partition to call the density selector after CoE
  snow coupling.
- Preserved surface-driven publication/default behavior as
  `SnowDensityModel::LegacyWepp`.
- Added CoE boundary depth/density/settle carry to winter-column snow state,
  direct snow runtime carry, R4G inputs/state/downstream/shadow projection, and
  runner direct publication builders.

## Tests

- Added `snowdensity07_runtime_opt_in` integration test.
- Updated the SNOWDENSITY-03 production-confinement guard to recognize
  SNOWDENSITY-07's contract-authorized runtime opt-in surfaces while still
  rejecting unapproved `physics_bulk` spread.
- Updated older R7G snow tests to set the legacy CoE boundary carry explicitly.

## Ran

- `cargo test --test snowdensity07_runtime_opt_in -- --nocapture`: pass.
- `cargo test --test snowdensity03_physics_bulk_offline_contract -- --nocapture`: pass.
- `cargo test -p openwepp-hillslope-orchestrator --lib r7g_r4g_snow_coupling_mutates_winter_column_snow_state -- --nocapture`: pass.
- `cargo test -p openwepp-hillslope-orchestrator --lib r7g_executor_commits_r4g_winter_column_snow_state_to_lane -- --nocapture`: pass.
- `cargo test -p openwepp-hillslope-orchestrator --lib r7b_constructor_type_size_layout_is_bounded -- --nocapture`: pass.

