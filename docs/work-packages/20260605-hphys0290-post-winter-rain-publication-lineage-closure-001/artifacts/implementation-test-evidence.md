# Implementation Test Evidence

Status: complete
Evidence mode: Static + Ran

Static:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs` now validates and publishes `snow.post_winter_rain_m` from the normalized post-snow hyetograph liquid input.
- `crates/openwepp-runner/src/hillslope/mod.rs` now computes WB13 `RM` from explicit `snow.post_winter_rain_m + snow.routed_melt_m + Irr`.
- `crates/openwepp-runner/src/hillslope/mod.rs` requires `snow.post_winter_rain_m` from the flux surface, so state-only defaults or stale state values cannot satisfy WB13 publication.
- `crates/openwepp-sim-contract/src/units.rs` now declares `snow.post_winter_rain_m` as a typed required non-negative finite depth.
- `crates/openwepp-runner/src/hillslope/mod.rs` trace schema advanced to `v14` with `snow_routed_melt_m` and `snow_post_winter_rain_m` diagnostics.

Ran:

- `cargo fmt`
- `cargo test -p openwepp-runner hphys0290_wb13_rm_publication -- --nocapture` -> pass (`6 passed`).
- `cargo test --test hphys0290_post_winter_rain_publication_contract -- --nocapture` -> pass (`3 passed`).
- `cargo test --test sim_contract_boundary_unit_registry hphys0290_registry_declares_post_winter_rain_flux_metadata -- --nocapture` -> pass (`1 passed`).
- `cargo test --test sim_contract_boundary_unit_registry canonical_registry_resolves_climate_soil_and_snow_runtime_aliases -- --nocapture` -> pass (`1 passed`).
- `cargo test --test sim_contract_boundary_unit_registry hphys0275_registry_marks_only_migrated_aliases_typed_required -- --nocapture` -> pass (`1 passed`).
- `cargo test -p openwepp-runner hphys0289_wb13_rm_publication -- --nocapture` -> pass (`5 passed`).
- `cargo test --test hphys0288_winter_rain_snowmelt_partition_contract -- --nocapture` -> pass (`3 passed`).
- `cargo test --test hphys0289_wb13_rm_snowwater_publication_contract -- --nocapture` -> pass (`2 passed`).
- `cargo test --test sim_contract_boundary_unit_registry -- --nocapture` -> pass (`13 passed`).
- `cargo test --test wb13_daily_water_balance_output_surface_contract -- --nocapture` -> pass (`3 passed`).
- `python3 docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/hphys0290_diagnostics.py --run-root /tmp/hphys0290_full_release_current_20260605T011429Z_postfix` -> pass; runtime `39/39`, semantic pass `0/39`.
- Targeted trace rerun for H1/H7/H39 -> all returned `0`, each wrote `24,837` trace rows under `/tmp/hphys0290_target_traces_current_20260605T011834Z_postfix`.
- Final gates in `/tmp/hphys0290_final_gates_20260605T013019Z_after_nan/status.tsv` all returned `0`: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`, `tools/release/check_authority_suite_antievasion.sh`, and `auth11_required_suite_obligation_guards_contract`.

Disposition: implementation satisfies scoped functional tests and runtime suite, but semantic parity remains open.
