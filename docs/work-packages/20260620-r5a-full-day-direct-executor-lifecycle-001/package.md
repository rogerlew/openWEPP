# R5A - Full-Day Direct Executor Lifecycle

Status: complete pending commit and push.

Package type: implementation work package / array-native runtime R5 lifecycle
prerequisite.

## Objective

Turn the R2/R4 direct skeleton into a real direct run/lane/day lifecycle while
keeping phase math unchanged. R5A must execute the existing direct spans across
every day and OFE lane, carry typed lane state into each direct day frame,
commit end-of-day typed state back to the lane frame, and report canonical
phase status/counters without changing public output authority.

## Scope

In scope:

- direct executor iteration over `day_count * lane_count` direct day frames;
- explicit `DirectLaneFrame -> DirectDayFrame` persistent-state handoff;
- explicit `DirectDayFrame -> DirectLaneFrame` end-of-day commit semantics;
- direct report counters for day-frame commits and canonical 14-phase status;
- focused orchestrator tests for full lifecycle, canonical phase status, and
  fail-closed invalid day/lane indexing;
- runner audit-counter tests proving default-disabled zero cost and opt-in
  all-day/all-lane execution with one compatibility-edge handoff;
- package evidence, review, verification, catalog updates, burn-down tracker
  updates, commit, and push.

Out of scope:

- new process equations or direct phase math;
- direct ownership of `StorageBounds`, decomposition/residue, or growth
  transition phase internals;
- public WB13/WAT/PASS/loss/manifest/schema cutover;
- default direct activation;
- scheduler phase-order changes;
- R6 publication authority.

## Authority

- `docs/work-packages/r5-burndown-execplan.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`

No science-contract amendment is expected because R5A changes runtime lifecycle
plumbing only. If implementation uncovers a process-physics or guard authority
gap, this package must stop and record `HOLD`.

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `docs/work-packages/20260620-r5a-full-day-direct-executor-lifecycle-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/r5-burndown-execplan.md`
- `docs/ROADMAP.md`

No output writer, output schema, kernel request, compatibility scheduler,
symbol registry, or public CLI activation edit is authorized.

## Subagent Authorization

No delegated subagent work is required. Review and verification artifacts may
be completed locally; they must not claim delegated work occurred.

## Phase Plan

1. Record scope selection, process-span contract, operand lineage,
   pre-implementation contract gate, no-compatibility proof plan,
   default-disabled gate plan, endpoint/RSS plan, kernel-profile checklist, and
   line-count baseline.
2. Add explicit lane-state handoff and end-of-day commit APIs to the direct
   runtime.
3. Widen `DirectFrameExecutor::run_skeleton` from lane-only day `0` execution
   to all days and lanes.
4. Add lifecycle report fields for day-frame commit count and canonical
   phase-status counts, including hold status for non-hydrology phases not yet
   direct-owned.
5. Update focused orchestrator tests and runner counter tests for
   all-day/all-lane execution.
6. Run focused tests, no-compatibility scan, scheduler diff review, Rust gates,
   scoped docs lint, `git diff --check`, and default-disabled H2637 reps.
7. Complete review, verification, disposition, roadmap/catalog updates,
   burn-down progress update, commit, and push.

## Exit Criteria

- Direct executor constructs exactly `day_count * lane_count` direct day
  frames.
- Each day frame is seeded from its lane's persistent typed state and each day
  commits typed state back to the lane.
- Existing R3/R4 direct spans still run without compatibility requests,
  writeback payloads, symbol registries, hot tables, indexed surfaces, dense
  refreshes, or dirty flushes.
- Missing non-hydrology phases are visible in canonical phase-status counts as
  typed hold/no-op lifecycle entries, not hidden compatibility calls.
- Direct report records mode, lane count, day count, planned phase count,
  canonical phase-status counts, direct phase-entry counts, and day-frame
  commit count.
- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit direct opt-in runner fixture records direct execution over all
  fixture days and lanes and exactly one compatibility-edge handoff.
- Public outputs remain compatibility-authoritative.
- Focused tests, no-compatibility scan, scheduler diff review, full Rust gates,
  scoped docs lint, `git diff --check`, and default-disabled H2637 median
  `<= 676.67 s` pass.

## Closure Verdict

COMPLETE-R5A-FULL-DAY-DIRECT-EXECUTOR-LIFECYCLE.

R5A implemented the full direct run/lane/day lifecycle prerequisite without
changing phase math or public output authority. `DirectFrameExecutor` now
executes existing direct spans across every day and lane, seeds day frames from
persistent lane state, commits end-of-day state back to lanes, records
day-frame commit counts, and reports canonical phase status counts. The five
remaining non-hydrology phases reserved for R5B-D are explicit `Hold` statuses.

No scheduler phase-order, default runtime selection, output writer, output
schema, or public direct-only CLI activation changed. Focused tests, full Rust
gates, no-compatibility scan, docs lint, protected output comparison, and H2637
default-disabled timing passed. Final H2637 default-disabled reps were
`643.98 s`, `647.95 s`, and `643.45 s` with median `643.98 s` against the
`<= 676.67 s` threshold.
