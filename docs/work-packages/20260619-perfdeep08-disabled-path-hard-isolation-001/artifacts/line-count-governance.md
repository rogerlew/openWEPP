# Line-Count Governance

Status: passed for HOLD scope.
Evidence mode: Static/Ran.

## Touched Rust Files

No Rust file is retained in the final diff.

Temporary candidate line counts:

```text
3179 crates/openwepp-hillslope-orchestrator/src/scheduler.rs
 608 crates/openwepp-runner/src/hillslope/indexed_shadow_surface.rs
 137 crates/openwepp-runner/src/hillslope/scheduler_trace/perfdeep02_frame_roundtrip.rs
```

`scheduler.rs` was reverted before timing because touching a 3000+ line file
would require a split or explicit closure plan before completion. The remaining
runner hook edits were timed, rejected, and reverted.

## Gate

PASS for HOLD scope. No 3000+ Rust file remains touched.
