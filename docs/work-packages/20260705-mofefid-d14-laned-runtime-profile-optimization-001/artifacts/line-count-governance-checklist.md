# Line-Count Governance Checklist

Status: **EXECUTED (D14-S5)**.

Evidence mode: **Ran** (`wc -l` on the final tree).

| File | Before (`git show HEAD | wc -l`) | After (final, post-review-fixes) | Status |
|---|---|---|---|
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` | 1,002 | 1,184 | OK (< 2,000 WARN) |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs` | 445 | 483 | OK |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs` | 329 | 401 | OK |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/profile.rs` | — (new) | 178 | OK |
| `crates/openwepp-runner/src/hillslope/laned_shadow.rs` | 505 | 681 | OK |
| `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | 1,260 | 1,262 | OK |

No file crosses the 2,000-line WARN threshold; no 3,000-line refactor
obligation is triggered. Function-level `clippy::too_many_lines` pressure was
resolved structurally (the `SampleRecorder` extraction in the solver and the
`build_cascade_segments` / `build_day_rate_series` extraction in the runner
collector), not by `#[allow]`.
