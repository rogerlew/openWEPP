# PERFIDX02 Line-Count Governance

Status: PASS 2026-06-16
Evidence mode: **Ran**

Command:

```text
wc -l crates/openwepp-runner/src/hillslope/indexed_shadow_surface.rs \
  crates/openwepp-runner/src/hillslope/symbol_registry_audit.rs \
  crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs \
  crates/openwepp-kernel-contract/src/lib_mod/core_types.rs \
  crates/openwepp-kernel-contract/src/lib.rs
```

Result:

```text
   592 crates/openwepp-runner/src/hillslope/indexed_shadow_surface.rs
   956 crates/openwepp-runner/src/hillslope/symbol_registry_audit.rs
  2373 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
  2634 crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs
  1948 crates/openwepp-kernel-contract/src/lib_mod/core_types.rs
   503 crates/openwepp-kernel-contract/src/lib.rs
  9006 total
```

No touched file crosses the 3,000-line required-refactor threshold.
