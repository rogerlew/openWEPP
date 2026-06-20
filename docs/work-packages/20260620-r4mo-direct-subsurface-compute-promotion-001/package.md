# R4M/O - Direct Subsurface Compute Promotion

Status: complete.

Package type: implementation work package / array-native runtime R4M/O.

## Objective

Promote the R4D/R4E-H subsurface handoff surface into direct WB18/WB19
compute. R4M/O must own typed layer-vector percolation and subsurface
hydrology inputs, compute `D`, `Pe`, per-layer percolation fluxes, lateral
`q`, tile drainage `Qdd`, final `Qd`, layer withdrawal, carry diagnostics,
state mutation, downstream operands, and shadow projection without direct
runtime compatibility requests or symbol/writeback surfaces.

## Rationale

R4D and R4E-H proved the downstream storage-budget wiring with handoff values.
That is not enough for the full R4 direct hydrology path: the direct runtime
must execute WB18/WB19 process compute from typed layer state and feed R4B from
direct `D` and `Qd` lineage. WB17 ET/root uptake and public publication cutover
remain separate packages.

## Scope

In scope:

- direct WB18 percolation typed inputs, layer mutation, `D`, `Pe`, per-layer
  fluxes, downstream operands, and shadow projection;
- direct WB19 lateral/drainage typed inputs, layer mutation, lateral `q`,
  drainage `Qdd`, final `Qd`, carry diagnostics, downstream operands, and
  shadow projection;
- daily and hourly-relevant branch fixtures, including drainage-before-lateral
  ordering where hourly tail behavior is in scope;
- R4D/R4E/R4B wiring changes so storage reconciliation consumes direct WB18
  and WB19 results rather than R4D/R4E-H handoff inputs;
- focused direct-runtime tests, compatibility-authority parity fixtures,
  missing-upstream and invalid-domain fail-closed tests, anti-alias vectors,
  runner counter updates, package artifacts, reviews, verification, final
  gates, commit, push, and ExecPlan progress update.

Out of scope:

- WB17 evapotranspiration/root uptake compute migration;
- public `Dp`, `latqcc`, `Qd`, WB13/WAT/PASS/loss/schema publication cutover;
- scheduler edits, default activation, compatibility runtime activation, or
  output writer changes;
- changing canonical WB18/WB19 physics equations or loosening existing guards.

## Authority

Canonical authority:

- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - WB18 percolation/deep-seepage ordering, layer storage, `D`, `Pe`, and
    per-layer flux lineage;
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - WB19 lateral/drainage ordering, `q`, `Qdd`, `Qd`, carry diagnostics, and
    layer-withdrawal constraints;
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - daily water-balance ordering and storage-budget consumption constraints;
- existing compatibility kernel phase implementation as source-code authority
  for request-adapter-to-compute parity, not as a direct-runtime dependency.

R4M/O does not amend canonical `SC-*` text unless the pre-implementation
contract gate finds insufficient authority.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/r4-burndown-execplan.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

On-demand source inventory:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/00_lateral_transfer.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/01_tile_drainage.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4mo.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `docs/work-packages/20260620-r4mo-direct-subsurface-compute-promotion-001/**`
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

1. Record scope selection, process spans, operand lineage, contract gate,
   no-compatibility plan, default-disabled gate plan, and line-count baseline.
2. Add a subsurface-specific direct-runtime module so WB18/WB19 compute does
   not inflate the root direct-runtime file.
3. Add typed direct WB18 inputs/state/downstream/shadow types and request-free
   percolation compute.
4. Add typed direct WB19 inputs/state/downstream/shadow types and request-free
   drainage/lateral compute.
5. Wire R4D/R4E/R4B consumption so `D` and `Qd` are produced by direct compute
   before storage reconciliation.
6. Add focused tests for parity, branch behavior, invalid guards,
   anti-aliasing, missing-upstream fail-closed behavior, aggregate counters,
   and runner counters.
7. Run focused tests, closure gates, no-compatibility scan, scheduler no-diff,
   markdown lint, `git diff --check`, and H2637 default-disabled reps.
8. Complete review, disposition, verification, line-count governance, roadmap
   and package catalog updates, worker handoff, final disposition, commit, and
   push.

## Exit Criteria

- R4M/O process spans are recorded before production Rust edits.
- Operand lineage records units, source authority, and diagnostic vs
  authoritative status for WB18/WB19 layer storage, `D`, `Pe`, `q`, `Qdd`,
  `Qd`, withdrawals, carry arrays, and storage-budget handoffs.
- Direct producers include typed inputs, direct compute, direct state mutation,
  downstream operands, and shadow projection.
- Direct producers validate finite nonnegative layer depths, capacities,
  conductivities, water storages, fluxes, branch counters, and geometry.
- Focused fixtures compare direct WB18/WB19 results against compatibility
  kernel authority for daily and hourly-relevant branches.
- Drainage/lateral ordering, realized-withdrawal caps, `q + Qdd = Qd`, and
  carry-array production are tested.
- R4B consumes direct WB18 `D` and direct WB19 `Qd`, and fails closed if those
  producers did not run.
- Anti-alias tests distinguish `D`, `Pe`, `q`, `Qdd`, `Qd`, public `Dp`,
  public `latqcc`, root-zone ET, and storage residual.
- Direct-runtime source remains free of compatibility
  storage/request/writeback/symbol calls and owned legacy-symbol construction.
- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit opt-in runner fixture records positive counters for all direct spans
  through R4M/O and one production compatibility handoff.
- Default-disabled H2637 median is `<= 676.67 s` and protected identity passes.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass.
- Scoped markdown lint and `git diff --check` pass.
- Line-count governance is current; no touched non-exempt `.rs` file at or
  above 3000 lines remains unresolved.
- Dual review, finding disposition, dual verification, worker handoff, and
  final disposition are complete.

## Closure Verdict

COMPLETE-R4MO-DIRECT-SUBSURFACE-COMPUTE-PROMOTION.

## Security / Safety

R4M/O preserves fail-closed typed errors, direct-runtime default-disabled
behavior, no scheduler/publication cutover, and no compatibility storage access
inside direct runtime. Direct subsurface projections remain shadow-only until a
later publication-cutover package.
