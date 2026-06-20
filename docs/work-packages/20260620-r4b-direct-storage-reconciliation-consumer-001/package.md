# R4B - Direct Storage-Reconciliation Consumer Span

Status: complete.

Package type: implementation work package / array-native runtime R4B.

## Objective

Implement the next narrow direct hydrology-process span after R4A: a direct
WB12 storage-reconciliation consumer of the R4A runoff result.

R4B must prove that the direct runtime can consume one direct process result
downstream, compute WB12 storage reconciliation from canonical operands, mutate
direct storage state, produce downstream operands, and shadow-project the
storage result without compatibility storage, request/writeback APIs, symbol
lookup, publication cutover, scheduler edits, or default activation.

## Rationale

R4A moved the first narrow runoff-partition equation slice into the direct
runtime. The safest next package is the downstream route from the R4A handoff:
consume R4A `Q` as a direct operand and reconcile storage under the existing
WB12 storage equation. This expands the process chain without taking on broad
upstream liquid assembly, WB18/WB19 producer migration, or public output
authority.

The selected storage slice is:

```text
storage_reconciled_m =
  storage_initial_m
  + precip_input_m
  + snow_coupling_m
  - q_runoff_m
  - evapotranspiration_m
  - deep_seepage_m
  - subsurface_loss_m
```

where `q_runoff_m` must be consumed from the R4A direct downstream runoff
operand, not reconstructed from compatibility surfaces or diagnostic ledgers.

## Scope

In scope:

- record the R4B process-span contract, operand lineage, and
  pre-implementation contract gate before production Rust edits;
- add direct storage-reconciliation input/state/downstream/shadow types;
- consume R4A `DirectRunoffDownstreamOperands::q_runoff_m` as the direct `Q`
  input;
- consume explicit direct storage inputs for initial storage, precipitation
  input, signed snow coupling `S`, ET, deep seepage `D`, subsurface loss `Qd`,
  and closure tolerance;
- compute the `SC-WATBAL-001` WB12 storage-reconciliation equation from direct
  operands only;
- mutate direct storage state only, initially `DirectWaterState::soil_water_m`
  or a narrower direct storage field if implementation splits one out first;
- produce direct downstream storage operands;
- shadow-project direct storage reconciliation and closure residual;
- validate finite operands, nonnegative storage/loss terms, signed finite `S`,
  nonnegative reconciled storage, and closure residual within tolerance;
- update aggregate direct-runtime counters for R3A/R3B/R3C/R4A/R4B spans;
- update focused tests and runner opt-in/default counter assertions;
- preserve default-disabled H2637 median `<= 676.67 s`;
- update package artifacts, roadmap/catalog state, reviews, verification,
  line-count governance, and disposition.

Out of scope:

- upstream liquid/input assembly, interception, snowmelt, irrigation, frost,
  Green-Ampt infiltration, WB18, WB19, ET producer, percolation producer, or
  lateral-flow producer migration;
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
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - `INV-SUBHYD-022`
  - `INV-SUBHYD-023`

R4B is a direct-runtime migration slice under existing contract authority. It
does not amend canonical `SC-*` text unless the pre-implementation contract
gate finds that the selected storage operands are not sufficiently authoritative.

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
- `docs/work-packages/20260620-r4a-direct-runoff-partition-span-001/package.md`
- `docs/work-packages/20260620-r4a-direct-runoff-partition-span-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260620-r4a-direct-runoff-partition-span-001/artifacts/r4a-process-span-contract.md`
- `docs/work-packages/20260620-r4a-direct-runoff-partition-span-001/artifacts/operand-lineage.md`
- `crates/AGENTS.md`

On-demand source inventory:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_storage_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `docs/work-packages/20260620-r4b-direct-storage-reconciliation-consumer-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

No scheduler, output writer, output schema, dependency, or compatibility
runtime edit is authorized. Science-contract edits are authorized only if the
pre-implementation contract gate proves the selected R4B storage slice lacks
canonical authority.

## Subagent Authorization

No delegated subagent work is required by this package. Review and verification
artifacts may be completed locally unless the user separately requests delegated
agents.

## Phase Plan

1. Confirm the selected R4B storage slice, operand lineage, and contract gate.
2. Add direct storage-reconciliation input/state/downstream/shadow types,
   constants, validation, and execution.
3. Extend the direct executor to run R4B after R4A and before the diagnostic R3B
   ledger for each seeded direct lane/day frame, unless implementation review
   proves a safer ordering and records it before Rust edits.
4. Add focused tests for exact storage-reconciliation identity,
   anti-tautology/anti-alias vectors, invalid input failures, aggregate
   counters, and default/opt-in runner counters.
5. Run focused tests, full Rust gates, no-compatibility proof, markdown lint,
   `git diff --check`, and the default-disabled H2637 benchmark.
6. Complete review, finding disposition, verification, line-count governance,
   roadmap/catalog updates, worker handoff, and final disposition.

## Exit Criteria

- R4B process span is recorded before production Rust edits.
- Canonical authority maps to `SC-WATBAL-001`; package does not invent or amend
  process physics unless the contract gate explicitly requires an amendment.
- Operand lineage records units, normalization, sign, authority, and diagnostic
  vs authoritative status.
- Direct span includes explicit inputs, direct compute, direct state mutation,
  downstream operands, and shadow projection.
- R4B consumes R4A direct `q_runoff_m`; tests reject reconstructing `Q` from
  compatibility state, publication fields, or R3B diagnostic ledger values.
- Tests separate accepted storage formula from wrong aliases such as omitted
  `S`, wrong `Q` sign, omitted ET/D/Qd, swapped ET/D/Qd terms, publication
  storage targets, and initial-storage/precipitation aliasing.
- Direct-runtime source remains free of compatibility storage/request/writeback
  calls and owned legacy-symbol construction.
- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit opt-in runner fixture records positive R3A/R3B/R3C/R4A/R4B counters
  and one production compatibility handoff.
- Default-disabled H2637 median is `<= 676.67 s` and protected identity passes.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass.
- Scoped markdown lint and `git diff --check` pass.
- Direct-runtime line-count governance is current. If a touched `.rs` file
  crosses 2000 lines, record a WARN disposition and prefer a narrow split before
  closure; any non-exempt 3000+ line file blocks closure.
- Dual review, finding disposition, dual verification, line-count governance,
  and final disposition are complete.

## Security / Safety

R4B must preserve fail-closed typed errors, avoid broad error swallowing, avoid
new dependency fallback wrappers, keep direct runtime default-disabled, and keep
R4A/R4B direct state shadow-only until a later explicit publication package
proves output identity and closure.

## Closure Verdict

`COMPLETE-R4B-DIRECT-STORAGE-RECONCILIATION-CONSUMER-SPAN`

R4B implemented a direct WB12 storage-reconciliation consumer span after R4A.
It consumes R4A direct `q_runoff_m`, computes storage reconciliation from
explicit direct operands, mutates only direct storage state, produces direct
downstream storage operands, and shadow-projects the result. It does not edit
publication, scheduler, compatibility runtime, output schemas, or default
activation.

Closure evidence:

- full Rust gates passed;
- no-compatibility proof passed;
- default-disabled H2637 median was `641.14 s`, below the `<= 676.67 s` gate;
- protected output identity and PASS parquet row equivalence passed;
- `direct_runtime.rs` entered the 2000+ line WARN band at 2101 lines, below the
  non-exempt 3000-line blocker.
