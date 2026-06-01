# 20260601-hphys0232-wb18-hourly-lane-percolation-alignment-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Carry out immediate next actions from HPHYS0231 by reconciling WB18
percolation hourly-lane seepage attenuation authority from legacy
`watbal_hourly.for`/`purk.for` (`ui_LFtstp`) into production WB18 execution,
then rerunning `unpalatable-rind` (`H1..H39`) for updated `Dp` transient
readjudication evidence.

## Why This Package Exists
HPHYS0231 restored full cohort execution (`39/39`) and closed H7 guard
behavior, but H1 early-day `Dp`/`Total-Soil` transient mismatch remains the
stream-level HOLD reason. Immediate handoff guidance requires narrowing closure
hypothesis against legacy `perc.for`/`purk.for` lane details before the next
rerun.

## Scope
### Included
- Amend canonical percolation contract authority to encode hourly-lane seepage
  attenuation lineage (`ui_LFtstp`) with explicit branch semantics.
- Add/adjust WB18 contract-derived tests for lane attenuation behavior and
  typed guard posture.
- Implement runtime-to-kernel lane attenuation wiring for WB18 percolation.
- Rerun `H1..H39` and semantic comparison; publish updated residual matrix and
  H1 day-1..7 transient trace.
- Execute workspace gates and publish disposition/handoff.

### Explicitly Out of Scope
- Full `watbal_hourly` multi-hour infiltration/ET loop migration.
- WB19 or watershed-routing process-physics changes.
- Non-WB18 parser-format changes not required for lane attenuation wiring.

## Closure Measures (Required)
1. `MEASURE-HP232-001`: canonical contract text explicitly documents WB18
   hourly-lane seepage attenuation lineage and guard posture.
2. `MEASURE-HP232-002`: contract-derived tests cover hourly attenuation
   behavior and pass.
3. `MEASURE-HP232-003`: production implementation publishes/consumes explicit
   WB18 lane attenuation control with typed domain guards.
4. `MEASURE-HP232-004`: `H1..H39` rerun and semantic reports are regenerated
   (`39/39` execution + comparator coverage).
5. `MEASURE-HP232-005`: H1 day-1..7 `Dp` transient readjudication is published
   against HPHYS0231 baseline trace with explicit HOLD/GO decision rationale.
6. `MEASURE-HP232-006`: required gates pass (`fmt`,`clippy`,`test`,`deny`) and
   disposition/handoff are published.

## Deliverables
1. `artifacts/hphys0232-h1-transient-lane-diagnostic.md`
2. `artifacts/hphys0232-residual-authority-gap-matrix.md`
3. `artifacts/hphys0232-contract-implementation-evidence.md`
4. `artifacts/hphys0232-contract-test-implementation-evidence.md`
5. `artifacts/hphys0232-preimplementation-contract-gate.md`
6. `artifacts/hphys0232-implementation-and-test-evidence.md`
7. `artifacts/hphys0232-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/hphys0232_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Sequence (Required)
1. Canonical contract amendments (`SC-*` authority update).
2. Contract-derived tests.
3. Pre-implementation contract gate evidence.
4. Production code implementation.
5. Validation gates + rerun/readjudication evidence + disposition.

## Autonomous Execution Intent (Required)
Execute phases end-to-end through disposition without requesting additional
user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/perc.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0231-wb18-h7-guard-recovery-and-rerun-001/artifacts/hphys0231_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0231-wb18-h7-guard-recovery-and-rerun-001/artifacts/worker-handoff.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0232-wb18-hourly-lane-percolation-alignment-001/**`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`

## Phase Plan
### Phase A - Intake and scaffolding
- Confirm immediate-next-action scope from HPHYS0231 handoff.
- Prepare queued artifacts and kickoff prompt.

### Phase B - Contract-first updates
- Amend `SC-PERC-001` for hourly-lane seepage attenuation authority.
- Update/add contract-derived tests.
- Record pre-implementation contract gate evidence.

### Phase C - Implementation
- Seed explicit lane attenuation control at runner WB11 intake.
- Consume lane attenuation control in WB18 percolation kernel with typed
  domain guards.

### Phase D - Rerun + adjudication
- Execute `H1..H39` rerun and semantic comparator.
- Publish residual delta matrix and H1 day-1..7 transient comparison.
- Run workspace gates and publish disposition/handoff.

## Exit Criteria
- `MEASURE-HP232-001..006` satisfied and evidenced.
- Stream-level HOLD/GO decision is explicit in disposition.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/runner/docs/test changes only; no credentials/network.
