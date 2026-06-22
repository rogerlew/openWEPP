# Verification

Status: executed-held.

## Static

- Static: R4K producer uses typed `DirectWb14InfiltrationProducerInputs`; no
  production direct use of compatibility `wb12_infiltration` or
  `wb12_depression_storage_delta` as R4K authority was added.
- Static: R4K mutates R4A, WB18, and R4N downstream operands before their
  direct spans execute.
- Static: R4L now consumes R4O hourly saturation carry when present.
- Static: `DirectFrameExecutor` still lacks same-day downstream lane mutation
  from current-lane `ui_SCrunf`/`ui_LfCrf` outputs; this is the next hold.
- Static: line-count governance has no production `.rs` file above `2000`
  lines. Test file `direct_runtime.rs` is `3205` lines and remains a
  pre-existing test-module warning, not production closure blocker for this
  executed hold.

## Ran

- Ran: `cargo fmt --check` passed.
- Ran: `cargo check -p openwepp-runner` passed.
- Ran: `cargo test -p openwepp-hillslope-orchestrator r4k_wb14_producer -- --nocapture`
  passed.
- Ran: `cargo test -p openwepp-hillslope-orchestrator
  r4l_sums_direct_hourly_saturation_carry_when_r4o_has_run -- --nocapture`
  passed.
- Ran: `cargo test -p openwepp-hillslope-orchestrator
  r4l_rejects_conflicting_saturation_handoff_when_hourly_carry_exists -- --nocapture`
  passed.
- Ran: `cargo test -p openwepp-runner
  r7d2_direct_seed_authority_is_lane_indexed_for_multiofe_profiles -- --nocapture`
  passed during the R7D3 execution loop after storage-input changes.
- Ran: `cargo test -p openwepp-runner
  r7c_direct_production_executor_runs_without_compatibility_edges -- --nocapture`
  passed during the R7D3 execution loop.
- Ran: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
  passed.
- Ran: H2637 direct production endpoint exited 0:
  `elapsed=192.90 rss_kb=643724`.
- Ran: H2637 default compatibility endpoint exited 0:
  `elapsed=637.63 rss_kb=227352`.
- Ran: scoped Markdown lint passed:
  `markdown-doc lint --path docs/work-packages/20260622-r7d3-direct-wb14-r4k-infiltration-producer-001 --path docs/work-packages/20260622-r7d4-direct-mofe-dynamic-carry-transfer-001 --path docs/work-packages/README.md --path docs/architecture/array-native-runtime-specification.md`.
- Ran: `git diff --check` passed.
- Not run: full `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`, because package disposition
  is executed-held before full R7D parity closure.
