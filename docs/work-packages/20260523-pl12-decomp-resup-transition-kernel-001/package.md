# 20260523-pl12-decomp-resup-transition-kernel-001

## Status
- state: complete
- date: 2026-05-23
- timezone: UTC

## Objective
Implement production decomposition/residue transition kernel execution against
typed contexts and projected event controls in the monolithic openWEPP
scientific hydrology/erosion model.

## Why This Package Exists
PL09 identified `PL09-GAP-006` as a Tier-A blocker: production growth/decomp/
resup kinetics were not implemented in production paths. PL11 closed runtime
projection blockers (`PL09-GAP-004`/`PL09-GAP-005`) and provided transition-
control payloads required for decomposition/residue execution.

PL12 is the next plant-lane execution package and must remain contract-first:
algorithm authority and contract-derived gate tests are required before kernel
code edits are considered complete.

## Scope
### Included
- Implement production decomposition/residue transition execution in scheduler
  phase flow using typed decomposition contexts.
- Consume projected annual/perennial transition-control payload families from
  PL11 surfaces (`resmgt`, `ncut`, `ncycle`, `cutday_*`, `gday_*`, `gend_*`,
  grazing payload fields).
- Enforce typed guard behavior for invalid domains/cardinality/range and
  impossible transfer/removal states.
- Author/update canonical contract authority for decomposition/residue
  transition algorithm details and guard expectations.
- Author/update contract-derived conformance tests and run documented
  pre-implementation contract gate prior to production kernel code edits.
- Add targeted kernel tests, invariants, and residue trajectory checks.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- Production annual/perennial growth transition execution (`PL13`).
- Tier-A comparator closeout execution (`PL14`/`PL15`).
- Water-balance kernel implementation packages (`WB10+`).

## Deliverables
1. Decomposition/residue process-contract authority statement:
   - `artifacts/pl12-process-contract-authority.md`
2. Kernel algorithm and guard map note:
   - `artifacts/pl12-decomp-resup-kernel-algorithm.md`
3. Canonical contract amendment plan/evidence:
   - `artifacts/pl12-sc-contract-amendment-plan.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/pl12-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/pl12-implementation-and-test-evidence.md`
6. Typed-seam non-regression evidence:
   - `artifacts/pl12-typed-seam-non-regression-evidence.md`
7. Kernel profile compliance checklist:
   - `artifacts/pl12-kernel-profile-compliance-checklist.md`
8. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl12_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/openwepp-vs-baseline-pl-parity-gap-register.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl10-active-slot-authority-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl10b-contract-blind-authority-and-conformance-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl11-pl-event-runtime-projection-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl11-pl-event-runtime-projection-001/artifacts/pl11_disposition.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-pl12-decomp-resup-transition-kernel-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL11 completion state and PL12 queue scope.

### Phase 1 - Contract Authority
- Ratify decomposition/residue transition algorithm authority and guard
  expectations in canonical contracts.

### Phase 2 - Contract-Test Gate (Pre-Implementation)
- Author/update contract-derived conformance tests from ratified authority.
- Execute and record pre-implementation gate results before kernel code edits.

### Phase 3 - Implementation
- Implement production decomposition/residue transition execution in
  decomposition phases with typed error propagation.

### Phase 4 - Verification
- Run targeted kernel and integration tests plus required repository gates.

### Phase 5 - Disposition
- Publish contract amendments, implementation evidence, and reconciliation
  outcomes.

## Exit Criteria
- PL12 closure contribution to `PL09-GAP-006` (decomposition/resup lane) is
  evidence-backed.
- Production decomposition/residue transition path exists (no placeholder/no-op
  execution for covered transition semantics).
- Transition/residue pool updates enforce typed guard behavior and invariant
  checks.
- Pre-implementation contract-gate evidence exists and shows contract-derived
  tests authored/updated before kernel code edits.
- `SC-PLANT-001` / `SC-RESIDUE-001` updates satisfy kernel profile requirements.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: scientific kernel and contract/test changes only.
