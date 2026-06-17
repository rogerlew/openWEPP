# PERFIDX03 Line-Count Governance

Status: WARN 2026-06-17
Evidence mode: **Ran**

Command:

```text
wc -l crates/openwepp-runner/src/hillslope/symbol_registry_audit.rs \
  crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  crates/openwepp-runner/src/hillslope/intake_lane_setup/lane_setup_helpers.rs \
  crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/scheduler.rs \
  crates/openwepp-hillslope-orchestrator/src/lib.rs \
  crates/openwepp-runner/src/hillslope/tests03/publication/publication_scheduler_pl_activation.rs
```

Result:

```text
  1132 crates/openwepp-runner/src/hillslope/symbol_registry_audit.rs
  2546 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
   444 crates/openwepp-runner/src/hillslope/intake_lane_setup/lane_setup_helpers.rs
  2667 crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs
  2059 crates/openwepp-hillslope-orchestrator/src/scheduler.rs
    66 crates/openwepp-hillslope-orchestrator/src/lib.rs
   155 crates/openwepp-runner/src/hillslope/tests03/publication/publication_scheduler_pl_activation.rs
  9069 total
```

Governance result:

- `WARN`: `00_runner_intake_and_lane_setup.rs`, `scheduler_trace/scheduler_seed_and_runtime.rs`,
  and `scheduler.rs` are above 2000 lines.
- No touched file exceeds the 3000-line required-refactor threshold.
- No line-count refactor is required before this HOLD disposition.
