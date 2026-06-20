# R5 Full OFE-Day Direct-Path Burn-Down ExecPlan

This ExecPlan is a living document. The sections `Progress`,
`Surprises & Discoveries`, `Decision Log`, and `Outcomes & Retrospective` must
be kept up to date as work proceeds.

Maintain this plan in accordance with `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`,
`docs/architecture/array-native-runtime-specification.md`, and the relevant
science-contract local playbooks. R5 work is kernel-affecting: every package
that edits runtime, scheduler, runner, or kernel code must read
`docs/specifications/science-contracts/AGENTS.md` before production edits.

## Purpose / Big Picture

R5 is the full OFE-day direct path. Its target is to make all 14 canonical
hillslope phases execute from typed direct-frame state for every day and OFE
lane, without hot-loop compatibility requests, payloads, writeback surfaces,
symbol lookup, dense refresh, or dirty flush. R5 must preserve the
no-publication-cutover boundary: R6 decides whether HBP/WAT/PASS/loss/manifest
read typed projection operands directly.

R4 closed the hydrology direct path through shadow-only projection. R5 extends
that foundation to the phases that still rely on compatibility scheduler/kernel
machinery or only have partial direct coverage:

- full run/lane/day direct executor lifecycle and persistent typed state;
- complete `Normalization` and `StorageBounds` phase ownership;
- complete `DecompositionTransition` and `ResiduePartitionTransition` phase
  ownership;
- complete `AnnualGrowthTransition` and `PerennialGrowthTransition` phase
  ownership;
- full 14-phase direct endpoint readiness with H2637 identity and endpoint/RSS
  evidence.

The remaining scope is grouped into five R5 work packages to amortize expensive
workspace, H2637, DuckDB/Arrow, and documentation gates. A grouped package may
contain multiple phase details, but it must close as one coherent, reviewable
unit with package-local evidence, final gates, commit, and push before the next
grouped package starts.

## Progress

- [ ] R5A full-day direct executor lifecycle.
- [ ] R5B direct normalization and storage-bounds phases.
- [ ] R5C direct decomposition and residue transitions.
- [ ] R5D direct annual and perennial growth transitions.
- [ ] R5E full 14-phase direct endpoint readiness and R5 closure.

When closing any unchecked row, replace it with the completion date, package
directory, pushed commit SHA, pushed branch, and final verdict.

## R5 Phase Inventory

The 14 canonical phases are fixed by `HillslopePhase::ORDERED` and
`DirectPhaseKind::ORDERED`:

| Phase | Current R4/R3 direct status | R5 disposition |
|---|---|---|
| `Normalization` | partial direct producers exist for input accounting, storage input, snow coupling, and liquid input | R5B owns the full phase as one typed phase view |
| `StorageBounds` | not direct-owned | R5B ports typed storage/layer/domain bound enforcement |
| `DecompositionTransition` | compatibility request/context path | R5C ports direct decomposition transition |
| `ResiduePartitionTransition` | compatibility request/context path | R5C ports direct residue partition transition |
| `AnnualGrowthTransition` | compatibility request/context path | R5D ports direct annual growth transition |
| `PerennialGrowthTransition` | compatibility request/context path | R5D ports direct perennial growth transition |
| `PercolationDeepSeepage` | R4M direct compute | R5E folds into canonical 14-phase executor |
| `Evapotranspiration` | R4N direct surface ET compute | R5E folds into canonical 14-phase executor |
| `Drainage` | R4O direct subsurface compute | R5E folds into canonical 14-phase executor |
| `LateralTransfer` | R3C/R4J/R4O direct transfer/carry compute | R5E folds into canonical 14-phase executor |
| `PlantRootUptake` | R4N direct root uptake compute | R5E folds into canonical 14-phase executor |
| `RunoffReconciliation` | R4A/R4I-L direct runoff path | R5E folds into canonical 14-phase executor |
| `StorageReconciliation` | R4B plus R4P/Q/Z direct storage/projection | R5E folds into canonical 14-phase executor |
| `ClosureDiagnostics` | R3B plus R4P/Q/Z direct closure/projection | R5E folds into canonical 14-phase executor |

R5A is a lifecycle prerequisite rather than a science phase. It must remove the
current single-day skeleton limitation and make direct execution operate over
the real run/lane/day dimensions before the missing phases are promoted.

## Package Sequence

Execute the packages in this order unless live package evidence proves a
different order is required. Any reordering must be recorded in the `Decision
Log` before implementation starts for the reordered package.

### R5A - Full-Day Direct Executor Lifecycle

Package name:
`YYYYMMDD-r5a-full-day-direct-executor-lifecycle-001`.

R5A turns the R2/R4 direct skeleton into a real run/lane/day lifecycle while
keeping phase math unchanged except for existing direct spans. It must run the
direct executor across every day and lane, carry persistent typed lane state
between days, commit typed day results back to lane state, and report direct
phase status in canonical order.

Minimum acceptance:

- direct executor iterates all `day_count * lane_count` direct day frames, not
  only day `0`;
- `DirectRunFrame`, `DirectLaneFrame`, and `DirectDayFrame` have explicit
  persistent-state handoff and end-of-day commit semantics;
- existing R3/R4 direct spans run under the lifecycle without constructing
  compatibility requests, writeback payloads, symbol registries, hot tables,
  indexed surfaces, dense refreshes, or dirty flushes;
- missing non-hydrology phases are represented as explicit typed no-op/hold
  phase views with status and counters, not hidden compatibility calls;
- default-disabled runner fixture still records zero direct-runtime counters;
- explicit opt-in runner fixture records direct execution over all fixture days
  and lanes and exactly one production compatibility-edge handoff while public
  outputs remain compatibility-authoritative;
- direct report records canonical 14-phase order, day/lane counts, phase entry
  counts, and persistent commit counts;
- no default activation, public output cutover, scheduler phase-order change,
  or R6 publication authority change;
- default-disabled H2637 median remains `<= 676.67 s`.

Rationale for ordering: every later R5 package needs a real direct day/lane
lifecycle to prove phase behavior across state persistence and not just isolated
fixtures.

### R5B - Direct Normalization And Storage-Bounds Phases

Package name:
`YYYYMMDD-r5b-normalization-storage-bounds-direct-phases-001`.

R5B completes the first two canonical phases. It consolidates R3A/R4C/R4G/R4I
normalization-adjacent producers into a typed `Normalization` phase view and
adds direct `StorageBounds` enforcement for soil/layer/frost/storage domains.
It must not change hydrology equations or public outputs.

Minimum acceptance:

- typed direct inputs, direct compute, state mutation, downstream operands, and
  shadow projection for `Normalization`;
- typed direct inputs, direct compute, state mutation, downstream operands, and
  shadow projection for `StorageBounds`;
- `Normalization -> StorageBounds -> DecompositionTransition` phase identity is
  explicit and tested;
- direct normalization owns daily forcing projection, initial storage inputs,
  snow/frost/rain/liquid classification inputs, runon/carry seed inputs, and
  direct phase context required by downstream R4 hydrology;
- direct storage bounds owns finite/domain/layer-capacity checks and any
  contract-authorized bounded normalization; it must fail closed on missing or
  invalid storage/layer state;
- anti-alias fixtures distinguish normalized direct forcing from raw climate,
  publication `RM`, snow-water state, irrigation publication, prior-day storage,
  and storage residuals;
- anti-alias fixtures distinguish storage bounds from storage reconciliation,
  publication profile capacity, frost exchange, and hydrology residual
  compensation;
- no public WB13/WAT/PASS/loss/schema cutover and no default activation;
- default-disabled H2637 median remains `<= 676.67 s`.

Rationale for ordering: `StorageBounds` is the dependency edge before
decomposition/residue and protects later plant/decomposition work from invalid
state.

### R5C - Direct Decomposition And Residue Transitions

Package name:
`YYYYMMDD-r5c-decomposition-residue-direct-transitions-001`.

R5C ports `DecompositionTransition` and `ResiduePartitionTransition` into the
direct frame. It owns active decomposition context, residue partition state,
decomposition-rate payloads, residue pools that influence ET/runoff, and
downstream operands consumed by growth and hydrology.

Minimum acceptance:

- typed direct inputs, direct compute, state mutation, downstream operands, and
  shadow projection for `DecompositionTransition`;
- typed direct inputs, direct compute, state mutation, downstream operands, and
  shadow projection for `ResiduePartitionTransition`;
- active management slot resolution for decomposition is typed and does not use
  `BoundarySymbol`, `BoundaryValue`, `HillslopeKernelRequest`, or
  compatibility context objects in the direct phase path;
- annual/fallow and perennial decomposition branches have focused fixtures and
  anti-alias vectors;
- residue partition outputs that feed ET/runoff are projected and compared to
  compatibility evidence without becoming public-output authoritative;
- missing schedule, ambiguous slot, invalid residue/decomposition domain, and
  nonfinite pool values fail closed with typed errors;
- R5B upstream phase state is required and tested;
- no growth phase migration is hidden inside R5C except typed downstream
  operands needed by R5D;
- no public output cutover or default activation;
- default-disabled H2637 median remains `<= 676.67 s`.

Rationale for ordering: decomposition and residue partition precede growth
transitions in the canonical phase graph and produce state used by later plant
and hydrology phases.

### R5D - Direct Annual And Perennial Growth Transitions

Package name:
`YYYYMMDD-r5d-growth-transition-direct-phases-001`.

R5D ports `AnnualGrowthTransition` and `PerennialGrowthTransition` into the
direct frame. It owns typed plant/growth schedule state, annual/perennial
activation, root-depth/runtime plant sentinels, LAI/canopy/residue coupling
operands, and downstream plant water-use context consumed by R4N ET/root
uptake.

Minimum acceptance:

- typed direct inputs, direct compute, state mutation, downstream operands, and
  shadow projection for `AnnualGrowthTransition`;
- typed direct inputs, direct compute, state mutation, downstream operands, and
  shadow projection for `PerennialGrowthTransition`;
- active crop/management slot resolution is typed and tested for annual,
  perennial, fallow, pre-plant skip, harvest/cut/grazing, and rotation-boundary
  cases in scope;
- R4N consumes direct growth/root-depth/plant-stress context where available
  and fails closed if required direct growth context is absent;
- anti-alias fixtures distinguish active crop slot, growth state, root depth,
  transpiration demand inputs, LAI/canopy signals, and publication ET aliases;
- missing active crop, ambiguous crop slot, invalid schedule domain, and
  nonfinite plant state fail closed with typed errors;
- R5C upstream residue/decomposition state is required and tested;
- no public WB13 ET, WAT plant metadata, PASS, loss, or manifest cutover;
- no default activation;
- default-disabled H2637 median remains `<= 676.67 s`.

Rationale for ordering: growth transitions are the last non-hydrology phases
before the hydrology tail and they provide direct plant context required for
R4N final `Ep`, `Ws`, and root-uptake lineage.

### R5E - Full 14-Phase Direct Endpoint Readiness And R5 Closure

Package name:
`YYYYMMDD-r5e-full-ofe-day-endpoint-readiness-001`.

R5E closes R5 by integrating R5A-D and R4A-P/Q/Z into one canonical 14-phase
direct OFE-day executor. It must prove that every phase is executed from direct
typed frame state for the full H2637 run, record endpoint/RSS evidence, and
decide whether R6 publication cutover is ready or blocked.

Minimum acceptance:

- direct executor records exactly the 14 canonical phase entries per OFE day in
  `DirectPhaseKind::ORDERED`, with no duplicated overlap spans counted as
  separate phase execution;
- all R4 hydrology spans are folded under their canonical phase entries or
  explicitly recorded as sub-operations of the canonical phase;
- all direct phases have inputs, direct compute, state mutation, downstream
  operands, and shadow projection;
- full H2637 opt-in direct execution runs for all days and lanes without
  constructing hot-loop compatibility requests, payloads, writeback surfaces,
  symbol lookup, dense refresh, or dirty flush;
- full H2637 identity passes against compatibility through shadow comparison
  and protected output checks; if parquet bytes vary, DuckDB/Arrow row
  equivalence is required;
- endpoint/RSS evidence is recorded for default-disabled compatibility,
  opt-in direct shadow+compatibility, and opt-in direct-only/projection-only
  execution if that mode exists;
- if opt-in direct-only/projection-only is slower than compatibility after
  compatibility-edge removal, close in `HOLD` with the named hotspot and a
  defect-shaped follow-on instead of proceeding to R6;
- default-disabled H2637 median remains `<= 676.67 s`;
- no default activation unless direct identity is clean, endpoint improves over
  compatibility, rollback remains available, and the package explicitly
  authorizes activation; otherwise leave activation off;
- no public WB13/WAT/PASS/loss/schema cutover; R6 owns that decision;
- R5 completion is recorded in `docs/ROADMAP.md` and
  `docs/work-packages/README.md`.

Rationale for ordering: this package should be the only R5 package allowed to
claim full OFE-day endpoint readiness because it depends on every missing phase
package and the completed R4 hydrology projection.

## Plan Of Work For Each Package

Run commands from `/home/workdir/openWEPP`.

Before starting a package, confirm the worktree:

```text
git status --short --branch
git pull --ff-only
```

Do not create or switch branches unless the user explicitly requests it. If the
current branch is `main`, commit and push `main`.

For each unchecked package row:

1. Scaffold the package directory using the package name declared in this plan,
   with `package.md`, `prompts/active/`, `prompts/archived/`, and `artifacts/`.
2. Add the package to `docs/work-packages/README.md` as scaffolded/queued and,
   when appropriate, to `docs/ROADMAP.md` as the active roadmap item.
3. Record pre-implementation artifacts before Rust edits: producer selection
   or phase-scope selection, process-span contract, operand lineage,
   pre-implementation contract gate, no-compatibility proof plan,
   default-disabled gate plan, endpoint/RSS plan where applicable, and
   line-count baseline.
4. Implement the narrowest direct phase/lifecycle slice that satisfies the
   package objective. Prefer existing direct-runtime patterns from R3/R4.
5. Add focused tests for phase identity, direct compute, state mutation,
   downstream operands, shadow projection, invalid input guards, anti-alias
   vectors, missing-upstream fail-closed behavior, and aggregate direct-runtime
   counters.
6. Run focused tests during iteration.
7. Run the package-required closure gates once for the grouped package after
   the whole grouped scope is implemented and focused iteration evidence is
   recorded.
8. Complete review artifacts, finding disposition, verification artifacts,
   line-count governance, gate results, worker handoff, roadmap/catalog
   updates, and final disposition.
9. Commit and push the package write set.
10. Update this ExecPlan's `Progress` checklist only after the pushed commit
    SHA is known.

Do not mark a grouped package partially complete. If one included phase cannot
close, the grouped package closes in `HOLD` with a named blocker, or the plan
is amended before implementation starts with a recorded decision that changes
the group boundary.

## Required Package Artifacts

Every R5 package must include at least:

- `package.md`;
- `artifacts/producer-selection.md` or `artifacts/scope-selection.md`;
- `artifacts/process-span-contract.md`;
- `artifacts/operand-lineage.md`;
- `artifacts/pre-implementation-contract-gate.md`;
- `artifacts/implementation-test-evidence.md`;
- `artifacts/no-compatibility-proof-checklist.md`;
- `artifacts/default-disabled-regression-gate.md`;
- `artifacts/endpoint-rss-evidence.md` for R5A and R5E, optional but
  recommended for R5B-D;
- `artifacts/gate-results.md`;
- `artifacts/line-count-governance.md`;
- `artifacts/review_agent_a.md`;
- `artifacts/review_agent_b.md`;
- `artifacts/verification_agent_a.md`;
- `artifacts/verification_agent_b.md`;
- `artifacts/disposition.md`;
- `artifacts/worker-handoff.md`;
- `prompts/active/<package>_kickoff_agent_prompt.md`;
- `prompts/archived/README.md`.

The package must explicitly authorize delegated review/verification subagents
only when delegation is required. If delegation is not authorized, equivalent
review and verification artifacts may be completed locally, but they must not
claim delegated work occurred.

## Validation And Acceptance

Each individual R5 package is accepted only when:

1. its package-local exit criteria have current direct evidence;
2. all touched direct phases include inputs, direct compute, state mutation,
   downstream operands, and shadow projection;
3. missing upstream producers or phase prerequisites fail closed with typed
   `DirectRuntimeError` or package-specific typed errors;
4. anti-alias tests distinguish the accepted operand from every plausible
   adjacent process/publication/diagnostic/residual substitute in scope;
5. direct-runtime source remains free of compatibility storage/request/
   writeback/symbol access;
6. default-disabled runner fixture records zero direct-runtime counters;
7. explicit opt-in runner fixture records positive counters for completed
   direct phases and only the declared validation compatibility edge while
   public outputs remain compatibility-authoritative;
8. default-disabled H2637 median remains `<= 676.67 s`;
9. protected output identity passes, with DuckDB/Arrow row equivalence used
   when parquet bytes vary;
10. endpoint/RSS evidence is recorded for R5A and R5E;
11. `cargo fmt --check` passes;
12. `cargo clippy --workspace --all-targets -- -D warnings` passes;
13. `cargo test --workspace` passes;
14. `cargo deny check` passes;
15. scoped markdown lint passes;
16. `git diff --check` passes;
17. dual review and dual verification artifacts explicitly check the Gate
   Evidence Non-Deferral Rule;
18. no `.rs` touched file at or above 2000 lines lacks a WARN disposition, and
   no non-exempt touched file at or above 3000 lines remains unresolved;
19. the package commit has been pushed to `origin`, and this ExecPlan records
   the pushed commit SHA.

R5 as a whole is accepted only when every grouped Progress checklist item from
R5A through R5E is complete or intentionally held with a named architecture
blocker, and the final R5E package records whether the next stage is R6 direct
publication cutover, an R5 performance hold-lift package, or an R5
identity/closure hold-lift package.

## Concrete Commands

Use package-specific focused tests first. At minimum, each package should add
or update a direct-runtime test filter and a runner counter filter similar to:

```text
cargo test -p openwepp-hillslope-orchestrator r5x_ -- --nocapture
cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture
cargo test -p openwepp-runner r2a_ -- --nocapture
```

Run the no-compatibility source scan against direct-runtime files:

```text
rg -n "SymbolRegistry|BoundarySymbol|BoundaryValue|Option<BoundaryValue>|HillslopeWritebackSurface|KernelWritebackPayload|IndexedWritebackSurface|HotSymbolTables|HillslopeKernelRequest|execute_with_kernel|state_value_for_symbol|flux_value_for_symbol|dirty_state_ids|dirty_flux_ids" \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/*.rs
```

Run scheduler diff review. Scheduler edits are allowed only for package-scoped
direct executor selection/reporting work; they must preserve default-disabled
compatibility behavior and canonical phase order:

```text
git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs \
  crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  crates/openwepp-runner/src/api.rs
```

Run final Rust gates:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

Build the release runner for H2637 evidence:

```text
/usr/bin/time -f 'release_build\t%e\t%M' \
  cargo build --release -p openwepp-runner --bin openwepp-cli-hill
sha256sum target/release/openwepp-cli-hill target/release/openwepp-cli-hill.json
```

Run three default-disabled H2637 reps with direct-runtime and diagnostic env
vars unset:

```text
env \
  -u OPENWEPP_PERFDEEP02_FRAME_ISLAND \
  -u OPENWEPP_PERFDEEP03_LANE_DENSE_STATE \
  -u OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH \
  -u OPENWEPP_INDEXED_SHADOW_REPORT_PATH \
  -u OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH \
  -u OPENWEPP_HPHYS0245_TRACE_PATH \
  /usr/bin/time -f 'r5x_h2637_default_repN\t%e\t%M' \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir /tmp/r5x-h2637/default/repN/h2637_same \
  --policy compat \
  --legacy-sidecar-discovery
```

When a package adds or changes an explicit direct opt-in CLI/API path, run and
record the opt-in mode command using the package's selected mode name. Until a
new mode exists, use runner unit tests that call
`HillslopeRuntimeSelection::DirectSkeletonNoop` or
`HillslopeRuntimeSelection::DirectSkeletonShadowOnly` directly.

Record known `MOFE01-MG-W-001` warnings as warnings, not failures, unless a
package changes their text, count, or classification.

Run PASS row equivalence against the retained PERFDEEP07 baseline when parquet
bytes vary:

```text
duckdb -c "WITH baseline AS (SELECT * FROM read_parquet('/tmp/perfdeep07/default/rep1/h2637_same/H2637.pass.parquet')), candidate AS (SELECT * FROM read_parquet('/tmp/perfmig01-final/current/anchor/h2637_same/H2637.pass.parquet')), left_minus AS (SELECT * FROM baseline EXCEPT ALL SELECT * FROM candidate), right_minus AS (SELECT * FROM candidate EXCEPT ALL SELECT * FROM baseline) SELECT (SELECT count(*) FROM baseline) AS baseline_rows, (SELECT count(*) FROM candidate) AS candidate_rows, (SELECT count(*) FROM left_minus) AS left_minus_right, (SELECT count(*) FROM right_minus) AS right_minus_left;"
duckdb -c "SELECT count(*) AS column_count FROM (DESCRIBE SELECT * FROM read_parquet('/tmp/perfmig01-final/current/anchor/h2637_same/H2637.pass.parquet'));"
```

Run scoped docs lint and whitespace checks:

```text
markdown-doc lint \
  --path docs/ROADMAP.md \
  --path docs/work-packages/README.md \
  --path docs/work-packages/r5-burndown-execplan.md \
  --path <package-dir> \
  --format json
git diff --check
```

Commit and push after the package closes:

```text
git status --short --branch
git add <package-write-set>
git commit -m "Complete <R5 group> <short package description>"
git push origin "$(git branch --show-current)"
git log -1 --oneline
git status --short --branch
```

Do not mark the package complete in this ExecPlan until the push succeeds.

## Idempotence And Recovery

This plan is safe to resume. If a package directory already exists, read its
`package.md`, artifacts, and latest git status before editing. Do not restart
from scratch unless the package explicitly says it is abandoned or superseded.

If a focused or full gate fails, fix the failure inside the current package
when it is inside the package authority envelope. If the failure requires
science-contract changes, public output schema changes, default activation, or
a different process family than the package declared, mark the package `HOLD`
and write a defect-shaped handoff instead of expanding scope silently.

If H2637 default-disabled median exceeds `676.67 s`, stop new R5 feature work.
First close a disabled-path hard-isolation package or explicitly supersede the
timing gate with a reviewed architecture decision. Do not keep adding R5
phases on top of a known default-disabled regression.

If an opt-in direct-only/projection-only endpoint is slower than compatibility
after compatibility-edge removal, close the current endpoint package in `HOLD`
with hotspot evidence. Do not proceed to R6 cutover planning as if endpoint
readiness passed.

If `git push` fails, leave the current package unchecked, record the blocker in
the package handoff and this plan's `Surprises & Discoveries`, and do not mark
the package complete until the push succeeds.

Do not use `git reset --hard` or revert unrelated user work. Stage only the
package write set.

## Surprises & Discoveries

- 2026-06-20: R5 planning starts after R4P/Q/Z closed the hydrology projection
  scope. The next unresolved direct-frame work is not more hydrology producers;
  it is full run/lane/day lifecycle plus the non-hydrology phases that precede
  the hydrology tail.
- 2026-06-20: Current explicit opt-in runner evidence uses
  `HillslopeRuntimeSelection::DirectSkeletonNoop` and
  `HillslopeRuntimeSelection::DirectSkeletonShadowOnly`; no public direct-only
  CLI endpoint exists yet. R5 packages must extend or replace that explicit
  selection path without changing default compatibility behavior.

## Decision Log

- 2026-06-20: Sequence R5 by canonical dependency order and endpoint risk:
  lifecycle first, normalization/storage bounds second, decomposition/residue
  third, growth transitions fourth, full endpoint readiness last. Rationale:
  each package depends on state or context produced by the prior package, and
  only R5E has enough scope to claim full 14-phase endpoint readiness.
- 2026-06-20: Group R5 into five packages: R5A, R5B, R5C, R5D, and R5E.
  Rationale: this preserves diagnosable phase-family boundaries while avoiding
  repeated full H2637/workspace gates for tightly coupled phase pairs.
- 2026-06-20: Keep public WB13/WAT/PASS/loss/manifest cutover out of R5.
  Rationale: the architecture assigns publication cutover to R6; R5 may prove
  full direct endpoint readiness and typed projection identity but must not
  make public outputs direct-authoritative unless this plan is amended and R6
  authority is explicitly granted.
- 2026-06-20: Require commit and push after every completed R5 package before
  checking its Progress row. Rationale: pushed SHAs keep this burn-down tracker
  auditable for autonomous agents.

## Outcomes & Retrospective

R5 has not started. R5A is the first package.
