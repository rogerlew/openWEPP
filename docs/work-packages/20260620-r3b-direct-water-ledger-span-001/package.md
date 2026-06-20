# R3B - Direct Water Ledger Span

Status: complete.

Package type: implementation work package / array-native runtime R3B.

## Objective

Extend the R3A direct-runtime harness with a second bounded direct phase span:
direct water-ledger accounting.

R3B must consume R3A direct input-accounting state plus direct water and
publication fields, compute a signed diagnostic ledger residual, mutate direct
ledger state, produce downstream operands, and shadow-project the ledger result.
It must remain below R4A hydrology-process migration risk: no WB11/WB12/WB14/
WB17/WB18/WB19 equations, no output publication cutover, no default activation,
and no endpoint-improvement claim.

## Rationale

R3A proved one complete direct span, but that span was intentionally narrow:
transfer-input accounting over typed forcing/transfer fields. R3B should prove
that the direct runtime can carry a richer dependency chain across spans without
crossing into hydrology-process authority. A water-ledger span is appropriate
because it is arithmetic over already-provided direct frame fields; it does not
decide runoff, infiltration, ET, drainage, lateral flow, storage, or publication
meaning.

## Scope

In scope:

- add an R3B direct phase span selected and recorded before Rust edits;
- consume R3A `DirectInputAccountingState` as a required upstream direct state;
- read direct water and publication fields as typed inputs;
- compute direct ledger totals and a signed diagnostic residual;
- mutate direct water-ledger state;
- produce direct downstream ledger operands;
- shadow-project the ledger result for identity/diagnostic comparison;
- validate finite/nonnegative input domains and finite signed residuals;
- update aggregate direct-runtime counters for both R3A and R3B spans;
- update focused tests and runner opt-in/default counter assertions;
- preserve default-disabled H2637 median `<= 676.67 s`;
- update package artifacts, roadmap/catalog state, reviews, verification,
  line-count governance, and disposition.

Out of scope:

- WB11/WB12/WB14/WB17/WB18/WB19 process equation migration;
- changing runoff, infiltration, ET, drainage, lateral flow, storage, or
  publication semantics;
- output publication cutover;
- HBP/WAT/PASS/loss schema, units, metadata, operands, or manifest changes;
- default activation;
- deleting or bypassing compatibility runtime paths;
- adding default-disabled instrumentation tax;
- claiming R4 hydrology-path, R6 publication, endpoint-improvement, or runtime
  readiness.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/decisions/0025-array-native-hillslope-day-frame.md`
- `docs/work-packages/20260620-r3a-first-direct-phase-span-001/package.md`
- `docs/work-packages/20260620-r3a-first-direct-phase-span-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260620-r3a-first-direct-phase-span-001/artifacts/no-compatibility-proof-checklist.md`
- `docs/work-packages/20260620-r3a-first-direct-phase-span-001/artifacts/default-disabled-regression-gate.md`
- `crates/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`

On-demand source inventory:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`

## Intended Write Set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `docs/work-packages/20260620-r3b-direct-water-ledger-span-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

No scheduler, output writer, output schema, science-contract, or compatibility
publication edit is authorized.

## Subagent Authorization

No delegated subagent work is required by this package. Review and verification
artifacts may be completed locally unless the user separately requests delegated
agents.

## Phase Plan

1. Record the R3B span contract and pre-implementation gate.
2. Add direct ledger state, downstream operands, shadow projection, constants,
   validation, and execution.
3. Extend the direct executor to run R3A then R3B for each seeded direct lane.
4. Add focused tests for exact ledger identity, signed residual, overflow and
   invalid-domain failures, aggregate counters, and default/opt-in runner
   counters.
5. Run focused tests, full Rust gates, no-compatibility proof, markdown lint,
   `git diff --check`, and the default-disabled H2637 benchmark.
6. Complete review, verification, line-count governance, roadmap/catalog
   updates, worker handoff, and disposition.

## Exit Criteria

- R3B span contract is recorded before production Rust edits.
- Direct span includes inputs, direct compute, state mutation, downstream
  operands, and shadow projection.
- R3A+R3B executor counters are deterministic and non-tautological.
- Direct-runtime source remains free of compatibility storage/request/writeback
  calls and owned legacy-symbol construction.
- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit opt-in runner fixture records positive R3A+R3B counters and one
  production compatibility handoff.
- Default-disabled H2637 median is `<= 676.67 s` and protected identity passes.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass.
- Scoped markdown lint and `git diff --check` pass.
- Dual review, finding disposition, dual verification, line-count governance,
  and final disposition are complete.

## Closure Verdict

`COMPLETE-R3B-DIRECT-WATER-LEDGER-SPAN`.

R3B closed the conservative post-R3A route. It added a second direct span with
richer state dependencies and signed diagnostic ledger projection while
preserving no-publication, no-R4, no-default-activation boundaries.

## Security / Safety

R3B must preserve fail-closed typed errors, avoid broad error swallowing, avoid
new dependency fallback wrappers, and keep direct runtime default-disabled.
