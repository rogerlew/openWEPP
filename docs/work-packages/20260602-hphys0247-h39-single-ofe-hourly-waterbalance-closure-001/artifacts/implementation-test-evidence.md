# Implementation/Test Evidence

Status: updated

Evidence mode: static + ran

Static:
- Changed SIMIMPL28 winter hourly forcing emission so legacy runtime triggers
  drive execution: existing SWE, existing frost state, or average daily
  temperature below `0 degC`; sidecar presence remains provenance/override
  metadata only.
- Changed hydrology active snow coupling to use the same runtime trigger
  predicate instead of `snow.options.snow_file_present`.
- Changed runner winter coupling provenance so `winter.active` reports runtime
  trigger activity while `snow_file_present` remains sidecar discoverability.
- Changed WB19 lateral transfer to use baseline bottom-contiguous `meblfc`
  layer selection, active-layer `tdvv` cap, and per-layer `fffx` conductivity
  weighting.
- Post-review contract clarification: explicitly documented and tested the
  baseline `solwpv < 2006` post-aggregation `fffx` multiplier branch that was
  already encoded by production and external-authority fixtures.
- Refreshed WB19 external-authority fixture expectations and runner manifest
  expectations so sidecar/runfile snow override presence does not imply
  winter execution activation.

Ran:
- `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::climate_runtime_surface_with_context -- --nocapture`
  passed `4` tests.
- `cargo test --test clim05_snow_runtime_kernel_contract -- --nocapture`
  passed `6` tests.
- `cargo test --test wb19_lateral_drainage_physics_kernel_contract -- --nocapture`
  passed `11` tests.
- `cargo test --test hphys0221_wb19_water_yield_fcdep_coupling_contract -- --nocapture`
  passed `4` tests.
- `cargo test --test hphys0227_wb19_fcwp_coca_watyld_authority_contract -- --nocapture`
  passed `2` tests.
- `cargo test -p openwepp-runner --test simimpl04_wepp_ui_mode_closure_contract -- --nocapture`
  passed `1` test.
- `cargo test --workspace` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `bash tools/release/check_authority_suite_antievasion.sh` passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
  passed `2` tests.
- `cargo deny check` passed with warnings for duplicate lockfile entries and
  unmatched license allowances.
- `cargo fmt --check` passed after `cargo fmt`.
- `cargo build -p openwepp-runner --bin openwepp-cli-hill` passed.
- H39 runner and semantic comparator ran under
  `/tmp/hphys0247_20260602T070132Z_final`; comparator remained
  `semantic_pass=false`.
