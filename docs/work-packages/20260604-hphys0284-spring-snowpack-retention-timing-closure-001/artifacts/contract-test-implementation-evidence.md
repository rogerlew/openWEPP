# Contract-Test Implementation Evidence

Status: complete
Evidence mode: Static + Ran

## Static: Test Added

- Added `tests/integration/hphys0284_negative_melt_snowpack_state_contract.rs`.
- Registered it in `Cargo.toml` as `hphys0284_negative_melt_snowpack_state_contract`.
- Updated `tests/integration/clim05_snow_runtime_kernel_contract.rs` so the HPHYS0269 negative-melt vector asserts corrected state loss rather than routed-net SWE closure.
- Added both mixed-melt branches required by `INV-SNOWFREEZE-019`: net-positive routed melt and net-nonpositive melt undoing positive hourly loss.
- Added depth and density assertions so the regression covers carried snow-depth/SWE lineage, not only exported SWE.

## Ran: Red/Green Evidence

- Pre-production gate: `cargo test --test hphys0284_negative_melt_snowpack_state_contract -- --nocapture` failed before production edits.
- Red failure: `runtime_swe=0.33736604`, expected `0.32989844`, `routed_melt=0.01263396`, `raw_positive=0.01636776`, `raw_negative=-0.00373380`.
- Post-production gate: `cargo test --test hphys0284_negative_melt_snowpack_state_contract -- --nocapture` passed, `2 passed; 0 failed`.
- Post-production CLIM05 snow suite: `cargo test --test clim05_snow_runtime_kernel_contract -- --nocapture` passed, `9 passed; 0 failed`.
