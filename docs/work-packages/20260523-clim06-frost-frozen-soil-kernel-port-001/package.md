# 20260523-clim06-frost-frozen-soil-kernel-port-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Implement frozen-soil/frost runtime kernel behavior and infiltration coupling
in the monolithic openWEPP scientific hydrology/erosion model.

## Why This Package Exists
PL15 retained PL08 hold and dispositioned `KERNEL-GAP-006` to `CLIM06` in the
PL09 queue addendum. CLIM05 closes snow runtime coupling and WB14 closes core
infiltration/hyetograph coupling preconditions; CLIM06 closes cold-season
frozen-soil branch behavior so infiltration/runoff partition responds to
runtime frost state under typed failure semantics.

This package is contract-first and implementation-bound: canonical contract
amendments and contract-derived tests must be implemented and pre-implementation
gate evidence recorded before production kernel code edits.

## Scope
### Included
- Implement runtime frozen-soil/frost kernel behavior from parsed frost control
  surfaces and winter state inputs.
- Couple frozen-soil state outputs into infiltration/runoff branch behavior in
  hydrology execution.
- Enforce typed guard behavior for invalid/non-finite/out-of-domain frost
  states, frozen-layer depth, and coupled infiltration-capacity terms.
- Implement canonical contract amendments for CLIM06 frost-runtime authority
  and guard behavior.
- Implement contract-derived CLIM06 tests from amended contract authority and
  run pre-implementation contract-gate evidence before production kernel code
  edits.
- Produce cold-season fixture replay evidence and infiltration-branch coupling
  evidence for frozen-soil scenarios.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- CLIM05 snow accumulation/melt runtime coupling (`KERNEL-GAP-005`).
- PL16/PL17 plant-physics kernelization (`KERNEL-GAP-002..003`).
- WB15 canopy interception and WB16 peak-runoff follow-ons.
- Tier-A hold-lift closeout disposition updates beyond CLIM06 scope.

## Deliverables
1. CLIM06 process-contract authority implementation evidence:
   - `artifacts/clim06-contract-implementation-evidence.md`
2. CLIM06 frost-runtime authority and guard map note:
   - `artifacts/clim06-frost-runtime-authority-and-guard-map.md`
3. CLIM06 contract-derived test implementation evidence:
   - `artifacts/clim06-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/clim06-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/clim06-implementation-and-test-evidence.md`
6. Cold-season fixture replay evidence:
   - `artifacts/clim06-cold-season-fixture-replay-evidence.md`
7. Infiltration/runoff branch coupling evidence:
   - `artifacts/clim06-infiltration-runoff-branch-coupling-evidence.md`
8. Typed-seam non-regression evidence:
   - `artifacts/clim06-typed-seam-non-regression-evidence.md`
9. Kernel profile compliance checklist:
   - `artifacts/clim06-kernel-profile-compliance-checklist.md`
10. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim06_disposition.md`
11. Dual review/verification artifacts:
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/claude-pl15-pre-closeout-physics-review.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim05-snow-runtime-kernel-port-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim05-snow-runtime-kernel-port-001/artifacts/clim05_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/artifacts/wb14_disposition.md`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/frost.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `tests/fixtures/infile/frost/**`
- `docs/work-packages/20260523-clim06-frost-frozen-soil-kernel-port-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL15 queue/addendum scope, CLIM05 closure baseline, and frost-control
  parser/runtime seam readiness.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for CLIM06 frost-runtime
  authority and invariants before runtime kernel changes.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests from amended contract authority.
- Execute and record pre-implementation contract-gate evidence before
  production kernel code edits.

### Phase 3 - Kernel Implementation
- Implement frozen-soil/frost runtime coupling behavior with typed guards and
  deterministic infiltration/runoff branch sequencing.

### Phase 4 - Verification
- Run targeted kernel/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract + contract-test implementation evidence, cold-season replay
  evidence, and CLIM06 closure posture.

## Exit Criteria
- `KERNEL-GAP-006` CLIM06 closure is evidence-backed.
- Frozen-soil state surfaces drive infiltration/runoff branch behavior in
  runtime execution.
- Frost/frozen-soil effects on infiltration-capacity and coupled hydrology
  terms are implemented under typed invariants.
- Canonical CLIM06-relevant contracts are implemented in SC files (not just
  proposed).
- Contract-derived CLIM06 tests are implemented and executed (not just
  planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production kernel code edits.
- Cold-season fixture replay evidence demonstrates coupled branch behavior.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: scientific frost-runtime kernel and contract/test implementation.
