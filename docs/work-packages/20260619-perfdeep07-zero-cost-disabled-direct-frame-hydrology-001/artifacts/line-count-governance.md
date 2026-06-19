# PERFDEEP07 Line-Count Governance

Status: queued.
Evidence mode: not-run.

## Requirement

Record `.rs` line-count governance before closure. Files at or above 2000 lines
are `WARN`; files at or above 3000 non-exempt lines require refactor before
implementation closure or an explicit package exception with a sunset plan.

PERFDEEP06 measured `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
at 3177 lines. PERFDEEP07 touches scheduler scope, so it must disposition this
before closure.
