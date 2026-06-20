# R4D - Direct Deep-Seepage Producer

Status: complete.

Package type: implementation work package / array-native runtime R4D.

## Objective

Implement the next post-R4C direct-runtime slice: a direct WB18/WB12
deep-seepage handoff producer that feeds R4B
`DirectStorageReconciliationInputs::deep_seepage_m`.

R4D must prove that R4B can consume a third direct upstream producer from a
distinct process authority family (`SC-PERC-001`) while preserving the
no-publication, no-default-activation, no-scheduler, and no-compatibility-access
boundaries established by R4A through R4C.

## Rationale

R4B still consumes several explicit storage-reconciliation operands after R4C:
snow coupling `S`, evapotranspiration, deep seepage `D`, and subsurface loss
`Qd`. The recommended next slice is `D` because:

- `D` has explicit Chapter-5 and WB18 authority in `SC-PERC-001` and WB12
  storage-budget authority in `SC-WATBAL-001`;
- it is downstream of percolation and upstream of WB12 storage reconciliation,
  so it naturally fits `PercolationDeepSeepage -> StorageReconciliation`;
- it avoids the wider branch surface of ET, snow coupling, and WB19 lateral /
  drainage `Qd`;
- it can remain a direct handoff producer without public `Dp` publication
  cutover or full WB18 process migration.

## Scope

In scope:

- record R4D process-span contract, operand lineage, producer-selection
  rationale, and pre-implementation contract gate before Rust edits;
- add direct deep-seepage producer input/state/downstream/shadow/report types;
- add a direct R4D span over
  `PercolationDeepSeepage -> StorageReconciliation`;
- consume a dedicated direct deep-seepage handoff operand and compute
  canonical direct `deep_seepage_m`;
- mutate direct deep-seepage state and
  `DirectStorageReconciliationInputs::deep_seepage_m`;
- require R4D as a direct upstream for R4B storage reconciliation;
- keep snow coupling, evapotranspiration, subsurface loss `Qd`, and closure
  tolerance as explicit R4B inputs;
- update aggregate direct-runtime counters for
  R3A/R3B/R3C/R4A/R4B/R4C/R4D spans;
- add focused tests for R4D identity, downstream operands, R4B consumption,
  anti-alias vectors, invalid input failures, no-compatibility proof coverage,
  and runner counters;
- preserve default-disabled H2637 median `<= 676.67 s`;
- update package artifacts, roadmap/catalog state, reviews, verification,
  line-count governance, and disposition.

Out of scope:

- full WB18 percolation equation migration;
- public WB13/WAT `Dp` publication, output schema, units, metadata, or manifest
  changes;
- WB19 lateral/drainage `q`, `Qdd`, or `Qd` producer migration;
- ET, snow coupling, irrigation, interception, carry-array, Green-Ampt
  infiltration, WB12 full producer migration, or endpoint-improvement claims;
- scheduler or compatibility runtime edits;
- deleting or bypassing compatibility production paths;
- default activation;
- using publication `Dp`, WB19 `Qd`, R3B diagnostic ledger values, or storage
  residual compensation as `deep_seepage_m` authority.

## Authority

Canonical authority:

- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - Chapter-5 daily water-balance `D` authority;
  - WB18 percolation / below-root-zone loss semantics;
  - WB13 daily output coupling context for `Dp` as a publication consumer, not
    an R4D publication target;
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - WB12 Reconciliation Authority Addendum;
  - `INV-WATBAL-013`;
  - `INV-WATBAL-016`;
  - `INV-WATBAL-034`;
  - HPHYS0239 / HPHYS0242 WB19->WB12/WB13 ordering boundaries;
- `docs/work-packages/20260620-r4c-direct-storage-input-producer-001/`
  - R4C worker handoff and no-publication boundary.

R4D is a direct-runtime migration slice under existing WB18/WB12 contract
authority. It does not amend canonical `SC-*` text unless the
pre-implementation contract gate proves the selected deep-seepage handoff lacks
sufficient authority.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/decisions/0025-array-native-hillslope-day-frame.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/work-packages/20260620-r4b-direct-storage-reconciliation-consumer-001/package.md`
- `docs/work-packages/20260620-r4c-direct-storage-input-producer-001/package.md`
- `docs/work-packages/20260620-r4c-direct-storage-input-producer-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260620-r4c-direct-storage-input-producer-001/artifacts/operand-lineage.md`
- `crates/AGENTS.md`

On-demand source inventory:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `docs/work-packages/20260620-r4d-direct-deep-seepage-producer-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

No scheduler, output writer, output schema, dependency, or compatibility runtime
edit is authorized. Science-contract edits are authorized only if the
pre-implementation contract gate proves the selected R4D deep-seepage handoff
slice lacks canonical authority.

## Subagent Authorization

No delegated subagent work is required by this package. Review and verification
artifacts may be completed locally unless the user separately requests delegated
agents.

## Phase Plan

1. Confirm the selected R4D deep-seepage handoff slice, producer-selection
   rationale, operand lineage, and contract gate.
2. Add direct deep-seepage producer types, validation, execution, downstream
   operands, and shadow projection.
3. Require R4D deep-seepage production before R4B storage reconciliation.
4. Extend the direct executor to run R4D after R4C and before R4B.
5. Add focused tests for exact deep-seepage identity, R4B consumption,
   anti-alias vectors, invalid input failures, aggregate counters, and runner
   counters.
6. Run focused tests, full Rust gates, no-compatibility proof, markdown lint,
   `git diff --check`, and the default-disabled H2637 benchmark.
7. Complete review, finding disposition, verification, line-count governance,
   roadmap/catalog updates, worker handoff, and final disposition.

## Exit Criteria

- R4D process span is recorded before production Rust edits.
- Canonical authority maps to `SC-PERC-001` plus `SC-WATBAL-001`; package does
  not invent or amend process physics unless the contract gate explicitly
  requires an amendment.
- Operand lineage records units, sign, authority, and diagnostic vs
  authoritative status.
- Direct span includes direct inputs, direct compute, direct state mutation,
  downstream operands, and shadow projection.
- R4D produces `deep_seepage_m` from the dedicated direct deep-seepage handoff,
  not publication `Dp`, WB19 `Qd`, ET, snow, precipitation, R3B diagnostic
  ledger values, or storage residual compensation.
- R4B consumes R4D-produced `deep_seepage_m` and fails closed if R4D did not
  run.
- Direct-runtime source remains free of compatibility
  storage/request/writeback/symbol calls and owned legacy-symbol construction.
- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit opt-in runner fixture records positive R3A/R3B/R3C/R4A/R4B/R4C/R4D
  counters and one production compatibility handoff.
- Default-disabled H2637 median is `<= 676.67 s` and protected identity passes.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass.
- Scoped markdown lint and `git diff --check` pass.
- Direct-runtime line-count governance is current; any non-exempt 3000+ line
  file blocks closure.
- Dual review, finding disposition, dual verification, line-count governance,
  and final disposition are complete.

## Security / Safety

R4D must preserve fail-closed typed errors, avoid broad error swallowing, avoid
new dependency fallback wrappers, keep direct runtime default-disabled, and keep
direct deep-seepage state shadow-only until a later explicit publication package
proves output identity and closure.

## Closure Verdict

`COMPLETE-R4D-DIRECT-DEEP-SEEPAGE-PRODUCER`

R4D implemented the direct WB18/WB12 deep-seepage handoff producer feeding R4B
`deep_seepage_m`. The span runs
`PercolationDeepSeepage -> StorageReconciliation`, consumes
`DirectDeepSeepageInputs::deep_seepage_handoff_m`, validates it as finite and
nonnegative, mutates direct deep-seepage state and
`DirectStorageReconciliationInputs::deep_seepage_m`, produces downstream
operands, and shadow-projects the result.

R4B now fails closed when R4C storage input, R4D deep seepage, or R4A runoff has
not produced its direct upstream shadow projection. The direct executor order is
R3A -> R4C -> R4D -> R4A -> R4B -> R3B per lane, after the run-level R3C span.

R4D did not migrate full WB18 percolation physics, publish `Dp`, edit output
schemas, edit the scheduler, activate direct mode by default, or introduce
compatibility storage/request/writeback/symbol access inside the direct runtime.

Final gates passed: focused R4D/R4B/R2A tests, runner R2A counter tests,
forbidden-token no-compatibility scan, scheduler no-diff, `cargo fmt --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test
--workspace`, `cargo deny check`, markdown lint, and `git diff --check`.

Default-disabled H2637 reps were `635.94 s`, `650.91 s`, and `645.47 s`
(median `645.47 s`, threshold `<= 676.67 s`), with only the known
`MOFE01-MG-W-001` sidecar warning. PASS parquet row equivalence against the
PERFDEEP07 baseline passed with `12419` rows, `17` columns, and zero
`EXCEPT ALL` differences in both directions.
