# PERFARRAY02 WB11 Runoff Pilot

Evidence: Static + Ran.

## Pilot Path

The pilot is behind `OPENWEPP_PERFARRAY02_ARRAY_RUNOFF_PILOT=1`.

Static:

- runner selects `execute_ofe_sequence_with_kernel_indexed_array_runoff_pilot` only when
  the flag parses true;
- accepted false values are absent, empty, `0`, `false`, `FALSE`, `off`, `OFF`;
- accepted true values are `1`, `true`, `TRUE`, `on`, `ON`;
- any other value is a typed runtime failure.

Static anchors:

- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs:2193`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs:2219`

## Real Work Included

The pilot executes the normal `RunoffReconciliation` phase and preserves the scheduler
guard/apply lifecycle:

- seed `ArrayHotState` from the current logical lane surface at the pilot boundary;
- pass empty logical maps plus `Some(&hot_state)` to the kernel request;
- run the real `Wb11HydrologyKernel` phase;
- convert the logical kernel writeback payload to `ArrayWritebackPayload`;
- evaluate with `evaluate_array_writeback`;
- apply with `apply_array_writeback`;
- materialize the post-runoff array back to logical surfaces for downstream non-pilot
  phases and publication.

Static anchors:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:1623`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:1665`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:1741`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:1783`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:1856`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:1878`

Ran:

```text
OPENWEPP_PERFARRAY02_ARRAY_RUNOFF_PILOT=1 cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract
```

Result before final artifact writing: pass.

Ran:

```text
OPENWEPP_PERFARRAY02_ARRAY_RUNOFF_PILOT=1 OPENWEPP_PERFARRAY02_TIMING=1 \
  target/release/openwepp-cli-hill ... --run-file /tmp/perfidx06/runfiles/h2637_same_current.run
```

Result: completed, `h2637_pilot_final 1096.11 229920`.
