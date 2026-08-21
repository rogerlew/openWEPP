# Owned-file manifest

Status: `FROZEN FOR EXECUTED HOLD`.

`Static:` Historical package paths are read-only evidence and are not in the
write set. No canonical science contract was changed.

`Ran:` final source/test/assurance/package paths touched by this increment are:

```text
Cargo.toml
assurance/v2/identity.lock.json
assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review.lock.json
assurance/v2/transactions/2bc79a057f41ae28fcf68f30989ed194e6b378504d8e01720a3a54cccfcd5271.json
assurance/v2/transactions/74ede41f6e091c0825f7ec7cfd8d207bde466d85d71d4196fb20d9cdb67f8533.json
crates/openwepp-coupled-time/src/clock.rs
crates/openwepp-hillslope-orchestrator/src/canonical_owner_bytes.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime/snow_stage3_shadow.rs
crates/openwepp-hillslope-orchestrator/src/direct_runtime/snow_stage3_v11_scheduler.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/stage3_evaluation_validation_tests/persistent_tests.rs
crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs
crates/openwepp-hillslope-orchestrator/src/lib.rs
crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment.rs
crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs
crates/openwepp-persisted-restart-v1/src/hydrology_restart.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs
tests/integration/land_surface_energy_balance_authority_contract.rs
tests/integration/snow_stage3_legacy_predecessor_bridge_contract.rs
tests/integration/snow_stage3_persistent_accumulation_shadow_contract.rs
tests/integration/snow_stage3_terminal_receiver_authority_contract.rs
tests/integration/snow_stage3_turbulent_operator_reconciliation_contract.rs
tests/integration/snow_stage3_v11_constitutive_boundary_contract.rs
tests/integration/snow_stage3_wind_source_custody_contract.rs
tests/integration/snow_surface_eb03_contract.rs
tests/integration/surface_liquid_hydrology_custody_authority_contract.rs
tests/integration/vegetation_boundary_authority_contract.rs
```

The package artifact files and exact assurance census are also in scope. The
terminal diff reconciliation is the authoritative final path list.
