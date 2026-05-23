# 20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Implement production infiltration kernel authority and within-day hyetograph
integration in the monolithic openWEPP scientific hydrology/erosion model,
replacing externally-seeded infiltration bookkeeping posture.

## Why This Package Exists
PL15 retained the PL08 hold and dispositioned `KERNEL-GAP-001` and
`KERNEL-GAP-004` as critical follow-on work. The PL09 hold-lift queue addendum
maps those gaps to `WB14` as the first required hydrology physics closure
before downstream climate/irrigation/peak-flow packages.

This package is contract-first and implementation-bound: canonical contract
amendments and contract-derived tests must be implemented and pre-implementation
gate evidence recorded before production kernel code edits.

## Scope
### Included
- Implement production infiltration kernel behavior (Green-Ampt lineage
  authority) in openWEPP runtime execution.
- Implement within-day hyetograph consumption/integration in hydrology
  execution loops for infiltration/runoff coupling.
- Remove dependency on externally-seeded infiltration bookkeeping for WB14
  acceptance paths.
- Implement canonical contract amendments for WB14 authority and guard behavior
  in science-contract files.
- Implement contract-derived WB14 tests from amended contract authority and run
  pre-implementation contract-gate evidence before production kernel code edits.
- Preserve typed failure propagation (no silent defaults/clamping for missing,
  non-finite, or out-of-domain infiltration/hyetograph inputs).
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- PL16 growth-physics kernelization (`KERNEL-GAP-002`).
- PL17 decomposition-physics kernelization (`KERNEL-GAP-003`).
- CLIM05/CLIM06 snow/frost runtime kernel ports (`KERNEL-GAP-005..006`).
- IRRIG10 irrigation runtime kernel port (`KERNEL-GAP-008`).
- WB16 peak runoff kernel (`KERNEL-GAP-009`) and erosion/watershed follow-ons.

## Deliverables
1. WB14 process-contract authority implementation evidence:
   - `artifacts/wb14-contract-implementation-evidence.md`
2. WB14 infiltration/hyetograph algorithm and guard map note:
   - `artifacts/wb14-infiltration-and-hyetograph-kernel-authority-and-guard-map.md`
3. WB14 contract-derived test implementation evidence:
   - `artifacts/wb14-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/wb14-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/wb14-implementation-and-test-evidence.md`
6. Replay provenance and infiltration lineage evidence:
   - `artifacts/wb14-replay-provenance-and-infiltration-lineage.md`
7. Typed-seam non-regression evidence:
   - `artifacts/wb14-typed-seam-non-regression-evidence.md`
8. Kernel profile compliance checklist:
   - `artifacts/wb14-kernel-profile-compliance-checklist.md`
9. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/wb14_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/claude-pl15-pre-closeout-physics-review.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb13-daily-water-balance-output-surface-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb13-daily-water-balance-output-surface-001/artifacts/wb13_disposition.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/comparator_tier_routing_metadata.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL15 addendum scope and WB13 closure handoff assumptions.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for WB14 authority and
  invariants before runtime kernel changes.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests from amended contract authority.
- Execute and record pre-implementation contract-gate evidence before
  production kernel code edits.

### Phase 3 - Kernel Implementation
- Implement infiltration and within-day hyetograph kernel behavior under typed
  guards and deterministic runtime sequencing.

### Phase 4 - Verification
- Run targeted kernel/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract + contract-test implementation evidence, replay provenance,
  and WB14 closure posture.

## Exit Criteria
- WB14 infiltration is computed by openWEPP kernel path (not fixture-seeded
  bookkeeping).
- Within-day hyetograph forcing is consumed by runtime hydrology execution.
- Canonical WB14-relevant contracts are implemented in SC files (not just
  proposed).
- Contract-derived WB14 tests are implemented and executed (not just planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production kernel code edits.
- Replay provenance explicitly documents infiltration-source lineage and
  computed-kernel path evidence.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: scientific hydrology kernel and contract/test implementation.
