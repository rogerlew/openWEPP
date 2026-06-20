# R4A - Direct Runoff-Partition Process Span

Status: complete.

Package type: implementation work package / array-native runtime R4A.

## Objective

Implement the first narrow direct hydrology-process span in the array-native
runtime: direct WB12 runoff-partition closure for an explicitly supplied direct
liquid/runon/infiltration/depression-storage boundary.

R4A must prove that the direct runtime can execute one contract-authoritative
hydrology process equation slice without compatibility storage, request,
writeback, symbol lookup, publication cutover, or default activation.

## Rationale

R3A-R3C proved the direct runtime mechanics needed before process migration:
typed inputs, direct compute, direct state mutation, downstream operands, shadow
projection, per-lane and run-level aggregation, phase-span identity, and
no-compatibility proof. R4A should now move one small process equation into the
direct runtime while avoiding broad WB12/WB14 scope.

The selected slice is the SC-RUNOFFPART runoff residual closure:

```text
partition_runoff_m =
  liquid_input_m + runon_input_m
  - cumulative_infiltration_m
  - depression_storage_delta_m

q_runoff_m = partition_runoff_m + surface_saturation_runoff_m
```

with explicit finite/nonnegative domain guards and direct-only mutation of
`DirectWaterState::{infiltration_m, runoff_m}`.

## Scope

In scope:

- scaffold R4A package artifacts before production Rust edits;
- add an R4A direct phase span selected and recorded before production edits;
- add direct runoff-partition input/state/downstream/shadow types;
- consume explicit direct liquid, runon, cumulative infiltration,
  depression-storage delta, and surface-saturation addback operands;
- compute SC-RUNOFFPART runoff residual closure from direct operands only;
- mutate direct water state for infiltration and runoff only;
- produce direct downstream runoff operands;
- shadow-project direct runoff-partition closure;
- validate finite/nonnegative operands, non-over-infiltration, nonnegative
  partition runoff, nonnegative `Q`, and finite closure residual;
- update aggregate direct-runtime counters for R3A/R3B/R3C/R4A spans;
- update focused tests and runner opt-in/default counter assertions;
- preserve default-disabled H2637 median `<= 676.67 s`;
- update package artifacts, roadmap/catalog state, reviews, verification,
  line-count governance, and disposition.

Out of scope:

- full WB12/WB14 runoff partition migration;
- Green-Ampt infiltration solving;
- canopy/residue interception, snowmelt, irrigation, frost, peak runoff,
  erosion, WB18, WB19, ET, plant uptake, storage reconciliation, or publication
  cutover;
- HBP/WAT/PASS/loss schema, units, metadata, operands, or manifest changes;
- scheduler or compatibility runtime edits;
- deleting or bypassing compatibility production paths;
- default activation;
- endpoint-improvement, R5/R6 publication, or full R4 hydrology-path readiness
  claims.

## Authority

Canonical authority:

- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  - `INV-RUNOFFPART-001`
  - `INV-RUNOFFPART-002`
  - `INV-RUNOFFPART-009`
  - `INV-RUNOFFPART-014`
  - `INV-RUNOFFPART-016`
  - `INV-RUNOFFPART-027`

R4A is a direct-runtime migration slice under existing contract authority; it
does not amend canonical `SC-*` text.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/decisions/0025-array-native-hillslope-day-frame.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/work-packages/20260620-r3c-direct-multilane-transfer-span-001/package.md`
- `docs/work-packages/20260620-r3c-direct-multilane-transfer-span-001/artifacts/worker-handoff.md`
- `crates/AGENTS.md`

On-demand source inventory:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `docs/work-packages/20260620-r4a-direct-runoff-partition-span-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

No scheduler, output writer, output schema, science-contract, dependency, or
compatibility runtime edit is authorized.

## Subagent Authorization

No delegated subagent work is required by this package. Review and verification
artifacts may be completed locally unless the user separately requests delegated
agents.

## Phase Plan

1. Record the R4A process-span contract, operand lineage, and
   pre-implementation contract gate.
2. Add direct runoff-partition input/state/downstream/shadow types, validation,
   constants, and span execution.
3. Extend the direct executor to run R4A after R3A and before R3B for each
   seeded direct lane/day frame.
4. Add focused tests for exact direct runoff-partition identity,
   non-aliased/anti-tautology vectors, invalid input failures, aggregate
   counters, and default/opt-in runner counters.
5. Run focused tests, full Rust gates, no-compatibility proof, markdown lint,
   `git diff --check`, and the default-disabled H2637 benchmark.
6. Complete review, verification, line-count governance, roadmap/catalog
   updates, worker handoff, and disposition.

## Exit Criteria

- R4A process span is recorded before production Rust edits.
- Canonical authority maps to `SC-RUNOFFPART-001`; package does not invent or
  amend process physics.
- Operand lineage records units, normalization, authority, and diagnostic vs
  authoritative status.
- Direct span includes explicit inputs, direct compute, direct state mutation,
  downstream operands, and shadow projection.
- Tests separate accepted formula from wrong aliases such as precipitation-only,
  runoff without depression storage, runoff without saturation addback, and
  infiltration-as-runoff.
- Direct-runtime source remains free of compatibility storage/request/writeback
  calls and owned legacy-symbol construction.
- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit opt-in runner fixture records positive R3A/R3B/R3C/R4A counters and
  one production compatibility handoff.
- Default-disabled H2637 median is `<= 676.67 s` and protected identity passes.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass.
- Scoped markdown lint and `git diff --check` pass.
- Dual review, finding disposition, dual verification, line-count governance,
  and final disposition are complete.

## Security / Safety

R4A must preserve fail-closed typed errors, avoid broad error swallowing, avoid
new dependency fallback wrappers, keep direct runtime default-disabled, and avoid
using diagnostic R3 ledgers as process authority.

## Closure Verdict

`COMPLETE-R4A-DIRECT-RUNOFF-PARTITION-SPAN`.

R4A implemented the first direct hydrology-process span: a narrow
SC-RUNOFFPART-authoritative runoff-partition closure slice. It preserved
no-publication, no-default-activation, no scheduler edit, and no compatibility
storage/request/writeback boundaries while passing full Rust gates and the
default-disabled H2637 regression gate.
