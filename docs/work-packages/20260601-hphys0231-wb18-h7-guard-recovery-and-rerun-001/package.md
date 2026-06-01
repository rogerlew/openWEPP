# 20260601-hphys0231-wb18-h7-guard-recovery-and-rerun-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Carry out immediate next actions from HPHYS0230 by (1) triaging WB18
`HKERNEL-WB11-PERC-E-003` on `H7` with concrete symbol/value diagnostics,
(2) reconciling WB18 `FC/UL -> Bi` guard placement to baseline-authoritative
branch semantics, and (3) rerunning `unpalatable-rind` (`H1..H39`) for updated
readjudication evidence.

## Why This Package Exists
HPHYS0230 completed the dynamic-`Bi` authority migration but left two explicit
blockers:
1. `H7` runtime hard-fails WB18 percolation guard (`HKERNEL-WB11-PERC-E-003`),
   preventing `39/39` semantic artifacts.
2. H1 early-day `Dp` overdrainage remains materially above baseline.

The worker handoff requires an immediate follow-on package to diagnose the `H7`
guard failure, restore full cohort execution, and refresh residual evidence.

## Scope
### Included
- Add WB18 failure diagnostics (symbol/value capture) for `HKERNEL-WB11-PERC-E-003`
  in hillslope execution-provenance error paths.
- Amend canonical WB18 contract authority in `SC-PERC-001` to make guard
  placement explicit for dynamic-`Bi` branch activation.
- Update WB18 contract-derived tests to exercise:
  - guard hard-fail when dynamic-`Bi` branch is active and ratio is invalid,
  - no false hard-fail when saturated branch (`stz >= 0.95`) bypasses dynamic-`Bi`.
- Implement production WB18 guard-placement correction as required by contract.
- Execute full `H1..H39` rerun + semantic comparator and publish updated
  readjudication deltas.
- Execute required workspace gates and publish disposition/handoff.

### Explicitly Out of Scope
- Full WB18 physics closure for H1 transient `Dp` mismatch.
- WB19 or watershed-routing kernel changes.
- Parser-format changes not required for WB18 guard-placement closure.

## Closure Measures (Required)
1. `MEASURE-HP231-001`: H7 guard failure diagnostic artifact captures concrete
   symbol/value context for the failing WB18 state.
2. `MEASURE-HP231-002`: canonical `SC-PERC-001` authority explicitly defines
   dynamic-`Bi` guard placement relative to WB18 saturation branch selection.
3. `MEASURE-HP231-003`: contract-derived WB18 tests are updated/added and pass.
4. `MEASURE-HP231-004`: production WB18 implementation matches amended guard
   placement and preserves typed fail-closed behavior where applicable.
5. `MEASURE-HP231-005`: post-change `H1..H39` run + semantic artifacts are
   produced with `39/39` successful comparator reports.
6. `MEASURE-HP231-006`: required gates pass (`fmt`,`clippy`,`test`,`deny`) and
   disposition/handoff are published.

## Deliverables
1. `artifacts/hphys0231-h7-guard-diagnostic-capture.md`
2. `artifacts/hphys0231-residual-authority-gap-matrix.md`
3. `artifacts/hphys0231-contract-implementation-evidence.md`
4. `artifacts/hphys0231-contract-test-implementation-evidence.md`
5. `artifacts/hphys0231-preimplementation-contract-gate.md`
6. `artifacts/hphys0231-implementation-and-test-evidence.md`
7. `artifacts/hphys0231-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/hphys0231_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/perc.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0230-wb18-overdrainage-authority-closure-001/artifacts/hphys0230_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0230-wb18-overdrainage-authority-closure-001/artifacts/worker-handoff.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0231-wb18-h7-guard-recovery-and-rerun-001/**`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`

## Phase Plan
### Phase A - Intake and scaffolding
- Confirm immediate-next-action scope from HPHYS0230 handoff.
- Prepare queued artifacts and kickoff prompt.

### Phase B - Contract-first updates
- Amend `SC-PERC-001` guard-placement authority for dynamic-`Bi` branching.
- Update/add contract-derived tests.
- Record pre-implementation contract gate evidence.

### Phase C - Implementation + diagnostics
- Implement WB18 guard-placement correction and execution-provenance diagnostics.
- Reproduce/triage H7 with symbol/value capture.

### Phase D - Rerun + adjudication
- Execute `H1..H39` rerun and semantic comparator.
- Publish residual delta matrix and H1 transient trace.
- Run workspace gates and publish disposition/handoff.

## Exit Criteria
- `MEASURE-HP231-001..006` satisfied and evidenced.
- Stream-level HOLD/GO decision is explicit in disposition.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/runner/docs/test changes only; no credentials/network.
