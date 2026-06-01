# 20260601-hphys0230-wb18-overdrainage-authority-closure-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Close WB18 deep-percolation over-drainage authority gap by migrating
legacy-authoritative percolation-rate behavior (`perc.for`/`purk.for`) into
openWEPP percolation execution so early-transient `Dp` does not drain the
profile to the FC cutoff in the first simulation week.

## Why This Package Exists
HPHYS0229 and the WB18 diagnostic established that persistent
`Total-Soil`/`SoilWaterTotal` deviations are driven by WB18 `Dp` over-drainage,
not WB19. The current WB18 implementation applies an incorrect conductivity
shape exponent path, producing early-day percolation bursts (order 40-82 mm/day
on H1) that are not baseline-authoritative. A contract-first WB18 correction is
required before additional residual-family hold-lift adjudication.

## Scope
### Included
- Amend canonical WB18 authority in `SC-PERC-001` to require per-layer `Bi`
  derivation from `FC/UL` ratio (`Bi = -2.655/log10(FC/UL)`) with explicit
  domain guards and legacy provenance.
- Add/adjust contract-derived tests for WB18 per-layer percolation behavior,
  including dynamic `Bi` damping.
- Record pre-implementation contract gate evidence.
- Update WB18 production percolation execution in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  to apply per-layer `Bi` authority.
- Execute targeted and workspace validation gates.
- Execute `unpalatable-rind` H1 acceptance trace and publish `Dp`/`Total-Soil`
  trajectory evidence; run cohort rerun/readjudication (`H1..H39`) to quantify
  residual movement after WB18 remediation.

### Explicitly Out of Scope
- WB19 lateral/drainage formula changes.
- Watershed routing kernel changes.
- Parser-format changes unrelated to WB18 execution.

## Closure Measures (Required)
1. `MEASURE-HP230-001`: canonical WB18 contract authority is amended with
   legacy-provenance `Bi` derivation and guard obligations.
2. `MEASURE-HP230-002`: contract-derived WB18 tests are updated/added and pass.
3. `MEASURE-HP230-003`: production WB18 percolation applies per-layer `Bi`
   dynamic damping and preserves typed hard-fail posture.
4. `MEASURE-HP230-004`: H1 acceptance trace shows early-day `Dp` collapse
   relative to pre-fix behavior (no 40-82 mm/day burst pattern).
5. `MEASURE-HP230-005`: post-change `H1..H39` semantic rerun artifacts are
   produced with valid alignment (`common_row_count > 0` for all hillslopes).
6. `MEASURE-HP230-006`: required guardrail + workspace gates pass
   (`fmt`,`clippy`,`test`,`deny`) and disposition/handoff are published.

## Deliverables
1. `artifacts/hphys0230-residual-authority-gap-matrix.md`
2. `artifacts/hphys0230-contract-implementation-evidence.md`
3. `artifacts/hphys0230-contract-test-implementation-evidence.md`
4. `artifacts/hphys0230-preimplementation-contract-gate.md`
5. `artifacts/hphys0230-implementation-and-test-evidence.md`
6. `artifacts/hphys0230-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0230_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

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
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/wepp-forest_260430_baseline/src/perc.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0229-post-0228-cohort-rerun-readjudication-001/artifacts/claude-code-wb18-overdrainage-diagnostic.md`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H1.parquet`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0230-wb18-overdrainage-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`

## Phase Plan
### Phase A - Intake and setup
- Confirm WB18 lane ownership from HPHYS0229 diagnostic and baseline sources.
- Prepare queued artifacts and execution prompt.

### Phase B - Contract-first updates
- Amend `SC-PERC-001` (and cross-contract pointers if required) with explicit
  `Bi` derivation authority and guard behavior.
- Update/add contract-derived tests.
- Record pre-implementation contract gate evidence.

### Phase C - Implementation + validation
- Implement WB18 percolation correction in production kernel.
- Run targeted WB18 tests and full workspace gates.
- Run H1 acceptance trace and cohort rerun/readjudication; publish deltas.
- Publish disposition, verification, and handoff.

## Exit Criteria
- `MEASURE-HP230-001..006` satisfied and evidenced.
- Integrated HPHYS stream remains explicit `HOLD` pending next production
  remediation closure adjudication.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local diagnostics/tests/docs only; no credentials/network.
