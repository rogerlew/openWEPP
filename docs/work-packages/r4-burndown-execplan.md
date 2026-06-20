# R4 Hydrology Direct-Path Burn-Down ExecPlan

This ExecPlan is a living document. The sections `Progress`,
`Surprises & Discoveries`, `Decision Log`, and `Outcomes & Retrospective` must
be kept up to date as work proceeds.

Maintain this plan in accordance with `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`,
`docs/architecture/array-native-runtime-specification.md`, and the relevant
science-contract local playbooks. R4 work is kernel-affecting: every package
that edits runtime or kernel code must read
`docs/specifications/science-contracts/AGENTS.md` before production edits.

## Purpose / Big Picture

R4 is the comprehensive array-native hydrology direct path. Its target is not a
new hydrology equation and not a public-output cutover. Its target is to make
the full daily OFE hydrology path executable from typed direct-frame state
without hot-loop compatibility requests, payloads, writeback surfaces, symbol
lookup, dense refresh, or dirty flush.

R4A through R4D proved the shape in small slices:

- R4A: direct runoff partition span.
- R4B: downstream WB12 storage-reconciliation consumer.
- R4C: storage-input producer for `storage_initial_m` and `precip_input_m`.
- R4D: WB18/WB12 deep-seepage handoff producer for `deep_seepage_m`.

The remaining R4 work must finish the direct storage-budget producers, replace
the seeded runoff inputs, promote handoff producers into direct WB17/WB18/WB19
compute, build a shadow publication projection, and close a full hydrology
direct-path integration gate.

The remaining scope is grouped into five R4 work packages to amortize expensive
workspace, H2637, DuckDB/Arrow, and documentation gates. A grouped package may
contain multiple originally planned R4 letters, but it still must close as one
coherent, reviewable unit with package-local evidence, final gates, commit, and
push before the next grouped package starts.

## Progress

- [x] (2026-06-20) R4A complete:
  `docs/work-packages/20260620-r4a-direct-runoff-partition-span-001/`.
- [x] (2026-06-20) R4B complete:
  `docs/work-packages/20260620-r4b-direct-storage-reconciliation-consumer-001/`.
- [x] (2026-06-20) R4C complete:
  `docs/work-packages/20260620-r4c-direct-storage-input-producer-001/`.
- [x] (2026-06-20) R4D complete:
  `docs/work-packages/20260620-r4d-direct-deep-seepage-producer-001/`.
- [ ] R4E-H direct storage-budget handoff completion.
- [ ] R4I-L direct runoff-path input completion.
- [ ] R4M/O direct subsurface compute promotion.
- [ ] R4N direct WB17 evapotranspiration/root-uptake compute promotion.
- [ ] R4P/Q/Z direct hydrology projection and R4 closure.

When closing any unchecked row, replace it with the completion date, package
directory, pushed commit SHA, pushed branch, and final verdict.

## Remaining R4 Direct Paths

The remaining hydrology direct paths fall into four groups.

First, R4B still has storage-reconciliation operands that must be produced by
direct upstream spans rather than seeded directly in tests or executor setup:
`subsurface_loss_m` / `Qd`, `evapotranspiration_m`, and
`snow_coupling_m`. Closure tolerance is a policy input rather than process
physics, but it must be owned explicitly so R4B has no ambiguous residual
authority.

Second, R4A still consumes seeded runoff-partition inputs:
`liquid_input_m`, `runon_input_m`, `cumulative_infiltration_m`,
`depression_storage_delta_m`, and `surface_saturation_runoff_m`. These must
come from direct interception/liquid-input, runon/carry, and WB14
infiltration/depression/saturation producers before R4A can claim a complete
direct runoff path.

Third, R4D and R4E-H will initially leave some upstream values as safe handoff
producers. Full R4 requires promoting those handoffs into direct process
compute: WB18 percolation for `D` and `Pe`, WB17 evapotranspiration/root uptake
for `ET`, `Ep`, `Es`, and `Er`, and WB19 lateral/drainage for `q`, `Qdd`, and
`Qd`.

Fourth, full hydrology execution needs direct aggregate storage recomputation
and a shadow publication projection for R6 readiness. R6 will decide public
publication cutover. R4 must only prove that direct hydrology produces the
projection operands and can compare them to compatibility outputs without using
compatibility storage in the direct hot loop.

## Package Sequence

Execute the packages in this order unless live package evidence proves a
different order is required. Any reordering must be recorded in the `Decision
Log` before implementation starts for the reordered package.

### R4E-H - Direct Storage-Budget Handoff Completion

Package name:
`YYYYMMDD-r4eh-direct-storage-budget-handoff-completion-001`.

R4E-H completes the remaining R4B upstream producer surface in one package. It
adds direct handoff producers for:

- `Qd` / `subsurface_loss_m` under `SC-SUBHYD-001`;
- aggregate `evapotranspiration_m`;
- signed `snow_coupling_m` under `SC-WATBAL-001#INV-WATBAL-013`;
- the R4B producer-completeness and closure-tolerance gate.

This package deliberately groups the former R4E, R4F, R4G, and R4H so the full
R4B closure matrix, default-disabled H2637 reps, protected identity check, and
workspace gates run once for the combined storage-budget handoff surface.

Minimum acceptance:

- typed direct inputs, direct handoff computation, state mutation where
  applicable, downstream operands, and shadow projection for
  `subsurface_loss_m`, `evapotranspiration_m`, and `snow_coupling_m`;
- natural spans include `Drainage -> LateralTransfer ->
  StorageReconciliation`, `Evapotranspiration -> StorageReconciliation`, and
  `SnowFrostCoupling -> StorageReconciliation`;
- finite nonnegative guards for `Qd` and aggregate ET, plus finite signed guard
  and explicit allowed sign semantics for `snow_coupling_m`;
- R4B requires R4C, R4D, the new R4E-H producers, and R4A upstream direct
  shadows, and fails closed if any required producer did not run;
- storage-budget tests prove every R4B process operand came from its direct
  producer, using sentinel values that would fail if a producer did not
  overwrite its target;
- closure tolerance is finite, nonnegative, and policy-owned;
- anti-alias fixtures distinguish `Qd` from lateral-only `q`, tile drainage
  `Qdd`, `D`, ET, snow, runoff, storage residual, and publication
  `latqcc`/`Dp`;
- anti-alias fixtures distinguish aggregate ET from `Ep`, `Es`, `Er`, residue
  interception, drainage, lateral loss, precipitation, runoff, and storage
  residual;
- anti-alias fixtures distinguish snow/frost storage coupling from raw
  precipitation, routed melt, post-winter rain, frozen-water publication,
  snow-water publication, runoff, ET, `D`, `Qd`, and residual compensation;
- direct storage reconciliation has no manual authoritative operand seeding in
  aggregate executor tests except explicit policy/default zero fields;
- no scheduler, output schema, public publication, compatibility runtime,
  public WB13 ET, `RM`, `Snow-Water`, `frozwt`, or default activation changes;
- default-disabled H2637 median remains `<= 676.67 s`.

### R4I-L - Direct Runoff-Path Input Completion

Package name:
`YYYYMMDD-r4il-direct-runoff-path-input-completion-001`.

R4I-L replaces the remaining seeded inputs consumed by R4A. It adds direct
producers for:

- `liquid_input_m` after interception and snow/rain/irrigation coupling;
- `runon_input_m` and MOFE carry operands;
- `cumulative_infiltration_m`;
- `depression_storage_delta_m`;
- `surface_saturation_runoff_m`;
- the R4A direct runoff-path completeness gate.

This package groups the former R4I, R4J, R4K, and R4L so the R4A identity,
missing-upstream, anti-alias, default-disabled, and full workspace gates run
once for the combined runoff input surface.

Minimum acceptance:

- typed direct inputs, direct handoff or compute, state mutation where
  applicable, downstream operands, and shadow projection for liquid input,
  runon/carry, infiltration, depression storage, and saturation addback;
- package pre-implementation disposition says explicitly whether
  infiltration/depression/saturation is handoff-only or direct WB14 compute;
- R4A requires R4I-L direct upstream shadows and fails closed if a required
  upstream output is missing;
- R4A aggregate tests seed wrong sentinels and prove producer overwrite before
  runoff computation;
- direct runoff partition remains formula-identical and
  closure-residual-clean;
- no direct runoff input remains authoritative solely because a test or
  executor initialized it;
- conservation-sensitive operand lineage covers liquid, interception, runon,
  infiltration, depression, and saturation terms;
- area-ratio and lane-topology guards preserve R3C transfer/topology evidence;
- MOFE hourly carry-array shape evidence is recorded when hourly lanes are in
  scope;
- anti-alias fixtures distinguish liquid after interception from raw
  precipitation, routed melt, irrigation publication, interception storage,
  runoff, and storage residual;
- anti-alias fixtures distinguish upstream surface carry from subsurface
  carry, lateral transfer, local precipitation, and public `UpStrmQ`;
- anti-alias fixtures distinguish cumulative infiltration from depression
  storage, saturation addback, partition runoff, liquid input, and publication
  runoff;
- no public WB15 interception, WB13 `RM`, WAT carry, or runoff publication
  cutover.

### R4M/O - Direct Subsurface Compute Promotion

Package name:
`YYYYMMDD-r4mo-direct-subsurface-compute-promotion-001`.

R4M/O promotes the R4D and R4E-H handoffs into direct WB18 and WB19 compute. It
owns layer-vector percolation state, `D`, `Pe`, per-layer percolation fluxes,
realized lateral `q`, tile drainage `Qdd`, final `Qd`, capacity/target
diagnostics, layer withdrawal, hourly/daily branch behavior, and MOFE carry
outputs under `SC-PERC-001`, `SC-SUBHYD-001`, and relevant
`SC-WATBAL-001` ordering invariants.

This package groups WB18 and WB19 because their layer-state mutation,
percolation, drainage, lateral withdrawal, and `Qd` storage-budget handoff are
tightly coupled. It stays separate from WB17 ET/root uptake so branch failures
remain diagnosable.

Minimum acceptance:

- typed direct WB18/WB19 layer-vector inputs, direct compute, mutated state,
  downstream operands, and shadow projection;
- bit-exact focused fixtures against compatibility/kernel authority for daily
  and hourly-relevant WB18 branches;
- focused WB19 fixtures for daily and hourly branch selection, drainage before
  lateral in hourly tail, realized-withdrawal caps, `q + Qdd = Qd`, and carry
  array production;
- `D` and `Pe` downstream operands replace the R4D handoff inputs;
- R4B consumes `Qd` from direct WB19 compute, not the R4E-H handoff;
- anti-alias fixtures distinguish percolation from lateral/drainage,
  root-zone ET, storage residual, and public `Dp`;
- anti-alias fixtures distinguish potential, target, realized `q`, `Qdd`,
  `Qd`, `D`, and public `latqcc`;
- no public `Dp` or subsurface publication cutover.

### R4N - Direct WB17 Evapotranspiration And Root-Uptake Compute Promotion

Package name:
`YYYYMMDD-r4n-direct-wb17-et-root-uptake-compute-001`.

R4N promotes the R4E-H aggregate ET handoff into direct WB17 ET/root-uptake
compute. It owns ET component state, root uptake, soil evaporation, residue
interception coupling, and the aggregate ET consumed by R4B. It must preserve
WB17 ordering relative to percolation and WB19 according to `SC-WATBAL-001`.

Minimum acceptance:

- typed direct WB17 input/state/downstream/shadow types;
- focused fixtures for component ET, aggregate ET, root uptake, and layer
  storage mutation;
- R4B consumes aggregate ET from direct WB17 compute, not the R4E-H handoff;
- anti-alias fixtures distinguishing aggregate ET from `Ep`, `Es`, `Er`,
  storage residual, and publication-side reconstruction;
- no public WB13 ET cutover.

### R4P/Q/Z - Direct Hydrology Projection And R4 Closure

Package name:
`YYYYMMDD-r4pqz-hydrology-projection-r4-closure-001`.

R4P/Q/Z closes R4 by recomputing aggregate storage from direct layer state,
building the direct hydrology publication-projection shadow, and proving the
full direct hydrology daily OFE path. It includes the former R4P, R4Q, and R4Z
because aggregate storage, projection shadows, full-chain no-compatibility
counters, H2637 identity, endpoint/RSS evidence, and roadmap closure are one
integration surface.

R4P/Q/Z must compare direct projection operands to compatibility publication
outputs without making direct projection public-authoritative. R6 decides
public WB13/WAT/PASS/loss/schema cutover.

Minimum acceptance:

- typed direct aggregate-storage state, direct recompute, downstream operands,
  and shadow projection;
- aggregate recompute from layer state, not stale storage surfaces or
  publication rows;
- frost/frozen-water separation evidence where frost state is present;
- R4C and R4B use direct aggregate storage lineage for same-day/next-day
  storage as defined by the package authority gate;
- typed direct hydrology publication projection structure for `Q`, `QOFE`,
  `Ep`, `Es`, `Er`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`,
  snow/frost storage terms, carry terms, and profile-capacity operands where
  direct R4 producers own the lineage;
- shadow comparison fixtures for single-OFE and MOFE cases;
- independent operand reconstruction for conservation-sensitive projection
  fields;
- manifest evidence that direct projection is shadow-only;
- direct hydrology opt-in path constructs no hot-loop compatibility surfaces;
- call-graph and runtime counters prove zero direct hot-loop calls to
  `execute_with_kernel*`, `HillslopeKernelRequest`, `KernelWritebackPayload`,
  `HillslopeWritebackSurface`, `SymbolRegistry`, hot tables, indexed surfaces,
  dense refresh, or dirty flush;
- focused phase and full-chain fixtures pass;
- H2637 protected identity passes;
- H2637 endpoint/RSS is recorded and shows material endpoint movement versus
  compatibility mode, or the package closes in `HOLD` with a named blocker;
- default-disabled H2637 median remains `<= 676.67 s`;
- no public `Total-Soil`, `SoilWaterTotal`, profile-capacity,
  WB13/WAT/PASS/loss/schema cutover;
- R4 completion is recorded in `docs/ROADMAP.md` and
  `docs/work-packages/README.md`.

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
3. Record pre-implementation artifacts before Rust edits:
   producer-selection, process-span contract, operand lineage, contract gate,
   no-compatibility proof plan, default-disabled gate plan, and line-count
   baseline.
4. Implement the narrowest direct producer/consumer slice that satisfies the
   package objective. Prefer existing direct-runtime patterns from R4A-R4D.
5. Add focused tests for phase-span identity, direct compute, state mutation,
   downstream operands, shadow projection, invalid input guards,
   anti-alias vectors, missing-upstream fail-closed behavior, and aggregate
   direct-runtime counters.
6. Run focused tests during iteration.
7. Run the package-required closure gates once for the grouped package after
   the whole grouped scope is implemented and focused iteration evidence is
   recorded.
8. Complete review artifacts, finding disposition, verification artifacts,
   line-count governance, gate results, worker handoff, roadmap/catalog updates,
   and final disposition.
9. Commit and push the package write set.
10. Update this ExecPlan's `Progress` checklist only after the pushed commit
    SHA is known.

Do not mark a grouped package partially complete. If one included slice cannot
close, the grouped package closes in `HOLD` with a named blocker, or the plan is
amended before implementation starts with a recorded decision that changes the
group boundary.

## Required Package Artifacts

Every R4 package must include at least:

- `package.md`;
- `artifacts/producer-selection.md` or `artifacts/scope-selection.md`;
- `artifacts/process-span-contract.md`;
- `artifacts/operand-lineage.md`;
- `artifacts/pre-implementation-contract-gate.md`;
- `artifacts/implementation-test-evidence.md`;
- `artifacts/no-compatibility-proof-checklist.md`;
- `artifacts/default-disabled-regression-gate.md`;
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

Each individual R4 package is accepted only when:

1. its package-local exit criteria have current direct evidence;
2. all touched direct spans include inputs, direct compute, state mutation,
   downstream operands, and shadow projection;
3. missing upstream producers fail closed with typed `DirectRuntimeError`
   evidence;
4. anti-alias tests distinguish the accepted operand from every plausible
   adjacent process/publication/diagnostic/residual substitute in scope;
5. direct-runtime source remains free of compatibility storage/request/
   writeback/symbol access;
6. default-disabled runner fixture records zero direct-runtime counters;
7. explicit opt-in runner fixture records positive counters for all completed
   direct spans and one production compatibility handoff, if compatibility
   execution still follows direct shadow execution;
8. default-disabled H2637 median remains `<= 676.67 s`;
9. protected output identity passes, with DuckDB/Arrow row equivalence used when
   parquet bytes vary;
10. `cargo fmt --check` passes;
11. `cargo clippy --workspace --all-targets -- -D warnings` passes;
12. `cargo test --workspace` passes;
13. `cargo deny check` passes;
14. scoped markdown lint passes;
15. `git diff --check` passes;
16. dual review and dual verification artifacts explicitly check the Gate
   Evidence Non-Deferral Rule;
17. no `.rs` touched file at or above 2000 lines lacks a WARN disposition, and
   no non-exempt touched file at or above 3000 lines remains unresolved;
18. the package commit has been pushed to `origin`, and this ExecPlan records
   the pushed commit SHA.

R4 as a whole is accepted only when every grouped Progress checklist item from
R4E-H through R4P/Q/Z is complete or intentionally held with a named
architecture blocker, and the final R4P/Q/Z package records whether the next
stage is R5 full OFE-day direct path, R6 publication cutover, or an R4
follow-on hold-lift package.

## Concrete Commands

Use the package-specific focused tests first. At minimum, each package should
add or update a direct-runtime test filter and a runner counter filter similar
to:

```text
cargo test -p openwepp-hillslope-orchestrator r4x_ -- --nocapture
cargo test -p openwepp-hillslope-orchestrator r4b_ -- --nocapture
cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture
cargo test -p openwepp-runner r2a_ -- --nocapture
```

Run the no-compatibility source scan against direct-runtime files:

```text
rg -n "SymbolRegistry|BoundarySymbol|BoundaryValue|Option<BoundaryValue>|HillslopeWritebackSurface|KernelWritebackPayload|IndexedWritebackSurface|HotSymbolTables|HillslopeKernelRequest|execute_with_kernel|state_value_for_symbol|flux_value_for_symbol|dirty_state_ids|dirty_flux_ids" \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
```

Run scheduler no-diff:

```text
git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs
```

Run final Rust gates:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

Build the release runner for H2637 endpoint evidence:

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
  /usr/bin/time -f 'r4x_h2637_default_repN\t%e\t%M' \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir /tmp/r4x-h2637/default/repN/h2637_same \
  --policy compat \
  --legacy-sidecar-discovery
```

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
  --path <package-dir> \
  --format json
git diff --check
```

Commit and push after the package closes:

```text
git status --short --branch
git add <package-write-set>
git commit -m "Complete <R4 group> <short package description>"
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
science-contract changes, public output schema changes, scheduler activation,
default activation, or a different process family than the package declared,
mark the package `HOLD` and write a defect-shaped handoff instead of expanding
scope silently.

If H2637 default-disabled median exceeds `676.67 s`, stop new R4 feature work.
First close a disabled-path hard-isolation package or explicitly supersede the
timing gate with a reviewed architecture decision. Do not keep adding R4
producers on top of a known default-disabled regression.

If `git push` fails, leave the current package unchecked, record the blocker in
the package handoff and this plan's `Surprises & Discoveries`, and do not mark
the package complete until the push succeeds.

Do not use `git reset --hard` or revert unrelated user work. Stage only the
package write set.

## Surprises & Discoveries

- No surprises recorded yet.

## Decision Log

- 2026-06-20: Sequence R4 by dependency and risk. Finish the remaining R4B
  storage-budget handoff producers first (`Qd`, ET, snow/frost), then prove
  R4A runoff input producer completeness, then promote WB18/WB17/WB19 from
  handoff to direct compute, then close projection shadow and full hydrology
  integration. Rationale: this preserves the R4A-R4D pattern of narrow,
  anti-aliased producers before wider branch-surface migration.
- 2026-06-20: Group the remaining R4 scope into five work packages:
  R4E-H, R4I-L, R4M/O, R4N, and R4P/Q/Z. Rationale: the full workspace,
  H2637, DuckDB/Arrow, and documentation gates have high fixed overhead, and
  these groups preserve useful failure isolation while avoiding repeated gates
  for tightly coupled slices.
- 2026-06-20: Keep public WB13/WAT/PASS/loss/manifest cutover out of R4.
  Rationale: the architecture assigns publication cutover to R6; R4 may build
  shadow projection operands but must not make them public-authoritative.
- 2026-06-20: Require commit and push after every completed package before
  checking its Progress row. Rationale: the user explicitly requested autonomous
  package completion with commit/push after each package, and pushed SHAs keep
  this burndown tracker auditable.

## Outcomes & Retrospective

R4A through R4D are complete and pushed before this plan was authored. R4E-H is
the next package to scaffold and execute.
