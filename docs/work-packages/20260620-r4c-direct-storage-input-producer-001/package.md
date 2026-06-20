# R4C - Direct Storage-Input Producer

Status: complete.

Package type: implementation work package / array-native runtime R4C.

## Objective

Implement the next post-R4B direct-runtime slice: a direct WB12 storage-input
producer that feeds R4B with canonical direct `storage_initial_m` and
`precip_input_m` operands.

R4C must prove that R4B can consume a second direct upstream producer, not just
explicit package-local storage inputs. It must also address the R4B line-count
WARN by moving storage-related direct-runtime code into a narrow module split
before adding more direct spans.

## Rationale

R4B reconciled storage from explicit operands and consumed R4A direct `Q`.
The lowest-risk next producer is the storage-input assembly boundary:

- `storage_initial_m` comes from current direct `DirectWaterState::soil_water_m`
  before R4B mutates reconciled storage;
- `precip_input_m` comes from R3A direct precipitation accounting;
- `S`, ET, `D`, `Qd`, and tolerance remain explicit direct R4B inputs until
  their producers are migrated one at a time.

This creates a longer direct producer/consumer chain without migrating WB18,
WB19, ET, snow, irrigation, or public output authority.

## Scope

In scope:

- record R4C process-span contract, operand lineage, and pre-implementation
  contract gate before Rust edits;
- split storage-related direct-runtime code into a narrow Rust submodule so
  `direct_runtime.rs` leaves or materially reduces the 2000+ line WARN band;
- add direct storage-input state, downstream operand, shadow projection, and
  span-report types;
- add a direct R4C span that consumes R3A direct precipitation plus current
  direct soil storage;
- mutate direct storage-input state and the R4B
  `DirectStorageReconciliationInputs::storage_initial_m` and
  `DirectStorageReconciliationInputs::precip_input_m` fields;
- require R4C as a direct upstream for R4B storage reconciliation;
- keep `snow_coupling_m`, `evapotranspiration_m`, `deep_seepage_m`,
  `subsurface_loss_m`, and `closure_tolerance_m` as explicit R4B inputs;
- update aggregate direct-runtime counters for R3A/R3B/R3C/R4A/R4B/R4C spans;
- add focused tests for R4C identity, downstream operands, R4B consumption,
  anti-alias vectors, invalid input failures, and runner counters;
- preserve default-disabled H2637 median `<= 676.67 s`;
- update package artifacts, roadmap/catalog state, reviews, verification,
  line-count governance, and disposition.

Out of scope:

- ET, percolation/deep seepage, WB19 `Qd`, snow coupling, irrigation,
  interception, Green-Ampt infiltration, carry-array, WB18, WB19, or full WB12
  producer migration;
- public WB13/WAT/HBP/PASS/loss schema, units, metadata, operands, or manifest
  changes;
- scheduler or compatibility runtime edits;
- deleting or bypassing compatibility production paths;
- default activation;
- endpoint-improvement, R5/R6 publication, or full R4 hydrology-path readiness
  claims;
- using R3B diagnostic ledger fields as storage authority.

## Authority

Canonical authority:

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `WB12 Reconciliation Authority Addendum`
  - `INV-WATBAL-013`
  - `INV-WATBAL-016`
  - `INV-WATBAL-034`
  - `HPHYS0239 WB19->WB12->WB13 Ordering and Flux-Authority Handoff Addendum`
  - `HPHYS0242 WB14/WB12 Hourly Cadence and Ordering Addendum`
- `docs/work-packages/20260620-r4b-direct-storage-reconciliation-consumer-001/`
  - R4B operand lineage and worker handoff

R4C is a direct-runtime migration slice under existing WB12 contract authority.
It does not amend canonical `SC-*` text unless the pre-implementation contract
gate finds that `storage_initial_m` or `precip_input_m` lacks sufficient
authority for this narrow producer slice.

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
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/work-packages/20260620-r4b-direct-storage-reconciliation-consumer-001/package.md`
- `docs/work-packages/20260620-r4b-direct-storage-reconciliation-consumer-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260620-r4b-direct-storage-reconciliation-consumer-001/artifacts/operand-lineage.md`
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
- `docs/work-packages/20260620-r4c-direct-storage-input-producer-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

No scheduler, output writer, output schema, dependency, or compatibility runtime
edit is authorized. Science-contract edits are authorized only if the
pre-implementation contract gate proves the selected R4C storage-input slice
lacks canonical authority.

## Subagent Authorization

No delegated subagent work is required by this package. Review and verification
artifacts may be completed locally unless the user separately requests delegated
agents.

## Phase Plan

1. Confirm the selected R4C storage-input slice, operand lineage, contract gate,
   and storage-module split plan.
2. Move storage-related direct-runtime code into a narrow storage submodule.
3. Add direct storage-input producer types, validation, execution, downstream
   operands, and shadow projection.
4. Require R4C storage input before R4B storage reconciliation.
5. Extend the direct executor to run R4C after R3A and before R4A/R4B.
6. Add focused tests for exact storage-input identity, R4B consumption,
   anti-alias vectors, invalid input failures, aggregate counters, and runner
   counters.
7. Run focused tests, full Rust gates, no-compatibility proof, markdown lint,
   `git diff --check`, and the default-disabled H2637 benchmark.
8. Complete review, finding disposition, verification, line-count governance,
   roadmap/catalog updates, worker handoff, and final disposition.

## Exit Criteria

- R4C process span is recorded before production Rust edits.
- Canonical authority maps to `SC-WATBAL-001`; package does not invent or amend
  process physics unless the contract gate explicitly requires an amendment.
- Operand lineage records units, sign, authority, and diagnostic vs
  authoritative status.
- Direct span includes direct inputs, direct compute, direct state mutation,
  downstream operands, and shadow projection.
- R4C consumes R3A direct precipitation and direct storage state; tests reject
  total-accounted-input, transfer-input, runoff-input, publication, and R3B
  diagnostic-ledger aliases as `precip_input_m` or `storage_initial_m`.
- R4B consumes R4C-produced storage initial and precipitation operands and fails
  closed if R4C did not run.
- Direct-runtime source remains free of compatibility
  storage/request/writeback/symbol calls and owned legacy-symbol construction.
- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit opt-in runner fixture records positive R3A/R3B/R3C/R4A/R4B/R4C
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

R4C must preserve fail-closed typed errors, avoid broad error swallowing, avoid
new dependency fallback wrappers, keep direct runtime default-disabled, and keep
direct storage state shadow-only until a later explicit publication package
proves output identity and closure.

## Closure Verdict

`COMPLETE-R4C-DIRECT-STORAGE-INPUT-PRODUCER`.

R4C implemented the direct storage-input producer and storage-module split
without scheduler, compatibility-runtime, publication, output schema, or default
activation changes. Full Rust gates passed, no-compatibility proof passed, and
the default-disabled H2637 gate passed at `637.63 s`, `640.25 s`, and
`639.19 s` (median `639.19 s`, threshold `<= 676.67 s`) with protected output
identity and PASS DuckDB row equivalence.
