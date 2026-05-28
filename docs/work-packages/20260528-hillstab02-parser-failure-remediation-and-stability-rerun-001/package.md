# 20260528-hillstab02-parser-failure-remediation-and-stability-rerun-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: HOLD

## Objective
Patch dominant parser compatibility failures observed in HILLSTAB01
(`SOL-E-006`, `MAN-E-009`), then rerun workspace tests and broad hillslope
stability cohorts to quantify pass-rate recovery.

## Rationale
HILLSTAB01 established a hard HOLD (`0/1185` pass) and identified parser
compatibility defects as the primary blocker. This package performs focused
remediation and rerun validation to establish whether parser closure materially
lifts cohort stability.

## Scope
### Included
- Contract-first parser compatibility closure work for legacy `.sol` and `.man`
  intake behaviors implicated by:
  - `SOL-E-006` variant arity mismatch
  - `MAN-E-009` reference-domain violations
- Contract-derived parser tests and fixture coverage updates.
- Production parser changes in openWEPP parser surfaces as required.
- Validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Re-run broad stability cohorts (1166 + release-gate watchlist) and publish
  delta vs HILLSTAB01.

### Excluded
- Non-parser physics model redesign unrelated to identified failure families.
- Watershed CLI feature expansion not required by parser compatibility closure.

## Deliverables
1. Parser remediation analysis:
   - `artifacts/hillstab02-remediation-report.md`
2. Rerun outcomes:
   - `artifacts/hillstab02-rerun-results.json`
   - `artifacts/hillstab02-rerun-delta-report.md`
3. Required package artifacts:
   - `artifacts/hillstab02-contract-implementation-evidence.md`
   - `artifacts/hillstab02-contract-test-implementation-evidence.md`
   - `artifacts/hillstab02-preimplementation-contract-gate.md`
   - `artifacts/hillstab02-implementation-and-test-evidence.md`
   - `artifacts/hillstab02-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/hillstab02_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/artifacts/hillstab01-stability-report.md`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/soil/**`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management/**`
- `/workdir/openWEPP/crates/openwepp-runner/src/hillslope/mod.rs`
- `/workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv`
- `/workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv`

## Intended Write Set
- `docs/work-packages/20260528-hillstab02-parser-failure-remediation-and-stability-rerun-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md` (if required)
- `crates/openwepp-input-contract/src/parsers/soil/**` (if required)
- `crates/openwepp-input-contract/src/parsers/management/**` (if required)
- `crates/openwepp-input-contract/tests/**` (if required)
- `crates/openwepp-runner/**` (if required)
- `tests/integration/**` (if required)

## Mandatory Contract-First Sequence
1. Implement required canonical contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Apply production parser/runtime code edits.

## Truthfulness Requirement
Each evidence artifact must label evidence class explicitly:
- `Static:` read/reasoned evidence
- `Ran:` executed command evidence

## Phase Plan
### Phase A - Failure family decomposition
- Use HILLSTAB01 result corpus to enumerate concrete `SOL-E-006` and `MAN-E-009`
  sub-patterns and representative fixtures.

### Phase B - Contract authority updates
- Amend canonical parser contract text where legacy-authoritative compatibility
  behavior is missing/incorrect.

### Phase C - Contract-derived tests + pre-implementation gate
- Add failing tests for each dominant parser sub-pattern.
- Record pre-implementation contract gate artifact.

### Phase D - Production parser remediation
- Implement parser support changes with typed errors and explicit guards.
- Do not introduce silent defaults/clamping.

### Phase E - Validation and stability rerun
- Run required cargo quality gates.
- Re-run 1166 + watchlist cohorts with the HILLSTAB harness.
- Publish delta report vs HILLSTAB01.

### Phase F - Review, verification, disposition
- Complete dual review and dual verification artifacts.
- Publish GO/HOLD disposition and worker handoff.

## Exit Criteria
- `SOL-E-006` and `MAN-E-009` families are eliminated or reduced with explicit
  residual accounting and rationale.
- Workspace gates pass (`fmt`, `clippy`, `test`, `deny`).
- Stability rerun artifacts are produced with pass/fail deltas vs HILLSTAB01.
- Disposition explicitly states whether hold-lift is supportable.

## Security Impact Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: parser contract/code/test and local execution harness rerun only.

## Autonomy
Package is execution-ready and must be executable end-to-end without user
intervention unless hard-blocked.
