# Contract-Test Implementation Evidence

Status: complete
Evidence mode: Static + Ran

Static:
- Added `tests/integration/hphys0287_snow_liquid_partition_guard_contract.rs` and registered it in `Cargo.toml`.
- Test vectors cover material negative SWE before direct-rain partition and dry-cold inactive fallback.
- Test vectors cover negative depth, negative/over-cap density, negative settle count, non-finite depth/density/settle, partial runtime snow-state vector failure, bounded SWE roundoff acceptance, and explicit no-projection/no-snow compatibility.
- Updated `tests/integration/hphys0285_spring_soil_storage_retention_contract.rs` so stale negative SWE is no longer accepted as harmless inactive state.
- Completed the `clim06` frost fixture runtime snow vector so frost seam tests do not rely on partial snow-state projection.

Ran:
- `cargo test --test hphys0287_snow_liquid_partition_guard_contract -- --nocapture` -> pass, 7 tests.
- `cargo test --test hphys0285_spring_soil_storage_retention_contract -- --nocapture` -> pass, 3 tests.
- `cargo test --test hphys0286_layer_retention_wb18_wb17_contract -- --nocapture` -> pass, 2 tests.
- `cargo test --test hphys0284_negative_melt_snowpack_state_contract -- --nocapture` -> pass, 3 tests.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` -> pass, 11 tests after fixture completion.

Pre-implementation gate:
- `/tmp/hphys0287_pre_impl_contract_gate.log` captured the initial failing contract gate before production code edits.
