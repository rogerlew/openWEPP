# R3C - Direct Multi-Lane Transfer Span

Status: complete.

Package type: implementation work package / array-native runtime R3C.

## Objective

Extend the R3A/R3B direct-runtime harness with a bounded run-level direct span:
multi-lane transfer/topology propagation.

R3C must consume direct lane topology and lane transfer buffers, compute a
diagnostic transfer ledger across lanes, mutate direct run-level state, produce
downstream operands, and shadow-project the run-level transfer result. It must
remain below R4A hydrology-process migration risk: no WB11/WB12/WB14/WB17/WB18/
WB19 equations, no output publication cutover, no default activation, and no
endpoint-improvement claim.

## Rationale

R3A proved a complete lane/day direct input-accounting span. R3B proved a richer
lane/day direct water-ledger dependency chain. R3C should prove that the direct
runtime can also carry run-level, multi-lane transfer and topology state without
touching compatibility storage or hydrology-process physics. This narrows the
remaining architecture gap before R4 process migration: direct runtime must be
able to represent lane-to-lane propagation surfaces, not only isolated lane/day
arithmetic.

## Scope

In scope:

- add an R3C direct phase span selected and recorded before Rust edits;
- consume `DirectLaneFrame` topology, area metadata, and transfer buffers;
- compute run-level lane transfer ledger values from direct buffers only;
- mutate direct run-level transfer ledger state;
- produce direct downstream transfer operands;
- shadow-project run-level transfer totals;
- validate finite/nonnegative topology and transfer input domains;
- update aggregate direct-runtime counters for R3A, R3B, and R3C spans;
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
- `docs/work-packages/20260620-r3b-direct-water-ledger-span-001/package.md`
- `docs/work-packages/20260620-r3b-direct-water-ledger-span-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260620-r3b-direct-water-ledger-span-001/artifacts/no-compatibility-proof-checklist.md`
- `docs/work-packages/20260620-r3b-direct-water-ledger-span-001/artifacts/default-disabled-regression-gate.md`
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
- `docs/work-packages/20260620-r3c-direct-multilane-transfer-span-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

No scheduler, output writer, output schema, science-contract, or compatibility
publication edit is authorized.

## Subagent Authorization

No delegated subagent work is required by this package. Review and verification
artifacts may be completed locally unless the user separately requests delegated
agents.

## Phase Plan

1. Record the R3C span contract and pre-implementation gate.
2. Add direct run-level transfer ledger state, downstream operands, shadow
   projection, constants, validation, and execution.
3. Extend the direct executor to run R3C once per direct run plus R3A/R3B for
   each seeded direct lane.
4. Add focused tests for exact multi-lane topology projection, invalid topology
   and transfer input failures, aggregate counters, and default/opt-in runner
   counters.
5. Run focused tests, full Rust gates, no-compatibility proof, markdown lint,
   `git diff --check`, and the default-disabled H2637 benchmark.
6. Complete review, verification, line-count governance, roadmap/catalog
   updates, worker handoff, and disposition.

## Exit Criteria

- R3C span contract is recorded before production Rust edits.
- Direct span includes inputs, direct compute, state mutation, downstream
  operands, and shadow projection.
- R3A+R3B+R3C executor counters are deterministic and non-tautological.
- Direct-runtime source remains free of compatibility storage/request/writeback
  calls and owned legacy-symbol construction.
- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit opt-in runner fixture records positive R3A+R3B+R3C counters and one
  production compatibility handoff.
- Default-disabled H2637 median is `<= 676.67 s` and protected identity passes.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass.
- Scoped markdown lint and `git diff --check` pass.
- Dual review, finding disposition, dual verification, line-count governance,
  and final disposition are complete.

## Closure Verdict

`COMPLETE-R3C-DIRECT-MULTILANE-TRANSFER-SPAN`.

R3C added a run-level direct span for multi-lane transfer/topology propagation
while preserving no-publication, no-R4, no-default-activation boundaries. It
proved direct inputs, direct compute, state mutation, downstream operands,
shadow projection, phase-span identity, no-compatibility source/runtime proof,
and the default-disabled H2637 regression gate.

## Security / Safety

R3C must preserve fail-closed typed errors, avoid broad error swallowing, avoid
new dependency fallback wrappers, and keep direct runtime default-disabled.
