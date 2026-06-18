# PERFARRAY02 Line-Count Governance

Evidence: Ran.

Command:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/scheduler.rs \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs \
  crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs \
  crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  crates/openwepp-kernel-contract/src/lib_mod/core_types.rs \
  crates/openwepp-hillslope-orchestrator/src/perfarray02_timing.rs \
  tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs
```

Result:

| File | Lines | Governance |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | 2826 | WARN, below 3000 required-refactor threshold |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs` | 2346 | WARN |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 2699 | WARN |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2420 | WARN |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs` | 2720 | WARN |
| `crates/openwepp-hillslope-orchestrator/src/perfarray02_timing.rs` | 162 | OK |
| `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs` | 1350 | OK |

Disposition:

The package stayed below the 3000-line required-refactor threshold. The WARN files
remain large pre-existing hot files; PERFARRAY02 added a small timing module rather than
putting timing counters into `scheduler.rs`. Follow-on decomposition should continue under
the existing MOFE line-count split queue item, not inside this performance NO-GO package.
