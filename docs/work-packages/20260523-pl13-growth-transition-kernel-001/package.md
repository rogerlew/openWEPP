# 20260523-pl13-growth-transition-kernel-001

## Status
- state: complete
- date: 2026-05-23
- timezone: UTC

## Objective
Implement production annual/perennial growth transition execution with
senescence/harvest transition signaling in the monolithic openWEPP scientific
hydrology/erosion model.

## Why This Package Exists
PL09 identified `PL09-GAP-006` as a Tier-A blocker: production growth/decomp/
resup kinetics were not implemented. PL12 closes decomposition/residue
transition execution for that blocker lane; PL13 closes the growth-transition
lane required for coupled daily execution and downstream WB11/INT10 packages.

PL13 remains contract-first: algorithm authority and contract-derived gate tests
are mandatory before growth-kernel code edits are considered complete.

## Scope
### Included
- Implement production annual/perennial growth transition execution in growth
  phases with day-window logic and transition signaling.
- Enforce state update/invariant behavior for key growth surfaces, including
  `sumgdd`, `vdmt`, `cancov`, `lai`, `rtmass`, `rtd`, and `hia`.
- Enforce typed guard behavior for invalid domains and impossible
  transition/update states.
- Author/update canonical contract authority for growth transition algorithm
  details and guard expectations.
- Author/update contract-derived conformance tests and run documented
  pre-implementation contract gate before production kernel code edits.
- Add targeted kernel transition tests and integration coverage for annual and
  perennial branches.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Parallel Execution Constraints (with PL13A)
- `PL13` and `PL13A` are authorized for parallel execution.
- `PL13` owns runtime growth-kernel implementation and growth transition tests.
- `PL13` must not edit alias continuity authority surfaces owned by `PL13A`:
  - `docs/specifications/science-contracts/symbol-alias-registry.md`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` alias-map
    sections
  - `crates/openwepp-sim-contract/src/symbols.rs`

### Explicitly Out of Scope
- Alias continuity closeout authority (`PL13A`).
- Tier-A comparator closeout execution (`PL14`/`PL15`).
- New water-balance kernel implementation (`WB10+`).

## Deliverables
1. Growth transition process-contract authority statement:
   - `artifacts/pl13-process-contract-authority.md`
2. Growth transition algorithm and guard map note:
   - `artifacts/pl13-growth-kernel-algorithm.md`
3. Canonical contract amendment plan/evidence:
   - `artifacts/pl13-sc-contract-amendment-plan.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/pl13-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/pl13-implementation-and-test-evidence.md`
6. Typed-seam non-regression evidence:
   - `artifacts/pl13-typed-seam-non-regression-evidence.md`
7. Kernel profile compliance checklist:
   - `artifacts/pl13-kernel-profile-compliance-checklist.md`
8. Parallel ownership boundary confirmation:
   - `artifacts/pl13-parallel-ownership-boundary.md`
9. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl13_disposition.md`
10. Dual review/verification artifacts:
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl12-decomp-resup-transition-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl12-decomp-resup-transition-kernel-001/artifacts/pl12_disposition.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-pl13-growth-transition-kernel-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL12 completion state and PL13 queue scope.

### Phase 1 - Contract Authority
- Ratify growth transition algorithm authority and guard expectations in
  canonical contracts.

### Phase 2 - Contract-Test Gate (Pre-Implementation)
- Author/update contract-derived conformance tests from ratified authority.
- Execute and record pre-implementation gate results before kernel code edits.

### Phase 3 - Implementation
- Implement production growth transition execution in growth phases with typed
  error propagation.

### Phase 4 - Verification
- Run targeted kernel/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract amendments, implementation evidence, and closure evidence.

## Exit Criteria
- PL13 closure contribution to `PL09-GAP-006` (growth lane) is evidence-backed.
- Production annual/perennial growth transition path exists (no placeholder/no-op
  execution for covered transition semantics).
- Growth state updates and transition signaling enforce typed guard behavior.
- Pre-implementation contract-gate evidence exists and shows contract-derived
  tests authored/updated before kernel code edits.
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
- Rationale: scientific growth-kernel and contract/test changes.
