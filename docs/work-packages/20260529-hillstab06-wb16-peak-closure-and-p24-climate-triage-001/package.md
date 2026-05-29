# 20260529-hillstab06-wb16-peak-closure-and-p24-climate-triage-001

## Status
- state: complete
- date: 2026-05-29
- timezone: UTC
- decision: GO

## Objective
Execute immediate follow-on actions from HILLSTAB05 by:
1. reducing or eliminating `HKERNEL-WB16-PEAK-E-003` residual failures, and
2. triaging/remediating the watchlist `p24` climate domain failure
   (`HS-SIMPIPE-E-001` `tmax < tmin`) when within scope and authority.

## Rationale
HILLSTAB05 eliminated slope-family blockers and raised cohort pass counts to
`90/1185`, but hold-lift remains blocked by dominant WB16 runtime failures
(`1094`) plus one watchlist climate residual (`p24`).

## Scope
### Included
- Contract authority updates for WB16 and climate-compatibility handling where
  required by baseline provenance and observed residuals.
- Contract-derived tests for newly identified WB16 and p24 residual branches.
- Pre-implementation contract gate evidence.
- Production runtime/kernel/orchestrator updates required for residual closure
  or explicit bounded triage.
- Required validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Cohort rerun and delta accounting against HILLSTAB05.

### Excluded
- New watershed feature expansion unrelated to residual closure.
- Unrelated parser families already closed in HILLSTAB05.

## Deliverables
1. Remediation analysis:
   - `artifacts/hillstab06-wb16-climate-remediation-report.md`
2. Rerun outcomes:
   - `artifacts/hillstab06-rerun-results.json`
   - `artifacts/hillstab06-rerun-delta-report.md`
3. Required package artifacts:
   - `artifacts/hillstab06-contract-implementation-evidence.md`
   - `artifacts/hillstab06-contract-test-implementation-evidence.md`
   - `artifacts/hillstab06-preimplementation-contract-gate.md`
   - `artifacts/hillstab06-implementation-and-test-evidence.md`
   - `artifacts/hillstab06-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/hillstab06_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab05-slope-residual-family-closure-001/artifacts/hillstab05-rerun-delta-report.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab05-slope-residual-family-closure-001/artifacts/hillstab05_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab05-slope-residual-family-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/tests/integration/wb16_peak_runoff_kernel_contract.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/**`
- `/workdir/openWEPP/crates/openwepp-runner/**`
- `/workdir/wepp-forest_260430_baseline/src/appmth.for`
- `/workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv`
- `/workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv`

## Intended Write Set
- `docs/work-packages/20260529-hillstab06-wb16-peak-closure-and-p24-climate-triage-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md` (if required)
- `tests/integration/wb16_peak_runoff_kernel_contract.rs` (if required)
- `tests/integration/cli03_runner_contract_derived_tests.rs` (if required)
- `crates/openwepp-hillslope-orchestrator/**` (if required)
- `crates/openwepp-runner/**` (if required)

## Mandatory Contract-First Sequence
1. Implement required canonical contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Apply production code edits.

## Truthfulness Requirement
Each evidence artifact must label evidence class explicitly:
- `Static:` read/reasoned evidence
- `Ran:` executed command evidence

## Phase Plan
### Phase A - Residual set decomposition
- Derive WB16 and p24 residual branch signatures from HILLSTAB05 rerun logs
  and identify authoritative baseline mismatch points.

### Phase B - Contract authority updates
- Amend canonical `SC-*` authority for any missing WB16/p24 compatibility or
  guard behavior required by baseline provenance.

### Phase C - Contract-derived tests + pre-implementation gate
- Add failing vectors/tests for each newly identified residual branch.
- Record pre-implementation gate failure evidence before code edits.

### Phase D - Production closure
- Implement WB16 and p24 runtime/kernel/orchestrator updates required by
  authority.
- Preserve typed error semantics; do not introduce silent defaults/clamping
  outside explicitly authorized compatibility branches.

### Phase E - Validation + cohort rerun
- Run required cargo gates.
- Re-run 1166 + watchlist cohort harness.
- Publish delta vs HILLSTAB05 with explicit family counts.

### Phase F - Review, verification, disposition
- Complete dual review and dual verification artifacts.
- Publish GO/HOLD disposition and worker handoff.

## Exit Criteria
- `HKERNEL-WB16-PEAK-E-003` is eliminated or materially reduced with explicit
  authority-backed residual rationale.
- Watchlist `p24` climate residual is either remediated or explicitly triaged
  with authoritative bounded follow-on.
- Required workspace gates pass.
- Cohort rerun and delta artifacts are complete and reproducible.
- Disposition explicitly states hold-lift impact.

## Security Impact Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/test/runtime closure and local harness rerun only.

## Autonomy
Package is execution-ready and must be executable end-to-end without user
intervention unless hard-blocked.
