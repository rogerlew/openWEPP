# R2A Review Agent A

Status: complete.
Evidence mode: Static + Ran.

Review focus:

- direct type boundary adequacy;
- no-compatibility proof legitimacy;
- default-disabled regression proof;
- protected-boundary integrity;
- Gate Evidence Non-Deferral.

Review agent A ran the focused orchestrator R2A tests and runner R2A tests.
The initial runner test run exposed a real race and failed; the finding was
fixed and the focused runner tests were rerun successfully.

| Finding | Severity | Disposition | Rationale |
|---|---|---|---|
| Runner R2A audit tests reset process-global counters outside the runner execution lock, making the default-disabled proof race-prone. | High | Fixed | `reset_direct_runtime_audit_counters()` now runs inside `execute_fixture_run_with_runtime_selection` after the lock is acquired. Individual test-level resets were removed. Rerun `cargo test -p openwepp-runner r2a_ -- --nocapture` passed. |
| Reserved forbidden-compatibility audit fields were tautological because no forbidden compatibility entrypoints incremented them. | High | Fixed | Removed `forbidden_compatibility_calls` and `compatibility_surface_constructions` from direct audit state and tests. Package proof now uses static source/call-graph evidence for forbidden calls and runtime counters only for direct skeleton construction/execution. |
| `DirectExecutionReport.audit` carried a process-global snapshot that could be polluted by prior direct skeleton runs. | Medium | Fixed | Removed the audit snapshot from `DirectExecutionReport`; tests read the explicit audit API after reset. |

Residual risk:

- No dynamic compatibility hot-loop instrumentation was added. This is
  intentional: forbidden-call absence is proven by direct-runtime source scan
  and no scheduler diff, avoiding a new default-disabled tax.
