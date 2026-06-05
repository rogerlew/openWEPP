# Contract-Test Implementation Evidence

Status: complete
Evidence mode: Static/Ran

## Tests Added

Static:

- Registered `tests/integration/hphys0289_wb13_rm_snowwater_publication_contract.rs` in `Cargo.toml`.
- Added source-level contract guards rejecting the old `precipitation_m + runtime_swe_before_m - runtime_swe_m + irrigation_m` proxy and requiring `snow.routed_melt_m` publication/consumption.
- Added runner behavior tests in `crates/openwepp-runner/src/hillslope/mod.rs` for:
  - snow-active `RM = routed wmelt + irrigation`,
  - missing `snow.routed_melt_m` fail-closed behavior,
  - warm-rain/no-snow `RM = prcp + irrigation`,
  - flux `snow.routed_melt_m` taking precedence over stale state shadowing,
  - negative routed melt fail-closed behavior.

## Final Test Evidence

Ran:

- `cargo test -p openwepp-runner hphys0289_wb13_rm_publication -- --nocapture`
  - Result: pass, `5 passed; 0 failed`.
- `cargo test --test hphys0289_wb13_rm_snowwater_publication_contract -- --nocapture`
  - Result: pass, `2 passed; 0 failed`.
- Final broad workspace run includes `openwepp-runner` unit result `62 passed; 0 failed`, including all five HPHYS0289 behavior tests.
  - Log: `/tmp/hphys0289_final_broad_gates_20260605T001506Z.log`.
