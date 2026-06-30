# Verification

Evidence mode: Ran/Static.

## Ran

- `cargo fmt --check` - PASS.
- `cargo check -p openwepp-runner` - PASS.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` - PASS.
- Baseline release build from detached clean worktree at `5b139058` - PASS.
- Baseline H2637:
  `/tmp/openwepp-baseline-carrier/target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637 --run-file /tmp/typed-direct-carrier-identity/base/h2637.run --output-dir /tmp/typed-direct-carrier-identity/base/output --manifest-path /tmp/typed-direct-carrier-identity/base/output/manifest.json`.
  Result: exit `0`, RSS `110916 KiB`.
- Current H2637:
  `target/release/openwepp-cli-hill --run-dir /tmp/perfho01/run-dirs/h2637 --run-file /tmp/typed-direct-carrier-identity/after/h2637.run --output-dir /tmp/typed-direct-carrier-identity/after/output --manifest-path /tmp/typed-direct-carrier-identity/after/output/manifest.json`.
  Result: exit `0`, RSS `91796 KiB`.
- H2637 byte comparison: `H2637.hbp`, `H2637.loss.json`,
  `H2637.plot.parquet`, `H2637.wat.parquet`, and `H2637.pass.parquet` are
  byte-identical.
- H2637 manifest counters: baseline and current both selected
  `direct-production-executor`, `compatibility_edge_invocations=0`,
  `day_frame_constructions=235961`, `erod14_wave2_kernel_status_seen=true`,
  `scheduler_kernel_executed=false`.
- Multi-OFE/Wave-2: `cargo test -p openwepp --test
  cli03_runner_contract_derived_tests
  cli03_mofe03_multiofe_runfile_executes_wave2_without_manual_symbol_injection
  -- --nocapture` - PASS.

## Static

Stage 1 remains blocked by surface-seeded direct authority. Static grep still
finds production direct setup references to `HillslopeWritebackSurface` in the
lane seed and day-input builder paths.

Ran:

```text
rg -n "direct_production_lane_seed_surfaces|direct_production_execution_runtime_surface|seed_direct_production_lane_constructor_inputs|DirectProductionDayInputBuilder::new|climate_context_surface: &runtime_surface|lane_seed_surfaces" crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
```

Result: direct production still enters
`direct_production_lane_seed_surfaces`, `direct_production_execution_runtime_surface`,
`seed_direct_production_lane_constructor_inputs`, and
`DirectProductionDayInputBuilder::new` with `HillslopeWritebackSurface` seed
authority.

Ran:

```text
rg -n "HillslopeWritebackSurface" crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers | wc -l
```

Result: `59` remaining runner direct-production/direct-publication helper
references in this narrowed source slice.
