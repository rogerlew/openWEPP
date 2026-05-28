# 20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: HOLD

## Objective
Execute a broad hillslope stability gate for `openwepp-cli-hill` release
build using two real-world cohorts:
1) the 1166-seed hillslope cohort from `wepp-forest`, and
2) the single-hillslope release-gate watchlist cohort.

## Why This Package Exists
Recent hillslope CLI optimization work established local performance
improvements, but broad stability has not yet been demonstrated against the
larger historical cohort and release-gate seeds maintained in `wepp-forest`.
This package closes that validation gap with reproducible run evidence.

## Scope
### Included
- Resolve and validate canonical cohort inputs from `wepp-forest`:
  - 1166-seed cohort (`defect_seeds.csv`)
  - hillslope release-gate watchlist (`hillslope_watchlist.csv`)
- Build/reuse release `openwepp-cli-hill`.
- Execute cohort runs with reproducible harness commands.
- Record per-cohort pass/fail counts, failure signatures, and elapsed timing
  summaries.
- Publish package evidence artifacts, gate results, and disposition.

### Explicitly Out of Scope
- New kernel/process-physics implementation.
- Contract authority changes unless an execution blocker requires explicit
  amendment.
- Watershed CLI execution or routing/parity analysis.

## Deliverables
1. Stability harness and raw result artifact:
   - `artifacts/hillstab01_stability_cohort.py`
   - `artifacts/hillstab01-stability-results.json`
2. Stability summary report:
   - `artifacts/hillstab01-stability-report.md`
3. Required governance artifacts:
   - `artifacts/hillstab01-contract-implementation-evidence.md`
   - `artifacts/hillstab01-contract-test-implementation-evidence.md`
   - `artifacts/hillstab01-preimplementation-contract-gate.md`
   - `artifacts/hillstab01-implementation-and-test-evidence.md`
   - `artifacts/hillstab01-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/hillstab01_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language only if stability execution reveals a
   contract authority mismatch that must be codified before code changes.
2. Implement contract-derived tests only if behavior/contract surfaces are
   changed.
3. Record pre-implementation contract-gate evidence.
4. Apply production code changes (if needed).

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical contract authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy cohort provenance authority remains in `wepp-forest` artifacts:
  - `/workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv`
  - `/workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv`
- Baseline binary authority anchor remains
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `/workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv`
- `/workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv`
- `/workdir/wepp-forest/tools/wb05e_global_closure_scoreboard.py`
- `/workdir/wepp-forest/tools/run_hillslope_watchlist.py`

## Intended Write Set
- `docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/**` (only if execution uncovers defects requiring fix)

## Phase Plan
### Phase A - Intake and cohort source validation
- Validate both cohort source files and expected field coverage.
- Validate source run directories and required input surfaces.

### Phase B - Harness preparation
- Implement a reproducible local harness to:
  - stage per-case run inputs without mutating source directories,
  - generate/open runfiles for `openwepp-cli-hill`,
  - execute release binary with typed run metadata capture,
  - persist structured results JSON.

### Phase C - 1166 cohort execution
- Run the full 1166-seed cohort against release `openwepp-cli-hill`.
- Record elapsed timing summary, pass/fail counts, and top failure signatures.

### Phase D - Release-gate watchlist execution
- Run the hillslope release-gate watchlist cohort against release
  `openwepp-cli-hill`.
- Record elapsed timing summary, pass/fail counts, and failure signatures.

### Phase E - Evidence, review, and disposition
- Publish summary report and governance artifacts.
- Set disposition to GO/HOLD based on observed stability results.

## Exit Criteria
- Both cohorts are executed with reproducible command provenance.
- Structured per-case outcomes are stored in artifact JSON.
- Summary report includes pass/fail counts and failure signature rollups.
- Required package artifacts are populated with truthfulness labels.
- Disposition is explicit (GO or HOLD) with immediate next actions.

## Execution Outcome
- HILLSTAB01 executed end-to-end on 2026-05-28 UTC.
- Stability outcome: HOLD.
- Cohort execution summary:
  - `wb05b_1166`: 1166 total, 0 passed, 1166 failed.
  - `release_gate_watchlist`: 19 total, 0 passed, 19 failed.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local execution harness + docs/evidence only; no external service
  or credential surface changes.
