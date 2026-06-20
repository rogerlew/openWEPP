# R5B - Direct Normalization And Storage-Bounds Phases

Status: complete. Pushed commit: `27de814c` on branch `main`.

Package type: implementation work package / array-native runtime R5 phase
promotion.

## Objective

Promote `Normalization` and `StorageBounds` from lifecycle hold/partial spans
into explicit direct phases. R5B must provide typed inputs, direct compute,
state mutation, downstream operands, and shadow projection for both phases
without changing public output authority or default runtime selection.

## Scope

In scope:

- direct `Normalization` phase over current direct day forcing, lane transfer
  buffers, current direct storage, and publication placeholder state;
- direct `StorageBounds` phase over normalized storage and scalar
  water-domain constraints available in the direct frame;
- focused tests for phase identity, direct compute, state mutation, downstream
  operands, shadow projection, invalid-domain guards, and missing-upstream
  failure;
- updated lifecycle phase-status counts so `StorageBounds` is no longer an R5
  hold phase;
- runner counter updates for the additional direct phase span;
- package evidence, review, verification, catalog updates, burn-down tracker
  updates, commit, and push.

Out of scope:

- decomposition/residue/growth phase migration;
- new process-physics formulas;
- publication cutover to WB13/WAT/PASS/loss/manifest outputs;
- scheduler phase-order changes;
- default direct activation;
- R6 public-output authority.

## Authority

- `docs/work-packages/r5-burndown-execplan.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`

No science-contract amendment is expected unless implementation needs a new
storage-domain normalization or guard tolerance. If that occurs, stop and
record `HOLD`.

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/normalization.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `docs/work-packages/20260620-r5b-normalization-storage-bounds-direct-phases-001/**`
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
2. Add a `direct_runtime/normalization.rs` module for R5B phase types and
   methods.
3. Add direct `Normalization` and `StorageBounds` phase state to
   `DirectDayFrame`.
4. Replace the executor's R3A lifecycle call with direct R5B normalization and
   storage-bounds phase calls while retaining the existing R3A span method for
   legacy focused tests.
5. Update phase status counts so only decomposition/residue/growth phases
   remain `Hold`.
6. Add focused direct-runtime and runner counter tests.
7. Run focused tests, closure gates, H2637 default-disabled reps, review,
   verification, disposition, roadmap/catalog updates, commit, push, and
   burn-down progress update.

## Exit Criteria

- `Normalization` has typed inputs, direct compute, state mutation, downstream
  operands, and shadow projection.
- `StorageBounds` has typed inputs, direct compute, state mutation, downstream
  operands, and shadow projection.
- `Normalization -> StorageBounds -> DecompositionTransition` identity is
  explicit in lifecycle status counts.
- Direct storage bounds fail closed on nonfinite or negative storage/domain
  values and on missing normalization upstream.
- No public output authority changes.
- No direct-runtime compatibility request/writeback/symbol access is added.
- Default-disabled runner counters remain zero.
- Explicit opt-in runner counters include the R5B additional direct phase span
  over all fixture days.
- Focused tests, no-compatibility scan, scheduler diff review, full Rust gates,
  docs lint, `git diff --check`, and default-disabled H2637 median
  `<= 676.67 s` pass.

## Closure Verdict

`COMPLETE-R5B-NORMALIZATION-STORAGE-BOUNDS-DIRECT-PHASES`.

R5B promotes `Normalization` and `StorageBounds` to explicit direct executor
phase calls with typed inputs, direct compute, state mutation, downstream
operands, and shadow projection. `StorageBounds` intentionally validates the
scalar direct storage/domain state currently available in the direct frame; it
does not invent new layer-capacity physics or change the existing R4 layer
producers.
