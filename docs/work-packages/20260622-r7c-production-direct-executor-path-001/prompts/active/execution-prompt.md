# R7C Execution Prompt

Execute `docs/work-packages/20260622-r7c-production-direct-executor-path-001/package.md`.

Do not stop after adding a runtime enum or diagnostic-only branch. R7C closure
requires an explicit opt-in production direct mode that constructs typed direct
frames, enters `DirectFrameExecutor`, runs the full run/lane/day loop, records
nonzero direct phase and publication counters, records zero compatibility-edge
invocations, and preserves default compatibility and R6J cutover behavior.

If direct public output authority is incomplete, do not relabel that as R7C
completion. Record the R7D boundary explicitly while still proving the R7C
executor path and counters.
