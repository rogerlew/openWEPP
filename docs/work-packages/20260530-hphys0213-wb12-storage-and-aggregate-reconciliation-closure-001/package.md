# 20260530-hphys0213-wb12-storage-and-aggregate-reconciliation-closure-001

## Status
- state: completed
- date: 2026-05-30
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0213 to close HPHYS0212 follow-on defects by:
1. remediating WB12 storage-reconciliation domain failures (`HKERNEL-WB12-STORAGE-E-003`)
   caused by non-physical WB19 subsurface-loss publication,
2. enforcing WB19 withdrawal/flux publication continuity such that emitted
   `q`, `Qdd`, and `Qd` are bounded by physically realizable layer withdrawals,
3. restoring WB11 aggregate soil-water continuity after WB19 mutations so
   `Total-Soil`/`SoilWaterTotal` lineage remains deterministic and non-stale.

## Why This Package Exists
HPHYS0212 completed WB11 seed lifecycle and WB19 runtime-source control
remediation but retained `HOLD` due:
- H5 execution failure in WB12 storage reconciliation (`HKERNEL-WB12-STORAGE-E-003`),
- residual saturation in `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`,
- unresolved aggregate continuity risk after WB19 layer withdrawals.

This package is the immediate closure lane before integrated hold-lift
adjudication in HPHYS0214.

## Scope
### Included
- Contract-authority intake for WB12 storage closure, WB19 withdrawal/flux
  continuity, and WB11 aggregate publication lineage.
- Contract-derived tests covering:
  - WB19 flux emission bounded by realized withdrawals,
  - WB12 storage reconciliation non-negative closure under WB19 outputs,
  - WB11 aggregate (`wb11_soil_water`) updates after WB19 mutation.
- Production code edits in hydrology kernel + runner surfaces required to close
  HPHYS0212 `HP212-GAP-001..003`.
- Required workspace gates and targeted tests with evidence.
- Unpalatable-rind 39-hillslope rerun + semantic summary deltas.

### Explicitly Out of Scope
- Watershed/channel/impoundment kernel work.
- Non-hydrology refactors outside WB11/WB12/WB19/WB13 lineage.
- Final integrated hold-lift adjudication package (`HPHYS0214`).

## Closure Measures (Required)
1. `MEASURE-HP213-001`: contract-first sequence evidence is complete
   (authority intake -> contract-derived tests -> pre-implementation gate ->
   production edits).
2. `MEASURE-HP213-002`: WB12 storage-reconciliation domain failure is closed
   for H5 (`HKERNEL-WB12-STORAGE-E-003` no longer produced in rerun lane).
3. `MEASURE-HP213-003`: WB19 published subsurface/lateral/drain fluxes are
   bounded by realized withdrawals (no synthetic over-withdraw publication).
4. `MEASURE-HP213-004`: WB11 aggregate soil-water lineage is updated after WB19
   withdrawals and remains deterministic for WB13 publication.
5. `MEASURE-HP213-005`: required workspace gates pass and 39-hillslope rerun
   evidence is published with truthful `HOLD`/`GO` decisioning.

## Deliverables
1. `artifacts/hphys0213-contract-implementation-evidence.md`
2. `artifacts/hphys0213-contract-test-implementation-evidence.md`
3. `artifacts/hphys0213-preimplementation-contract-gate.md`
4. `artifacts/hphys0213-implementation-and-test-evidence.md`
5. `artifacts/hphys0213-kernel-profile-compliance-checklist.md`
6. `artifacts/hphys0213-residual-gap-matrix.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0213_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Ingest canonical contract authority and HPHYS0212 residual lineage.
2. Implement/adjust contract-derived tests for HPHYS0213 closure measures.
3. Record pre-implementation contract gate evidence.
4. Apply production edits and run validation/rerun evidence.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting user
direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority is `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy comparator baseline anchor:
  `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Comparator outcomes remain diagnostic evidence; process-authoritative closure
  is the promotability gate.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0212-wb11-wb19-lifecycle-coupling-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0212-wb11-wb19-lifecycle-coupling-closure-001/artifacts/hphys0212-residual-gap-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0212-wb11-wb19-lifecycle-coupling-closure-001/artifacts/hphys0212_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0212-wb11-wb19-lifecycle-coupling-closure-001/artifacts/worker-handoff.md`
- `/tmp/hphys0212_20260530T221447Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0212_20260530T221447Z/parity/logs/h5.stderr.log`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260530-hphys0213-wb12-storage-and-aggregate-reconciliation-closure-001/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`

## Phase Plan
### Phase A - Authorization and scope freeze
- Confirm HPHYS0213 authorization from HPHYS0212 handoff.
- Freeze scope to WB12/WB19/WB11 aggregate continuity remediation.

### Phase B - Contract + test intake
- Capture authoritative WB12/WB19/WB11 obligations.
- Land/adjust contract-derived tests before production edits.

### Phase C - Production remediation
- Enforce WB19 realized-withdrawal bounded publication.
- Restore WB11 aggregate continuity after WB19 mutation.
- Close WB12 storage reconciliation domain failure in H5 lane.

### Phase D - Validation and rerun
- Run required workspace gates and targeted tests.
- Re-run unpalatable-rind 39-hillslope diagnostics and publish deltas.

### Phase E - Disposition and handoff
- Publish truthful `HOLD`/`GO` decision and immediate-next queue
  (`HPHYS0214` as required).

## Exit Criteria
- `MEASURE-HP213-001..005` are evidenced.
- Owned-file manifest is complete.
- Disposition and worker handoff identify any residual blockers explicitly.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: hydrology runtime/math + documentation updates only; no auth,
  network, or privilege-surface changes.
