# R4P/Q/Z - Direct Hydrology Projection And R4 Closure

Status: complete.

Package type: implementation work package / array-native runtime R4 closure.

## Objective

Close R4 by adding a shadow-only direct hydrology projection layer after the
direct hydrology compute chain. R4P/Q/Z must recompute aggregate storage from
direct layer state, assemble typed direct publication-projection operands, and
prove the full daily OFE hydrology direct path without changing public
WB13/WAT/PASS/loss output authority.

## Scope

In scope:

- typed direct aggregate-storage recompute from final direct layer state;
- typed direct hydrology projection structure for direct-owned hydrology
  operands;
- direct state mutation, downstream operands, and shadow projection for the
  R4P/Q/Z closure span;
- focused single-OFE and MOFE-ish shadow comparison fixtures;
- no-compatibility source scan, direct runtime counters, default-disabled H2637
  gate, package evidence, review, verification, commit, push, and burn-down
  tracker update.

Out of scope:

- public WB13/WAT/PASS/loss/schema cutover;
- scheduler activation, default direct activation, or output writer changes;
- new hydrology equations or science-contract amendments unless the
  pre-implementation gate finds missing authority;
- replacing R6 publication-cutover work.

## Authority

- `docs/work-packages/r4-burndown-execplan.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/projection.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4pqz.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `docs/work-packages/20260620-r4pqz-hydrology-projection-r4-closure-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/r4-burndown-execplan.md`
- `docs/ROADMAP.md`

No scheduler, output schema, public output writer, or compatibility runtime edit
is authorized.

## Subagent Authorization

No delegated subagent work is required. Review and verification artifacts may
be completed locally; they must not claim delegated work occurred.

## Phase Plan

1. Record producer selection, process span, operand lineage, contract gate,
   no-compatibility plan, default-disabled gate plan, and line-count baseline.
2. Add a projection-specific direct-runtime module so R4P/Q/Z does not expand
   the root direct-runtime file unnecessarily.
3. Add typed aggregate-storage recompute and projection state/downstream/shadow
   structures.
4. Wire the aggregate executor to run R4P/Q/Z after R4B and before R3B.
5. Add focused fixtures for aggregate-from-layer storage, projection
   anti-aliasing, missing-upstream fail-closed behavior, and counter coverage.
6. Run focused tests, closure gates, no-compatibility scan, scheduler no-diff,
   markdown lint, `git diff --check`, and default-disabled H2637 reps.
7. Complete review, verification, disposition, roadmap/catalog updates, commit,
   push, and ExecPlan progress update.

## Exit Criteria

- R4P/Q/Z includes typed inputs, direct compute, state mutation, downstream
  operands, and shadow projection.
- Aggregate storage is recomputed from final direct layer state, not stale
  scalar storage or publication rows.
- Projection operands cover direct-owned hydrology fields for `Q`, `QOFE`,
  `Ep`, `Es`, `Er`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, snow/frost
  storage terms, carry terms, and profile-capacity placeholders where direct R4
  has authority.
- Public output authority remains compatibility-owned.
- Focused fixtures prove single-OFE projection identity, MOFE transfer/carry
  projection, anti-aliasing, missing upstream failure, and invalid-domain
  failure.
- No-compatibility source scan and scheduler no-diff pass.
- Default-disabled runner counters remain zero; opt-in counters include R4P/Q/Z.
- Full Rust gates, scoped docs lint, `git diff --check`, and default-disabled
  H2637 median `<= 676.67 s` pass.

## Closure Verdict

COMPLETE-R4PQZ-HYDROLOGY-PROJECTION-R4-CLOSURE.

R4P/Q/Z implemented the final R4 shadow-only hydrology projection span. The
span requires direct upstream shadows from R4A, R4B, R4G, R4J, R4M, R4O, and
R4N; recomputes aggregate liquid storage from the final R4N layer vector;
separates frozen-layer and explicit frozen storage; assembles typed direct
projection operands for runoff, ET, percolation, lateral/drainage, snow/frost,
carry, profile-capacity placeholders, and publication comparison fields; and
keeps `public_output_cutover = false`.

R4P/Q/Z preserved the no-publication, no-default-activation, and no-scheduler
boundaries. Focused and aggregate direct-runtime tests, runner counter tests,
no-compatibility scans, full Rust gates, docs lint, `git diff --check`,
protected PASS DuckDB row equivalence, and the default-disabled H2637 timing
gate passed. Final H2637 reps were `645.54 s`, `644.74 s`, and `640.28 s`
with median `644.74 s` against the `<= 676.67 s` threshold.

R4 is closed for the grouped burn-down scope. Public WB13/WAT/PASS/loss
publication cutover remains deferred to a later R6 package; the next work
should plan R5 full OFE-day direct path / endpoint activation readiness before
any publication-authority change.
