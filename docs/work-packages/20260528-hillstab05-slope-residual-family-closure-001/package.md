# 20260528-hillstab05-slope-residual-family-closure-001

## Status
- state: complete
- date: 2026-05-29
- timezone: UTC
- decision: HOLD

## Objective
Close residual slope parser/runtime failure families observed after HILLSTAB02,
including:
- slope token parse (`line 7, column 3`) failures,
- endpoint constraint branch failures,
- cross-OFE boundary mismatch branch failures,
- `HS-RUNTIME-E-023` derived-average-slope runtime-domain failures.

## Rationale
HILLSTAB02 resolved dominant parser compatibility blockers but left a stable
slope-family residual set that continues to prevent cohort pass-rate recovery
and hold-lift progress.

## Scope
### Included
- Contract authority updates for slope parser/runtime guards where needed.
- Contract-derived tests for each residual slope branch.
- Pre-implementation contract gate evidence.
- Production parser/runtime/orchestrator updates required for slope-family
  closure.
- Required validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Cohort rerun and delta accounting against HILLSTAB02.

### Excluded
- `HKERNEL-WB16-PEAK-E-003` closure (handled in sibling follow-on package).
- `HKERNEL-EROD14-WAVE2-E-003` closure (handled in sibling follow-on package).
- Watershed CLI feature expansion unrelated to slope-family closure.

## Deliverables
1. Remediation analysis:
   - `artifacts/hillstab05-slope-remediation-report.md`
2. Rerun outcomes:
   - `artifacts/hillstab05-rerun-results.json`
   - `artifacts/hillstab05-rerun-delta-report.md`
3. Required package artifacts:
   - `artifacts/hillstab05-contract-implementation-evidence.md`
   - `artifacts/hillstab05-contract-test-implementation-evidence.md`
   - `artifacts/hillstab05-preimplementation-contract-gate.md`
   - `artifacts/hillstab05-implementation-and-test-evidence.md`
   - `artifacts/hillstab05-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/hillstab05_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab02-parser-failure-remediation-and-stability-rerun-001/artifacts/hillstab02-rerun-delta-report.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab02-parser-failure-remediation-and-stability-rerun-001/artifacts/hillstab02_disposition.md`
- `/workdir/openWEPP/tests/integration/infile_slope_parser_contract.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/slope.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/**`
- `/workdir/openWEPP/crates/openwepp-runner/**`
- `/workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv`
- `/workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv`

## Intended Write Set
- `docs/work-packages/20260528-hillstab05-slope-residual-family-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` (if required)
- `tests/integration/infile_slope_parser_contract.rs` (if required)
- `tests/integration/parser_runtime_seam_integration.rs` (if required)
- `crates/openwepp-input-contract/src/parsers/slope.rs` (if required)
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
- Derive the residual slope failure set from HILLSTAB02 logs and classify each
  branch with representative case IDs and inputs.

### Phase B - Contract authority updates
- Amend canonical `SC-*` contract text for missing slope parser/runtime guard
  authority required by residual branches.

### Phase C - Contract-derived tests + pre-implementation gate
- Add failing vectors/tests for all identified slope residual branches.
- Record pre-implementation gate failure evidence before code edits.

### Phase D - Production closure
- Implement slope parser/runtime/orchestrator fixes for residual branches.
- Preserve typed error semantics; do not introduce silent defaults/clamping.

### Phase E - Validation + cohort rerun
- Run required cargo gates.
- Re-run 1166 + watchlist cohort harness.
- Publish delta vs HILLSTAB02 with explicit branch counts.

### Phase F - Review, verification, disposition
- Complete dual review and dual verification artifacts.
- Publish GO/HOLD disposition and worker handoff.

## Exit Criteria
- The slope residual family count is eliminated or explicitly bounded with
  authoritative rationale.
- Required workspace gates pass.
- Cohort rerun and delta artifacts are complete and reproducible.
- Disposition explicitly states hold-lift impact.

## Security Impact Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local parser/runtime closure and local harness rerun only.

## Autonomy
Package is execution-ready and must be executable end-to-end without user
intervention unless hard-blocked.
