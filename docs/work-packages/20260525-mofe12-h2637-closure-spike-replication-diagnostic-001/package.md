# 20260525-mofe12-h2637-closure-spike-replication-diagnostic-001

## Status
- state: complete
- date: 2026-05-26
- timezone: UTC

## Objective
Assess whether openWEPP reproduces the documented WEPP-forest H2637 day-44
closure-spike defect signature from incident package:
`/home/workdir/wepp-forest/docs/ablation/20260430_uncapped-spectacular_h2637_hillslope_closure-spike/incident.md`.

## Why This Package Exists
The incident package demonstrates a binary-lineage closure anomaly for H2637 in
legacy WEPP outputs (`day44_hillslope_error_mm_legacy=-180.31779`, dominant
`OFE19=-180.4590`). openWEPP needs explicit diagnostic evidence on whether the
same signature appears on openWEPP candidate outputs under equivalent input
forcing.

## Scope
### Included
- Reconstruct authoritative legacy defect metric from incident artifacts.
- Build openWEPP TOML runfile for staged H2637 inputs.
- Execute openWEPP candidate run for H2637 with compat policy.
- Compute incident-aligned day-44 diagnostic metrics on candidate output.
- Classify replication posture (`replicated` / `not-replicated` /
  `indeterminate`) with explicit constraints.

### Explicitly Out of Scope
- Production kernel physics/code edits.
- Comparator schema/policy redesign.
- Non-H2637 hillslope generalization.

## Deliverables
1. Defect replication diagnostic report:
   - `artifacts/mofe12-h2637-defect-replication-diagnostic-report.md`
2. Contract implementation evidence:
   - `artifacts/mofe12-contract-implementation-evidence.md`
3. Contract-test implementation evidence:
   - `artifacts/mofe12-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate:
   - `artifacts/mofe12-preimplementation-contract-gate.md`
5. Implementation/test evidence:
   - `artifacts/mofe12-implementation-and-test-evidence.md`
6. Kernel profile checklist:
   - `artifacts/mofe12-kernel-profile-compliance-checklist.md`
7. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/mofe12_disposition.md`
   - `artifacts/worker-handoff.md`
8. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
9. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end without user
intervention unless hard-blocked by missing executable prerequisites.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Contract-First Sequence (Required)
1. Confirm existing canonical contract authority is sufficient for a
   diagnostics-only package.
2. Confirm contract-derived tests are not required when no production code is
   edited.
3. Record pre-implementation contract gate evidence.
4. Execute diagnostic runtime + analysis commands and record disposition.

No production kernel behavior edits are permitted in this package.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe11-oratea-zero-domain-compatibility-and-h324-parity-rerun-001/artifacts/mofe11_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe11-oratea-zero-domain-compatibility-and-h324-parity-rerun-001/artifacts/worker-handoff.md`
- `/home/workdir/wepp-forest/docs/ablation/20260430_uncapped-spectacular_h2637_hillslope_closure-spike/incident.md`
- `/home/workdir/wepp-forest/docs/ablation/20260430_uncapped-spectacular_h2637_hillslope_closure-spike/artifacts/repro/staged/runs/`
- `/home/workdir/wepp-forest/docs/ablation/20260430_uncapped-spectacular_h2637_hillslope_closure-spike/artifacts/logs/lane_day44_legacy_closure.csv`

## Intended Write Set
- `docs/work-packages/20260525-mofe12-h2637-closure-spike-replication-diagnostic-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Diagnostic Authority and Formula Recovery
- Reconstruct the incident day-44 legacy residual formula from incident data.
- Record source references and validation checks.

### Phase B - Contract/Test Gate Documentation (No-Code Path)
- Record that canonical contracts are unchanged for this diagnostics-only lane.
- Record no new contract-derived production tests are required.

### Phase C - Candidate Execution
- Build TOML runfile for staged H2637 files.
- Execute `openwepp-cli-hill` with compat policy.

### Phase D - Defect Replication Assessment
- Compute incident-aligned day-44 metric(s) from candidate output.
- Compare against incident signature and classify replication status.

### Phase E - Closeout
- Complete evidence artifacts, gate matrix, reviews, verification, and
  disposition.

## Exit Criteria
- OpenWEPP H2637 candidate run executes or yields typed blocker evidence.
- Incident-aligned day-44 diagnostic metrics are computed from candidate output
  (or explicitly marked indeterminate with cause).
- Disposition explicitly answers whether openWEPP replicates the defect
  signature under available publication geometry.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: diagnostics-only package; no secrets/network boundary changes.
