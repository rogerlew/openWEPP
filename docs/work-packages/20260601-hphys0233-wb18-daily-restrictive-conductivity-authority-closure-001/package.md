# 20260601-hphys0233-wb18-daily-restrictive-conductivity-authority-closure-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Carry out immediate next actions from HPHYS0232 by reconciling WB18 daily-lane
percolation authority with baseline `perc.for`/`purk.for` restrictive-layer
conductivity handling (`slflag`/`kslast`) and by hardening WB13 `Dp` publication
lineage against stale state shadowing, then rerunning `unpalatable-rind`
(`H1..H39`) for readjudication.

## Why This Package Exists
HPHYS0232 closed hourly-lane attenuation lineage but produced no movement in
daily-lane HOLD residuals. H1 retains severe early-transient overdrainage
(`Dp` burst; `Total-Soil` collapse). Static baseline audit identifies a
remaining daily-lane authority gap: bottom-layer seepage conductivity reduction
through restrictive-layer branch (`slflag=1` with `kslast`) is parsed in
producer contracts but not consumed in WB18 runtime execution.

## Scope
### Included
- Amend canonical percolation contract authority for daily-lane restrictive
  conductivity branch and WB13 D/Pe publication lineage posture.
- Add/adjust contract-derived tests for:
  - daily bottom-layer restrictive conductivity branch semantics,
  - typed guards for restrictive conductivity inputs,
  - WB13 `D` publication anti-shadow behavior (flux-preferred lineage).
- Implement runtime projection + WB18 kernel changes required for branch
  authority (`slflag`, `kslast`, unit-consistent effective conductivity).
- Implement WB13 publication lineage hardening for flux-owned symbols.
- Rerun `H1..H39` and semantic comparison; publish residual matrix and H1
  day-1..7 transient trace.
- Execute workspace gates and publish disposition/handoff.

### Explicitly Out of Scope
- Full `frsoil` per-layer frozen-conductivity (`sscv`) migration.
- Full hourly bottom-boundary weighted harmonic branch (`ui_bdrkth`) migration.
- WB19 or watershed-routing process-physics changes.

## Closure Measures (Required)
1. `MEASURE-HP233-001`: canonical contract text documents WB18 daily
   restrictive conductivity branch authority and WB13 D/Pe publication lineage.
2. `MEASURE-HP233-002`: contract-derived tests for restrictive conductivity and
   publication anti-shadow pass.
3. `MEASURE-HP233-003`: production implementation projects and consumes
   restrictive conductivity symbols with typed domain guards.
4. `MEASURE-HP233-004`: `H1..H39` rerun and semantic reports regenerated
   (`39/39` execution + comparator coverage).
5. `MEASURE-HP233-005`: H1 day-1..7 `Dp` transient readjudication is published
   against HPHYS0232 baseline with explicit HOLD/GO rationale.
6. `MEASURE-HP233-006`: required gates pass (`fmt`, `clippy`, `test`, `deny`)
   and disposition/handoff are published.

## Deliverables
1. `artifacts/hphys0233-h1-transient-lane-diagnostic.md`
2. `artifacts/hphys0233-residual-authority-gap-matrix.md`
3. `artifacts/hphys0233-contract-implementation-evidence.md`
4. `artifacts/hphys0233-contract-test-implementation-evidence.md`
5. `artifacts/hphys0233-preimplementation-contract-gate.md`
6. `artifacts/hphys0233-implementation-and-test-evidence.md`
7. `artifacts/hphys0233-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/hphys0233_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `/workdir/wepp-forest_260430_baseline/src/perc.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/frsoil.for`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0232-wb18-hourly-lane-percolation-alignment-001/artifacts/hphys0232_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0232-wb18-hourly-lane-percolation-alignment-001/artifacts/worker-handoff.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0233-wb18-daily-restrictive-conductivity-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`

## Phase Plan
### Phase A - Intake and scaffolding
- Confirm immediate-next-action scope from HPHYS0232 handoff.
- Prepare queued artifacts and kickoff prompt.

### Phase B - Contract-first updates
- Amend `SC-PERC-001` for daily restrictive conductivity branch and WB13
  publication lineage posture.
- Update/add contract-derived tests.
- Record pre-implementation contract gate evidence.

### Phase C - Implementation
- Project restrictive-layer runtime symbols from soil inputs.
- Consume restrictive-layer conductivity branch in WB18 percolation daily lane.
- Harden WB13 `D` publication lineage to avoid stale state shadowing.

### Phase D - Rerun + adjudication
- Execute `H1..H39` rerun and semantic comparator.
- Publish residual delta matrix and H1 day-1..7 transient comparison.
- Run workspace gates and publish disposition/handoff.

## Exit Criteria
- `MEASURE-HP233-001..006` satisfied and evidenced.
- Stream-level HOLD/GO decision is explicit in disposition.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/runner/docs/test changes only; no credentials/network.
