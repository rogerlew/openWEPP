# R5C - Direct Decomposition And Residue Transitions

Status: complete; pushed.

Package type: implementation work package / array-native runtime R5 phase
promotion.

## Objective

Promote `DecompositionTransition` and `ResiduePartitionTransition` from
compatibility-owned lifecycle hold phases into explicit direct phases. R5C
must provide typed inputs, direct compute, state mutation, downstream operands,
and shadow projection for both phases without changing public output authority
or default runtime selection.

## Scope

In scope:

- direct `DecompositionTransition` phase state over typed decomposition context
  available to the direct runtime;
- direct `ResiduePartitionTransition` phase state over typed residue/cover
  context available to the direct runtime;
- R5B upstream phase requirement and tests;
- focused tests for phase identity, direct compute, state mutation, downstream
  operands, shadow projection, invalid-domain guards, anti-alias vectors, and
  missing-upstream failure;
- lifecycle phase-status updates so decomposition/residue no longer report as
  R5 hold phases;
- runner counter updates for additional direct phase spans;
- package evidence, review, verification, catalog updates, burn-down tracker
  updates, commit, and push.

Out of scope:

- annual/perennial growth transition migration;
- hydrology equation changes;
- publication cutover to WB13/WAT/PASS/loss/manifest outputs;
- scheduler phase-order changes;
- default direct activation;
- R6 public-output authority.

## Authority

- `docs/work-packages/r5-burndown-execplan.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`

No science-contract amendment is expected unless implementation needs new
process equations, residue/decomposition bounded canonicalization, or guard
tolerance. If that occurs, stop and record `HOLD`.

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/decomposition.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime*.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `docs/work-packages/20260620-r5c-decomposition-residue-direct-transitions-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/r5-burndown-execplan.md`
- `docs/ROADMAP.md`

No scheduler, output schema, output writer, public CLI activation, or
compatibility request/writeback edit is authorized.

## Subagent Authorization

No delegated subagent work is required. Review and verification artifacts may
be completed locally; they must not claim delegated work occurred.

## Phase Plan

1. Record scope selection, process span, operand lineage, contract gate,
   no-compatibility plan, default-disabled plan, line-count baseline, and
   kernel-profile checklist.
2. Discover canonical decomposition/residue contract authority and current
   compatibility handoff surfaces.
3. Add a `direct_runtime/decomposition.rs` module for R5C phase types and
   methods.
4. Add direct decomposition/residue phase state to `DirectDayFrame`.
5. Add executor phase calls after R5B `StorageBounds` and before growth hold
   phases while retaining public compatibility output authority.
6. Update phase status counts so only annual/perennial growth phases remain
   `Hold`.
7. Add focused direct-runtime and runner counter tests.
8. Run focused tests, closure gates, H2637 default-disabled reps, review,
   verification, disposition, roadmap/catalog updates, commit, push, and
   burn-down progress update.

## Exit Criteria

- `DecompositionTransition` has typed inputs, direct compute, state mutation,
  downstream operands, and shadow projection.
- `ResiduePartitionTransition` has typed inputs, direct compute, state
  mutation, downstream operands, and shadow projection.
- R5B upstream state is required and tested.
- Direct decomposition/residue phases fail closed on missing upstream,
  nonfinite values, negative pool/domain values, and ambiguous or missing
  required active context in current scope.
- No public output authority changes.
- No direct-runtime compatibility request/writeback/symbol access is added.
- Default-disabled runner counters remain zero.
- Explicit opt-in runner counters include R5C additional direct phase spans
  over all fixture days.
- Focused tests, no-compatibility scan, scheduler diff review, full Rust gates,
  docs lint, `git diff --check`, and default-disabled H2637 median
  `<= 676.67 s` pass.

## Closure Verdict

`COMPLETE-R5C-DECOMPOSITION-RESIDUE-DIRECT-TRANSITIONS`.

Pushed commit: `efdf6710`.
