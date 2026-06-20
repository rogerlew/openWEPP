# R4N - Direct WB17 ET And Root-Uptake Compute Promotion

Status: complete.

Package type: implementation work package / array-native runtime R4N.

## Objective

Promote the R4E-H aggregate evapotranspiration handoff into direct WB17
evapotranspiration and root-uptake compute. R4N must own typed WB17 inputs,
surface/residue ET, soil-evaporation layer mutation, post-WB19 plant root
uptake, aggregate ET, downstream operands, and shadow projection without direct
runtime compatibility requests or symbol/writeback surfaces.

## Rationale

R4E-H proved that R4B can consume an aggregate ET producer, but the producer was
handoff-only. Full R4 direct hydrology requires WB17 lineage from typed layer
state. WB17 is split by canonical ordering: soil/residue evaporation consumes
post-WB18 layer state before WB19 drainage/lateral transfer, while plant root
uptake consumes post-WB19 layer state before WB12 storage reconciliation.

## Scope

In scope:

- direct WB17 typed inputs, stage/PMET options, layer state, component ET,
  root-uptake vectors, downstream operands, and shadow projection;
- an R4N surface ET span after R4M and before R4O;
- an R4N root-uptake/final ET span after R4O and before R4B;
- R4O consumption of the R4N soil-evaporation-mutated layer vector when present;
- R4B consumption of final R4N aggregate ET rather than the R4E-H handoff;
- focused direct-runtime tests, invalid-domain and missing-upstream tests,
  anti-alias vectors, runner counter updates, package artifacts, reviews,
  verification, final gates, commit, push, and ExecPlan progress update.

Out of scope:

- public WB13/WAT/PASS/loss/schema ET publication cutover;
- changing canonical WB17 equations, scheduler order, default activation, or
  compatibility runtime activation;
- replacing the legacy R4F handoff scaffold API outside the aggregate executor.

## Authority

Canonical authority:

- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
  - WB17 surface ET, residue interception, soil evaporation, PMET/stage
    options, layer mutation, SWU/root uptake, `Ep`, `Es`, `Er`, `ET`, `Ws`,
    `UPi`, `Ui`, and `pltol` normalization;
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - WB11 daily ordering: WB18 percolation, WB17 ET, WB19 drainage/lateral,
    SWU/root uptake, and WB12 storage reconciliation;
- existing compatibility kernel phase implementation as source-code authority
  for adapter-to-compute parity, not as a direct-runtime dependency.

R4N does not amend canonical `SC-*` text unless the pre-implementation
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
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

On-demand source inventory:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_infiltration_evap.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `tests/integration/wb17_et_physics_kernel_contract.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4n.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `docs/work-packages/20260620-r4n-direct-wb17-et-root-uptake-compute-001/**`
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
2. Add an evapotranspiration-specific direct-runtime module so WB17 compute
   stays outside the root direct-runtime file.
3. Add typed direct WB17 inputs/state/downstream/shadow types and request-free
   soil/residue ET compute.
4. Add post-WB19 root-uptake compute, final aggregate ET, and R4B handoff.
5. Wire R4O and aggregate executor ordering so ET-mutated layer state feeds
   WB19 and final R4N ET feeds R4B.
6. Add focused tests for parity, ordering, invalid guards, anti-aliasing,
   missing-upstream fail-closed behavior, aggregate counters, and runner
   counters.
7. Run focused tests, closure gates, no-compatibility scan, scheduler no-diff,
   markdown lint, `git diff --check`, and H2637 default-disabled reps.
8. Complete review, disposition, verification, line-count governance, roadmap
   and package catalog updates, worker handoff, final disposition, commit, and
   push.

## Exit Criteria

- R4N process spans are recorded before production Rust edits.
- Operand lineage records units, source authority, and diagnostic vs
  authoritative status for component ET, layer storage, root uptake, `Ws`,
  `UPi`, `Ui`, and aggregate ET.
- Direct producers include typed inputs, direct compute, direct state mutation,
  downstream operands, and shadow projection.
- Direct producers validate finite nonnegative layer depths, water storages,
  ET demand, residue interception, root depth, upper limits, and stage/PMET
  options.
- Focused fixtures compare direct WB17 results against compatibility kernel
  authority for soil evaporation, residue handling, root uptake, and layer
  mutation.
- R4O consumes the R4N soil-evaporation-mutated layer vector, and R4B consumes
  final R4N aggregate ET.
- R4B fails closed if final R4N ET did not run.
- Anti-alias tests distinguish aggregate ET from `Ep`, `Es`, `Er`, storage
  residual, handoff ET, and publication-side ET reconstruction.
- Direct-runtime source remains free of compatibility
  storage/request/writeback/symbol calls and owned legacy-symbol construction.
- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit opt-in runner fixture records positive counters for all direct spans
  through R4N and one production compatibility handoff.
- Default-disabled H2637 median is `<= 676.67 s` and protected identity passes.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass.
- Scoped markdown lint and `git diff --check` pass.
- Line-count governance is current; no touched non-exempt `.rs` file at or
  above 3000 lines remains unresolved.
- Dual review, finding disposition, dual verification, worker handoff, and
  final disposition are complete.

## Closure Verdict

COMPLETE-R4N-DIRECT-WB17-ET-ROOT-UPTAKE-COMPUTE-PROMOTION.

R4N implemented request-free direct WB17 evapotranspiration and post-WB19
root-uptake compute. The package added typed inputs, direct compute, direct
state mutation, downstream operands, and shadow projection for the surface ET
and final root-uptake spans. R4O now consumes the R4N soil-evaporation-mutated
layer vector when present, and R4B consumes final aggregate ET from R4N instead
of the prior R4E-H aggregate ET handoff.

R4N preserved the no-publication, no-default-activation, and no-scheduler
boundary. Public WB13/WAT/PASS/loss outputs remain compatibility-authoritative.
All focused and full Rust gates passed, and the default-disabled H2637 median
was `649.22 s`, below the `<= 676.67 s` gate, with protected PASS row
equivalence.

## Security / Safety

R4N preserves fail-closed typed errors, direct-runtime default-disabled
behavior, no scheduler/publication cutover, and no compatibility storage access
inside direct runtime. Direct WB17 projections remain shadow-only until a later
publication-cutover package.
