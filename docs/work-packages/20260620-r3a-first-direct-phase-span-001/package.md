# R3A - First Direct Phase Span

Status: complete.

Package type: implementation work package / array-native runtime R3A.

## Objective

Implement the first complete direct-runtime phase span on top of the R2A
direct skeleton.

R3A must turn the R2A skeleton into a real direct phase executor for this
bounded span while preserving the architectural rule that compatibility
surfaces are edge-only. The selected span must include typed inputs, direct
compute, direct state mutation, downstream operands, and shadow projection for
identity comparison. It must not publish outputs, claim endpoint improvement,
or activate direct mode by default.

## Rationale

R2A completed the separate direct-runtime namespace and no-op/shadow executor
skeleton with a default-disabled H2637 median of `636.01 s`, under the
`<= 676.67 s` gate. The next implementation step is not another skeleton and
not a broad hydrology port. It is one complete, reviewable direct phase span
that proves direct phase dispatch, frame validation, failure semantics,
no-compatibility proof, and gate evidence can scale beyond no-op execution.

The first span must not be a validation-only dispatch exercise. It must prove
that a direct phase can consume typed inputs, compute on direct state, mutate
state, produce operands consumed by a downstream direct phase or projection,
and shadow-project those operands for identity comparison against the current
compatibility path. If execution discovers that the selected span requires
authority R3A cannot source canonically, the package must hold instead of
inventing surrogate math.

## Scope

In scope:

- implement a direct phase-span execution path for one named phase span
  selected and recorded before Rust edits;
- require the selected span to include typed inputs, direct compute, state
  mutation, downstream operands, and shadow projection;
- add typed direct phase status/error reporting for this span;
- validate direct-frame numeric domains needed by the selected span, including
  finite direct state/forcing/transfer/publication values and nonnegative
  water-depth style quantities where applicable;
- preserve signed domains where they are physically valid, such as
  temperature;
- keep valid zero-valued R2A skeleton frames acceptable unless canonical input
  constructors are added and sourced from parsed inputs;
- extend the R2A direct executor so explicit opt-in/test direct skeleton
  selection runs this complete span before returning to compatibility
  execution;
- prove the direct span contains no compatibility storage/request/writeback
  calls or owned legacy-symbol construction;
- add focused tests for valid-span execution, fail-closed invalid direct-frame
  values, direct state mutation, downstream operand production, shadow
  projection identity, default-disabled direct inactivity, and explicit opt-in
  span execution;
- preserve R2A's default-disabled H2637 median gate `<= 676.67 s`;
- update package artifacts, roadmap/catalog state, review, verification,
  disposition, and worker handoff.

Out of scope:

- broad hydrology, erosion, growth, decomposition, frost, snow, irrigation, or
  publication path migration outside the single selected span;
- WB11/WB12/WB14/WB17/WB18/WB19 process migration outside the single selected
  span and its required inputs/compute/mutation/downstream operands/shadow
  projection;
- output publication cutover;
- HBP/WAT/PASS/loss schema, unit, metadata, operand, or manifest changes;
- default activation;
- deleting compatibility runtime paths;
- adding compatibility hot-loop instrumentation that creates default-disabled
  tax;
- claiming R4 hydrology-path, R6 publication, endpoint-improvement, or
  runtime-readiness closure.

## Required Reading

Core:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/work-packages/20260620-r3a-first-direct-phase-span-001/package.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/decisions/0025-array-native-hillslope-day-frame.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/package.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/direct-frame-type-boundary-decision.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/no-compatibility-proof-plan.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/r1-frame-constructor-projection-plan.md`
- `docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/package.md`
- `docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/artifacts/disposition.md`
- `docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/artifacts/gate-results.md`
- `docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/artifacts/no-compatibility-proof-checklist.md`
- `docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/artifacts/worker-handoff.md`

Required before Rust edits:

- `crates/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`

Conditional:

- `tests/AGENTS.md` before editing root tests.
- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md` if canonical contracts
  must change.
- `tools/owcmp/AGENTS.md` before editing comparator tooling.

On-demand source inventory:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-runner/src/api.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`

## Dependencies

- R2A completed with verdict `COMPLETE-R2A-SKELETON`.
- The R0/R1 planning package defines the direct type boundary and
  no-compatibility proof method.
- `docs/architecture/array-native-runtime-specification.md` remains binding
  architecture authority.

## Intended Write Set

- `docs/work-packages/20260620-r3a-first-direct-phase-span-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs` only if new public API
  exports are required
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  only if explicit opt-in selection must call a renamed/extended direct span
  entrypoint
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` only if opt-in naming
  or help text must change

Files outside this set require package amendment before edits.

## Phase Plan

1. Populate required-reading, owned-file, and pre-implementation artifacts.
2. Inventory R2A direct runtime execution and forbidden compatibility surfaces.
3. Define the R3A span contract, including selected phase name(s), typed
   inputs, direct compute, state mutation, downstream operands, and shadow
   projection.
4. Implement direct phase-span status/error types and dispatch for the selected
   span.
5. Implement fail-closed direct frame numeric-domain validation for the span.
6. Wire explicit opt-in/test direct skeleton selection through the complete
   span, preserving default compatibility early return.
7. Add focused tests for valid execution, invalid value rejection, state
   mutation, downstream operands, shadow projection identity, default-disabled
   inactivity, opt-in execution, source-token prohibitions, runtime counters,
   and no scheduler diff.
8. Run focused Rust tests and static no-compatibility scans.
9. Run the default-disabled H2637 regression gate. Required final median:
   `<= 676.67 s`.
10. Run full Rust closure gates, scoped docs lint, and `git diff --check`.
11. Complete line-count governance, dual review, finding disposition, dual
    verification, roadmap/catalog updates, disposition, and worker handoff.

## Acceptance Criteria

- Gate: phase-span identity plus no-compatibility call-graph proof and
  non-tautological runtime counters.
- A complete direct phase span is selected and recorded before Rust edits.
- The selected span includes typed inputs, direct compute, direct state
  mutation, downstream operands, and shadow projection.
- The selected span executes in order through the R2A direct executor path.
- Phase-span identity passes for the selected fixture(s), including direct
  state mutation and shadow-projected downstream operands.
- Valid zero-valued R2A skeleton frames remain accepted unless the package adds
  canonical parsed-input constructors for the affected fields.
- Invalid direct-frame numeric domains fail closed with typed direct runtime
  errors and tests.
- Direct phase-span source contains no prohibited compatibility storage,
  request, writeback, registry, hot-table, indexed-surface, dense-refresh, or
  dirty-flush APIs.
- Static call-graph proof and focused tests show `scheduler.rs` is not edited
  and the direct span does not enter compatibility execution.
- Runtime counters are non-tautological: they must record direct phase entry,
  direct compute, state mutation, downstream operand production, shadow
  projection, and zero direct-span compatibility edge invocations. If a
  compatibility edge counter is introduced, tests must exercise a positive
  increment path or otherwise prove the counter is not an always-zero field.
- Default-disabled compatibility execution constructs no direct span state.
- Explicit opt-in/test execution runs the complete span exactly once per direct
  skeleton invocation and still returns to compatibility output publication.
- Default-disabled H2637 final median remains `<= 676.67 s` with protected
  identity.
- Full Rust closure gates pass:
  `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`.
- Scoped markdown lint and `git diff --check` pass.
- Review and verification artifacts explicitly check Gate Evidence
  Non-Deferral, no-compatibility proof adequacy, default-disabled regression,
  protected-boundary integrity, review finding disposition, and line-count
  governance.

## Conservation / Output Acceptance

R3A must not change publication operands, units, metadata meaning, output
schema, conservation math, HBP/WAT/PASS/loss construction, or manifest
publication. If execution discovers that the selected phase span requires any
publication operand or output authority change, stop and amend or hold before
implementation.

## Contract-First Rule

No `SC-*` contract change is intended. If execution discovers a required change
to guard semantics, diagnostic attribution, output meaning, units,
conservation authority, or process physics, stop and re-scope before production
edits.

## Security Impact Gate

No secrets, credentials, external network dependencies, user data, or
production host actions are in scope. New direct phase validation must fail
closed and must not silently mask missing dependencies or invalid direct-frame
inputs.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only static-audit, benchmark runner, reviewer, and verifier subagents
for no-compatibility proof review, default-disabled H2637 regression runs,
closure-gate review, line-count-governance review, package artifact review, and
gate-legitimacy verification. Expected outputs are compact metrics, log paths,
call-graph findings, and review findings recorded in package artifacts. Write
access is limited to package artifacts unless this package is explicitly
amended.

## Deliverables

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/r3a-phase-span-contract.md`
- `artifacts/direct-phase-api-plan.md`
- `artifacts/no-compatibility-proof-checklist.md`
- `artifacts/default-disabled-regression-gate.md`
- `artifacts/phase-span-identity-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/kernel-profile-compliance-checklist.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`
- `artifacts/worker-handoff.md`

## Autonomy

When authorized for execution, run this package end to end. Do not stop after
adding a dispatch function without proving the phase-span, no-compatibility,
and default-disabled gates. Do not proceed into R4 hydrology or R6 publication
implementation.
