# 20260531-hphys0216c-profilefc-normalized-tail-delta-analysis-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0216 follow-up diagnostics to explain the `ProfileFCStore`
regression (`27/39 -> 39/39`) by isolating symbol-path deltas between
layer-aggregated FC publication and normalized-profile storage seed lineage,
then publish a concrete remediation package handoff.

## Why This Package Exists
HPHYS0216 landed the FC publication-authority realignment to layer aggregation
and preserved typed guards, but semantic rerun results regressed across all 39
hillslopes. Worker handoff from HPHYS0216 requires immediate regression
analysis before advancing queued `HPHYS0217` closure claims.

## Scope
### Included
- Analyze HPHYS0216 semantic outputs on `unpalatable-rind` 39-hillslope cohort.
- Quantify per-hillslope FC deltas and characterize shift patterns.
- Trace code lineage between:
  - WB13 FC publication (`crates/openwepp-runner/src/hillslope/mod.rs`)
  - profile seed projection + layer mapping
    (`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`).
- Publish root-cause hypothesis and remediation handoff with explicit write set
  and test obligations.

### Explicitly Out of Scope
- Production kernel/runtime code edits.
- Canonical contract amendments.
- Re-running full workspace gates (no code changes in this package).

## Closure Measures (Required)
1. `MEASURE-HP216C-001`: reproducible per-hillslope `ProfileFCStore` delta
   matrix is published from current HPHYS0216 cohort outputs.
2. `MEASURE-HP216C-002`: source-level symbol-path diagnosis explains the
   constant-offset regression mechanism.
3. `MEASURE-HP216C-003`: follow-up remediation package proposal is explicit
   about contract/test/code write set and closure criteria.
4. `MEASURE-HP216C-004`: work-package registry is updated and handoff is ready
   for immediate execution.

## Deliverables
1. `artifacts/hphys0216c-residual-gap-matrix.md`
2. `artifacts/hphys0216c-contract-implementation-evidence.md`
3. `artifacts/hphys0216c-contract-test-implementation-evidence.md`
4. `artifacts/hphys0216c-preimplementation-contract-gate.md`
5. `artifacts/hphys0216c-implementation-and-test-evidence.md`
6. `artifacts/hphys0216c-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0216c_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Sequencing (Required)
1. Analyze evidence outputs and source lineage first.
2. Record no-code preimplementation gate.
3. Publish residual diagnostics and remediation handoff.
4. Update queue/registry entries.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority remains in `docs/specifications/science-contracts/contracts/SC-*.md`.
- Baseline comparator provenance:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- This package does not modify physics authority; it diagnoses regression
  behavior to prepare authoritative remediation.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0216-profilefc-layer-authority-realignment-001/artifacts/worker-handoff.md`
- `/tmp/hphys0216_20260531T053959Z/parity/reports/semantic/`
- `/tmp/hphys0216_20260531T053959Z/parity/reports/hillslope_semantic_summary.tsv`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0216c-profilefc-normalized-tail-delta-analysis-001/**`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm follow-up authorization from HPHYS0216 handoff.

### Phase B - Regression diagnostics
- Compute per-hillslope FC deltas and cluster patterns from cohort outputs.

### Phase C - Source-path diagnosis
- Link observed deltas to specific symbol lineage and mapping behavior in
  runtime-input and WB13 publication code paths.

### Phase D - Disposition and follow-up queue
- Publish `HOLD`/`GO` decision and explicit remediation package handoff.

## Exit Criteria
- Closure measures `MEASURE-HP216C-001..004` are satisfied and evidenced.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: diagnostics/documentation-only package.
