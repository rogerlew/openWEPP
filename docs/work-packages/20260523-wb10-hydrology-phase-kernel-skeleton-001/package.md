# 20260523-wb10-hydrology-phase-kernel-skeleton-001

## Status
- state: complete
- date: 2026-05-23
- timezone: UTC

## Objective
Add production hydrology kernel entry scaffolding for ET/percolation/lateral
flow/drainage/runoff/storage phase classes (non-probe implementation path) in
the monolithic openWEPP scientific hydrology/erosion model.

## Why This Package Exists
The PL09 hold-lift queue defines `WB10` as the first water-balance lane
implementation package. Current posture requires replacing probe/test-only
hydrology execution paths with explicit production kernel-entry routing before
WB11..WB13 kernel implementation and coupling validation.

This package is contract-first: process authority and contract-derived
pre-implementation gate tests are required before production hydrology routing
code edits are considered complete.

## Scope
### Included
- Add production hydrology kernel entry scaffolding for phase classes:
  - evapotranspiration,
  - percolation/deep seepage,
  - lateral transfer,
  - drainage,
  - runoff reconciliation,
  - storage reconciliation.
- Wire scaffolding through scheduler phase-class dispatch with typed routing
  behavior and explicit unknown/unsupported phase handling.
- Author/update canonical hydrology contract authority for phase-entry routing
  semantics and guard expectations.
- Author/update contract-derived conformance tests and run documented
  pre-implementation contract gate before production routing edits.
- Add compile/test evidence for production kernel wiring and typed phase
  routing.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- Full hydrology kernel algorithm implementations (`WB11`/`WB12`/`WB13`).
- Plant growth/decomposition transition implementations (`PL10..PL13`).
- Tier-A comparator closeout execution (`PL14`/`PL15`).

## Deliverables
1. Hydrology phase-kernel skeleton authority note:
   - `artifacts/wb10-phase-kernel-skeleton-authority.md`
2. Scheduler phase-class routing and guard map:
   - `artifacts/wb10-phase-routing-guard-map.md`
3. Canonical contract amendment plan/evidence:
   - `artifacts/wb10-sc-contract-amendment-plan.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/wb10-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/wb10-implementation-and-test-evidence.md`
6. Typed-seam non-regression evidence:
   - `artifacts/wb10-typed-seam-non-regression-evidence.md`
7. Kernel profile compliance checklist:
   - `artifacts/wb10-kernel-profile-compliance-checklist.md`
8. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/wb10_disposition.md`
9. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl09a-pre-execution-preconditions-clearance-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl09a-pre-execution-preconditions-clearance-001/artifacts/pl09a_disposition.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-wb10-hydrology-phase-kernel-skeleton-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL09/PL09A prerequisite closure and WB10 queue scope.

### Phase 1 - Contract Authority
- Ratify hydrology phase-entry scaffolding authority and guard expectations.

### Phase 2 - Contract-Test Gate (Pre-Implementation)
- Author/update contract-derived conformance tests from ratified authority.
- Execute and record pre-implementation gate results before routing code edits.

### Phase 3 - Implementation
- Implement production hydrology phase-kernel skeleton entry/routing path with
  typed error propagation.

### Phase 4 - Verification
- Run targeted routing/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract amendments, implementation evidence, and disposition.

## Exit Criteria
- Production hydrology phase-kernel skeleton path exists and is wired through
  scheduler phase-class dispatch (non-probe implementation path).
- Phase-class routing is typed and explicitly rejects unsupported/unknown phase
  categories.
- Pre-implementation contract-gate evidence exists and shows contract-derived
  tests authored/updated before routing code edits.
- Contract updates satisfy kernel profile requirements.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: scientific kernel scaffolding and contract/test changes.
