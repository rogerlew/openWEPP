# R4E-H - Direct Storage-Budget Handoff Completion

Status: complete.

Package type: implementation work package / array-native runtime R4E-H.

## Objective

Complete the remaining handoff producer surface consumed by R4B direct storage
reconciliation by adding direct producers for `subsurface_loss_m` / `Qd`,
aggregate `evapotranspiration_m`, and signed `snow_coupling_m`.

R4E-H must prove that R4B no longer accepts those operands solely because tests
or executor setup initialized them. Each operand must have a typed direct input,
direct handoff compute, state mutation, downstream operand, and shadow
projection before R4B can reconcile storage.

## Rationale

R4C and R4D proved the direct upstream producer pattern for
`storage_initial_m`, `precip_input_m`, and `deep_seepage_m`. R4B still allowed
manual authoritative seeding for `Qd`, ET, and signed `S`. Grouping these
handoff producers closes the R4B storage-budget producer-completeness surface
without prematurely migrating full WB17 evapotranspiration or WB19
lateral/drainage physics.

## Scope

In scope:

- direct `Qd` / `subsurface_loss_m` handoff producer under `SC-SUBHYD-001`;
- direct aggregate ET handoff producer under daily closure authority;
- direct signed snow/frost coupling handoff producer under
  `SC-WATBAL-001#INV-WATBAL-013`;
- typed inputs, direct handoff compute, state mutation, downstream operands,
  and shadow projections for all three producers;
- R4B missing-upstream fail-closed requirements for R4E-H producers;
- focused tests for producer identity, R4B consumption, invalid guards,
  anti-alias vectors, aggregate executor counters, and runner counters;
- package artifacts, reviews, verification, line-count governance, final gates,
  commit, push, and ExecPlan progress update.

Out of scope:

- full WB17 evapotranspiration/root uptake compute migration;
- full WB19 lateral/drainage compute migration;
- public WB13/WAT/PASS/loss/schema publication cutover;
- scheduler edits, compatibility runtime edits, default activation, or output
  schema/manifest changes;
- treating publication `latqcc`, `Dp`, `Ep`, `Es`, `Er`, `RM`, `Snow-Water`,
  `frozwt`, residual compensation, or diagnostic ledger values as R4E-H
  authority.

## Authority

Canonical authority:

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - WB12 Reconciliation Authority Addendum;
  - `INV-WATBAL-013`;
  - `INV-WATBAL-016`;
  - `INV-WATBAL-034`;
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - Chapter-5 `Qd` coupling;
  - `INV-SUBHYD-009`;
  - `INV-SUBHYD-021`;
  - `INV-SUBHYD-023`;
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
  - daily ET withdrawal term for water-balance closure;
  - full WB17 compute remains out of scope until R4N.

R4E-H is a handoff migration package under existing contract authority. It does
not amend canonical `SC-*` text unless the pre-implementation contract gate
finds insufficient authority.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/r4-burndown-execplan.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/work-packages/20260620-r4b-direct-storage-reconciliation-consumer-001/package.md`
- `docs/work-packages/20260620-r4c-direct-storage-input-producer-001/package.md`
- `docs/work-packages/20260620-r4d-direct-deep-seepage-producer-001/package.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

On-demand source inventory:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `docs/work-packages/20260620-r4eh-direct-storage-budget-handoff-completion-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/r4-burndown-execplan.md`
- `docs/ROADMAP.md`

No scheduler, output writer, output schema, dependency, or compatibility
runtime edit is authorized.

## Subagent Authorization

No delegated subagent work is required by this package. Review and verification
artifacts may be completed locally unless the user separately requests
delegated agents.

## Phase Plan

1. Record producer selection, process spans, operand lineage, contract gate,
   no-compatibility plan, default-disabled gate plan, and line-count baseline.
2. Add R4E-H direct producer types, validation, compute, downstream operands,
   and shadow projections.
3. Extend R4B completeness checks so storage reconciliation requires R4C, R4D,
   R4E-H, and R4A upstream shadows.
4. Extend the direct executor order so R4E-H runs after R4D and before R4A/R4B.
5. Add focused tests for identity, invalid input guards, anti-alias vectors,
   missing-upstream fail-closed behavior, aggregate counters, and runner
   counters.
6. Run focused tests, closure gates, no-compatibility scan, scheduler no-diff,
   markdown lint, `git diff --check`, and H2637 default-disabled reps.
7. Complete review, disposition, verification, line-count governance, roadmap
   and package catalog updates, worker handoff, final disposition, commit, and
   push.

## Exit Criteria

- R4E-H process spans are recorded before production Rust edits.
- Operand lineage records units, sign, source authority, and diagnostic vs
  authoritative status for `Qd`, ET, and signed `S`.
- Direct producers include typed inputs, direct handoff compute, direct state
  mutation, downstream operands, and shadow projection.
- `Qd` and aggregate ET validate finite nonnegative inputs; signed `S`
  validates finite positive/negative/zero coupling.
- R4B consumes R4E-H-produced values and fails closed if any R4E-H producer did
  not run.
- Anti-alias tests distinguish R4E-H operands from adjacent
  process/publication/diagnostic/residual substitutes.
- Direct-runtime source remains free of compatibility
  storage/request/writeback/symbol calls and owned legacy-symbol construction.
- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit opt-in runner fixture records positive counters for all direct spans
  through R4E-H and one production compatibility handoff.
- Default-disabled H2637 median is `<= 676.67 s` and protected identity passes.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass.
- Scoped markdown lint and `git diff --check` pass.
- Line-count governance is current; no touched non-exempt `.rs` file at or
  above 3000 lines remains unresolved.
- Dual review, finding disposition, dual verification, worker handoff, and
  final disposition are complete.

## Closure Verdict

PASS. R4E-H added direct producers for subsurface-loss, aggregate
evapotranspiration, and signed snow-coupling handoffs. R4B now fails closed
unless those producers ran, focused and workspace gates passed, and the H2637
default-disabled median remained below the `676.67 s` regression threshold.

## Security / Safety

R4E-H preserves fail-closed typed errors, direct-runtime default-disabled
behavior, no scheduler/publication cutover, and no compatibility storage access
inside direct runtime. Handoff values are shadow-only until later R4/R6 packages
promote full compute and publication surfaces.
