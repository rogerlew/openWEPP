# Implementation Test Evidence

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-11

## Implementation Summary

Static:

- Replaced the retired freeze-index depth proxy in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
  with an hourly heat-flow update. Surface heat loss through the
  snow/residue/frozen path is published separately from lower unfrozen-soil
  heat flow (`Quf`); their signed balance advances or thaws `Dfrost` through
  latent-heat increments. Depth state is bounded by the physical soil profile
  depth from `solthk`, not by the former `0.20 m` model cap.
- Preserved the `0.20 m` CLIM06 tilled-layer conductivity scale as a
  conductivity input only. It no longer bounds model frost depth.
- Added a fail-closed guard so newly frozen-water storage cannot exceed
  available liquid `wb11_soil_water`.
- Added thaw-side liquid credit so reductions in `frost.runtime_ws_frz`
  restore prior frozen storage to `wb11_soil_water`.
- Added profile-depth propagation through `FrostCouplingOutcome` so downstream
  frost writeback validates against the same physical bound.
- Published runtime `frost.runtime_frdp_m` to the WAT surface as `frdp` in
  millimetres, with schema metadata, unit-registry aliases, profile-bound
  validation, and WAT interchange dataset version `1.4`.
- Bound WAT `frozwt` publication to
  `frost.runtime_frwatc_frozen_water_after_m`, added initial projection of
  the `frost.runtime_frwatc_*` diagnostics, and added a guard so missing
  exchange-store publication fails closed.
- Replaced the scalar `frdp * theta_active` frozen-water store with per-layer
  `wb18_perc_frozen_depth_####` and `wb18_perc_frzw_####` state. WAT `frozwt`
  now resolves the legacy `Σ soilf(i)` store from active layer state:
  `Σ(wb18_perc_frzw_#### + thetdr_#### * wb18_perc_frozen_depth_####)`.
- Removed stale runner/orchestrator constants that represented the retired
  model-depth cap.

## Focused Post-Implementation Tests

Ran:

- `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`
  - Result: passed, 19 tests, including layered-store rejection of scalar
    frozen-water equivalence and per-layer freeze update coverage.
- `cargo test --test cli04_runner_wat_parquet_contract_derived_tests -- --nocapture`
  - Result: passed, 2 tests.
- `cargo test --test sim_contract_boundary_unit_registry -- --nocapture`
  - Result: passed, 14 tests.
- `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract -- --nocapture`
  - Result: passed, 8 tests.
- `cargo test -p openwepp-runner --lib fdhp01_wb13 -- --nocapture`
  - Result: passed, 3 tests, including the missing exchange-store guard.
- `cargo test -p openwepp-runner --lib
  hphys0203_wb13_soil_water_total_preserves_watcon_alias -- --nocapture`
  - Result: passed, proving WAT `frozwt` follows
    `frost.runtime_frwatc_frozen_water_after_m` when it differs from
    `runtime_ws_frz`.
- `cargo test -p openwepp-hillslope-output schema_includes_required_dataset_metadata_keys -- --nocapture`
  - Result: passed, 1 test.

## Workspace Gates

Ran:

- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed.

## Notes

The landed tests prove narrow contract-critical implementation boundaries:
retired cap removed, separate `Qsrf`/`Quf` publication, warm heat-flow thaw
accepted for deep prior frost, frozen-water overdraw fail-closed, WAT `frdp`
value publication/profile bound/versioning, and unit registry authority
present. The layered continuation adds tests proving the published frozen store
is no longer the scalar `frdp * theta` quantity and that layer `frzw` changes
with freeze/thaw.

Post-review cohort validation supersedes the earlier "unavailable FDMC01
manifest" caveat: direct measurement was available and was run. The layered
cohort gate at `/tmp/fdhp01_layered_store_20260611T080722Z` produced `43/43`
clean exits, closed the annual `Total-Soil + frozwt` identity to numerical
noise (`1.2683574368566042e-07 mm` max abs residual), and broke the exact
`frozwt/frdp` scalar relation. This implementation remains held only on the D3
depth/duration parity gate: max-depth mean `1782.0379909380451 mm` versus
legacy `414.22093023255815 mm`, median depth correlation
`-0.27756218032931956`.
