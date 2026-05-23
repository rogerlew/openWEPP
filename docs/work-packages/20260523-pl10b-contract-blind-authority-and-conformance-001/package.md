# 20260523-pl10b-contract-blind-authority-and-conformance-001

## Status
- state: complete
- date: 2026-05-23
- timezone: UTC

## Objective
Author PL process-contract authority blind to openWEPP implementation,
author contract-derived conformance tests, execute those tests against current
implementation, and reconcile discovered gaps with explicit disposition classes.

## Why This Package Exists
Operator direction requires process intent and algorithm authority to live in
contracts first, with implementation subordinate to that authority. Without a
blind authoring pass, contract text can drift into implementation-backfit.

PL10 removed first-slot dispatch coupling and opened PL11 queue entry. PL10b
inserts a contract-first conformance gate between PL10 and PL11 so event
projection work is driven by explicit authority and tests rather than ad hoc
implementation interpretation.

## Scope
### Included
- Blind authoring pass for PL transition-control/runtime-projection authority
  without reading openWEPP implementation during Phase 1.
- Canonical `SC-PLANT-001` amendment to capture algorithm details required for
  annual extension and perennial event/cycle payload semantics.
- Contract-test authoring directly from contract assertions/invariants.
- Execution of contract tests against current implementation surfaces.
- Gap reconciliation matrix with explicit classification:
  - contract defect,
  - implementation defect,
  - ambiguous authority requiring escalation.
- Follow-on queue/dependency updates based on reconciled findings.

### Explicitly Out of Scope
- Implementing new production PL kinetics (`PL12`, `PL13`).
- Tier-A comparator closeout execution (`PL14`, `PL15`).
- Risk-acceptance closure of unresolved blockers without documented authority.

## Deliverables
1. Blind-authoring protocol and source-boundary attestation:
   - `artifacts/pl10b-blind-authoring-protocol.md`
2. Canonical contract amendment plan/evidence:
   - `artifacts/pl10b-sc-plant-001-contract-amendment.md`
3. Contract-derived test specification:
   - `artifacts/pl10b-contract-test-specification.md`
4. Contract-test execution evidence against implementation:
   - `artifacts/pl10b-contract-test-execution-evidence.md`
5. Gap reconciliation matrix:
   - `artifacts/pl10b-gap-reconciliation-matrix.md`
6. Queue/dependency patch summary:
   - `artifacts/pl10b-queue-dependency-patch-summary.md`
7. Kernel profile compliance checklist:
   - `artifacts/pl10b-kernel-profile-compliance-checklist.md`
8. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl10b_disposition.md`
9. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/openwepp-vs-baseline-pl-parity-gap-register.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl09a-pre-execution-preconditions-clearance-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl10-active-slot-authority-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl10-active-slot-authority-001/artifacts/pl10_disposition.md`
- `/workdir/wepp-forest_260430_baseline`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/**`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `docs/work-packages/20260523-pl10b-contract-blind-authority-and-conformance-001/**`
- `docs/work-packages/20260523-pl11-pl-event-runtime-projection-001/package.md`
- `docs/work-packages/20260523-pl11-pl-event-runtime-projection-001/prompts/active/pl11_kickoff_agent_prompt.md`
- `docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL10 closure state and PL11 scope handoff.
- Confirm blind-authoring constraints and allowed source corpus.

### Phase 1 - Blind Contract Authoring
- Author/amend contract authority using references/literature/legacy provenance
  only; no openWEPP implementation reads in this phase.
- Record attestation and source manifest.

### Phase 2 - Contract-Test Authoring
- Derive conformance tests from contract steps/invariants/guard table.
- Record deterministic expectations and failure semantics.

### Phase 3 - Implementation Conformance Test
- Execute contract tests against current implementation.
- Capture pass/fail evidence and typed error alignment.

### Phase 4 - Gap Reconciliation
- Reconcile findings into contract-vs-implementation-vs-authority classes.
- Patch queue/dependencies and finalize disposition.

## Exit Criteria
- Blind authoring attestation is complete and evidence-backed.
- `SC-PLANT-001` contains required algorithm-detail authority for PL transition
  control/projection semantics, compliant with kernel profile requirements.
- Contract-derived test specification exists and is executable.
- Contract tests are run against current implementation with recorded evidence.
- Gap reconciliation classifies each finding with explicit next action.
- `PL11` dependency is updated to require PL10b completion.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: contract/test/governance-first package with possible bounded
  runtime projection/test harness edits only.
