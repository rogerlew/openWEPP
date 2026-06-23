# R7F Kickoff Agent Prompt

You are executing
`docs/work-packages/20260623-r7f-direct-day-input-hot-loop-isolation-001`.

First action: close defect
`HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE`.

Replace the production direct interleaved day-input hot-loop dependency on
`DirectPublicationDayInputBuilder` with typed direct day-input/state
projection. The production direct day/OFE loop must not construct, clone, or
merge `HillslopeWritebackSurface`, `BoundarySymbol`, `BoundaryValue`, symbol
registries, indexed surfaces, dense refreshes, dirty flushes, or compatibility
wrappers for day-input construction.

Do not stop after renaming the builder, suppressing a counter, or adding a
wrapper around the same compatibility surface dependency. If the next failure
is in-envelope, fix it and rerun the gate. Close in HOLD only at a named
boundary accepted by review evidence.

Required gates before complete disposition:

- `cargo test -p openwepp-runner r7 -- --nocapture`
- `cargo test -p openwepp-runner r6 -- --nocapture`
- source scan proving production direct no longer invokes the compatibility
  day-input builder in the hot loop
- source scan proving production direct manifest counters report
  `compatibility_edge_invocations = 0` because the hot edge is gone
- `cargo fmt --check`
- `git diff --check`
- scoped Markdown lint over this package and touched catalogs

If complete implementation is claimed, also run the full Rust closure loop
required by `docs/work-packages/AGENTS.md` unless a legitimate package hold is
declared before closure.
