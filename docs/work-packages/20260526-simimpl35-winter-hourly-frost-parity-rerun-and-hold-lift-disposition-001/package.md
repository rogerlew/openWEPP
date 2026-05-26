# 20260526-simimpl35-winter-hourly-frost-parity-rerun-and-hold-lift-disposition-001

## Status
- state: package-complete-with-hold
- date: 2026-05-26
- timezone: UTC
- decision: HOLD

## Objective
Execute SIMIMPL35 by rerunning winter-hourly frost parity lanes after
SIMIMPL34 solver migration and publishing an explicit GO/HOLD hold-lift
disposition with residual ownership.

## Why This Package Exists
SIMIMPL34 completed baseline-authoritative frost solver migration and retained
`HOLD` pending SIMIMPL35 rerun/disposition closure. This package is the
queue-authorized final frost hold-lift gate in FROSTPLAN01.

## Scope
### Included
- Confirm queue/disposition authorization from FROSTPLAN01 and
  SIMIMPL31/32/33/34 sequence completion.
- Execute admissible winter-hourly replay/comparator lanes with post-SIMIMPL34
  candidate evidence.
- Classify residuals and produce explicit GO/HOLD decision evidence.
- Complete required governance artifacts, dual review, dual verification,
  gate results, and disposition.

### Explicitly Out of Scope
- New production kernel physics implementation.
- New canonical `SC-*` authority amendments unless rerun evidence proves
  direct contradiction.

## Deliverables
1. Winter-hourly parity evidence report:
   - `artifacts/simimpl35-winter-hourly-semantic-parity-evidence-report.md`
2. Hold-lift decision report:
   - `artifacts/simimpl35-hold-lift-decision-report.md`
3. Contract implementation evidence:
   - `artifacts/simimpl35-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/simimpl35-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/simimpl35-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/simimpl35-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/simimpl35-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl35_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
Kernel-affecting packages preserve contract-first sequencing when corrective
implementation is required:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

SIMIMPL35 executed as rerun/disposition evidence scope with no kernel edits.

## Autonomous Execution Intent (Required)
This package executed end-to-end without user intervention, including rerun
artifacts, gate execution, governance artifacts, and disposition.

## Truthfulness Labeling Requirement
All evidence artifacts include explicit `Static:` and/or `Ran:` sections.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/artifacts/frost-energy-solver-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl31-frost-energy-contract-authority-and-routine-map-001/artifacts/simimpl31_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl32-frost-hourly-contract-derived-tests-and-preimplementation-gate-001/artifacts/simimpl32_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl33-frost-energy-runtime-state-topology-and-seam-closure-001/artifacts/simimpl33_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260526-simimpl34-frost-energy-solver-kernel-migration-and-coupling-001/artifacts/simimpl34_disposition.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/README.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/frostn.for`
- `/workdir/wepp-forest_260430_baseline/src/frsoil.for`
- `/workdir/wepp-forest_260430_baseline/src/frwatc.for`
- `/workdir/wepp-forest_260430_baseline/src/frzng.for`
- `/workdir/wepp-forest_260430_baseline/src/frznw.for`
- `/workdir/wepp-forest_260430_baseline/src/getfreezecond.for`

## Intended Write Set
- `docs/work-packages/20260526-simimpl35-winter-hourly-frost-parity-rerun-and-hold-lift-disposition-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and Prerequisite Confirmation
- Confirmed SIMIMPL35 authorization and dependency completion.

### Phase B - Winter-Hourly Rerun Execution
- Executed replay/comparator lanes and captured both blocking and admissible
  diagnostics.

### Phase C - Required Gates
- Ran required package gates and captured outputs.

### Phase D - Governance and Hold-Lift Decision
- Completed artifacts, dual review/verification, and explicit HOLD decision.

### Phase E - Disposition
- Published final SIMIMPL35 disposition with residual ownership.

## Exit Criteria
- At least one admissible lane demonstrates non-zero common-key overlap and
  evidence for frost process-family closure, or decision remains `HOLD` with
  explicit blockers.
- Required package gates are executed and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Governance artifacts are complete with truthful labels.

## Execution outcome summary
- Admissible comparator lanes were produced by filtering `/wc1` parquet
  candidate to `wepp_id=5` (`common_row_count=1095`, semantic pass for both
  native-parquet and conversion-derived-dat lane classes).
- Post-SIMIMPL34 fresh candidate generation reruns failed with typed runtime
  domain violation (`KWRITEBACK-E-DOMAIN-VIOLATION`) on the shared fixture and
  with parser compatibility failure (`SOL-E-006`) on direct `/wc1` execution.
- Final decision: `HOLD` pending a fresh post-SIMIMPL34 candidate lane that
  reaches comparator execution without these blockers.

## Security Impact and Review Gate
- security_impact: medium
- dedicated_security_review_required: no
- Rationale: rerun/disposition package with hold-lift governance authority.
