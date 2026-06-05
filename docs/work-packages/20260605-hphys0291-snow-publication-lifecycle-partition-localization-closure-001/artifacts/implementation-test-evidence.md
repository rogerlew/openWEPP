# Implementation Test Evidence

Status: complete
Evidence mode: ran

Ran focused implementation checks:

```bash
cargo fmt
cargo test --test hphys0291_snow_publication_lifecycle_contract -- --nocapture
cargo test -p openwepp-runner hphys0291_wb13_rm_publication -- --nocapture
cargo test -p openwepp-runner hphys0290_wb13_rm_publication -- --nocapture
cargo test -p openwepp-runner hphys0289_wb13_rm_publication -- --nocapture
```

Results:

- `hphys0291_snow_publication_lifecycle_contract`: `5 passed; 0 failed`.
- `hphys0291_wb13_rm_publication`: `1 passed; 0 failed`.
- `hphys0290_wb13_rm_publication`: `6 passed; 0 failed`.
- `hphys0289_wb13_rm_publication`: `5 passed; 0 failed`.

Static implementation summary:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  now publishes both snow publication fluxes through
  `publish_same_day_snow_publication_fluxes`.
- `crates/openwepp-runner/src/hillslope/mod.rs` now requires WB13
  `snow.routed_melt_m` from the flux surface.
- `crates/openwepp-runner/src/hillslope/mod.rs` already required
  `snow.post_winter_rain_m` from the flux surface after HPHYS0290.
- `crates/openwepp-runner/src/hillslope/mod.rs` now records trace lifecycle
  fields from the flux surface only, so trace localization cannot fall back to
  stale state values.
- `hphys0291_wb13_rm_publication_rejects_state_only_routed_melt` verifies
  state-only routed melt cannot satisfy WB13 publication.

Post-review focused check:

```bash
cargo fmt
cargo test --test hphys0291_snow_publication_lifecycle_contract -- --nocapture
cargo test -p openwepp-runner hphys0291_wb13_rm_publication -- --nocapture
cargo test -p openwepp-runner hphys0290_wb13_rm_publication -- --nocapture
cargo test --test hphys0289_wb13_rm_snowwater_publication_contract -- --nocapture
cargo clippy --workspace --all-targets -- -D warnings
```

Post-review result:

- `hphys0291_snow_publication_lifecycle_contract`: `5 passed; 0 failed`.
- `hphys0291_wb13_rm_publication`: `1 passed; 0 failed`.
- `hphys0290_wb13_rm_publication`: `6 passed; 0 failed`.
- `hphys0289_wb13_rm_snowwater_publication_contract`: `2 passed; 0 failed`.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
