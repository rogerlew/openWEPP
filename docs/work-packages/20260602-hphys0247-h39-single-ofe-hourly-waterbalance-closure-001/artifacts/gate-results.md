# Gate Results

Status: hold

Evidence mode: ran

Static:
- External-authority fixture hashes and provenance were refreshed for WB19
  fixture expectations affected by the HPHYS0247 `fffx` authority.

Ran:
- `cargo fmt --check`: passed after `cargo fmt`.
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `bash tools/release/check_authority_suite_antievasion.sh`: passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`:
  passed `2` tests.
- `git diff --check`: passed.
- `cargo deny check`: passed with warnings for duplicate lockfile entries and
  unmatched license allowances; final status reported advisories, bans,
  licenses, and sources `ok`.
- `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::climate_runtime_surface_with_context -- --nocapture`:
  passed `4` tests.
- `cargo test --test clim05_snow_runtime_kernel_contract -- --nocapture`:
  passed `6` tests.
- `cargo test --test wb19_lateral_drainage_physics_kernel_contract -- --nocapture`:
  passed `10` tests.
- `cargo test --test hphys0227_wb19_fcwp_coca_watyld_authority_contract -- --nocapture`:
  passed `2` tests after fixture refresh.
- `cargo test -p openwepp-runner --test simimpl04_wepp_ui_mode_closure_contract -- --nocapture`:
  passed `1` test after manifest expectation refresh.
- H39 comparator:
  `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py` ran and
  produced `/tmp/hphys0247_20260602T070132Z_final/reports/H39.semantic.json`
  with `semantic_pass=false`.
