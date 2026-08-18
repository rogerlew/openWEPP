# Line-Count Governance

Status: PASS on the exact current worktree.

The package hard stop is 3,000 lines for every changed Rust source file. The
largest changed files are:

| Lines | Path | Disposition |
|---:|---|---|
| 2,974 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs` | PASS; terminal comparisons and ending validation extracted. |
| 2,973 | `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs` | PASS. |
| 2,960 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_tests.rs` | PASS; terminal regressions extracted. |
| 2,897 | `crates/openwepp-runner/src/hillslope/03_tests.rs` | PASS; touched only for workspace warnings-denied lint remediation. |
| 2,984 | `crates/openwepp-land-surface-energy/src/transaction.rs` | PASS. |
| 2,620 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs` | PASS. |
| 2,588 | `crates/openwepp-land-surface-energy/src/solver.rs` | PASS; its test body was extracted to `solver_tests.rs`. |

All other changed Rust files are at most 2,762 lines. No changed Rust file is
at or above 3,000 lines. The repository contains an unchanged assurance source
above 3,000 lines; it is outside this package diff and does not affect this
gate.

Evidence command:

```text
for each tracked or untracked changed *.rs path relative to 70d855ff6...:
wc -l; sort descending
```

No lint suppression was introduced to obtain these counts.
