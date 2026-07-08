# Line-Count Governance

Status: EXECUTED-COMPLETE
Evidence mode: Ran.

Command:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs crates/openwepp-hillslope-orchestrator/src/lib.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs crates/openwepp-runner/src/hillslope/laned_active.rs tests/integration/laned_shadow_h2637.rs
```

| File | Lines | Disposition |
|------|------:|-------------|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 820 | OK |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | 1249 | OK |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs` | 1214 | OK |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 182 | OK |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 1326 | OK |
| `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | 1445 | OK |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 2749 | WARN |
| `crates/openwepp-runner/src/hillslope/laned_active.rs` | 116 | OK |
| `tests/integration/laned_shadow_h2637.rs` | 492 | OK |

WARN disposition: `00_builders_and_authority.rs` was already a large shared
direct-publication builder. This package added only active mesh/trace config
projection there; no new split is justified inside this package. Future
non-trivial edits to that builder should schedule a decomposition package.
