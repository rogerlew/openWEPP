# 20260528-hillstab04-erod14-wave2-domain-closure-001

## Status
- state: queued
- date: 2026-05-28
- timezone: UTC
- decision: pending

## Objective
Eliminate `HKERNEL-EROD14-WAVE2-E-003` runtime-domain failures across the
hillslope stability cohorts (1166 + release-gate watchlist) using
contract-first sequencing and publish rerun deltas vs HILLSTAB02.

## Rationale
After HILLSTAB02 parser closure, `HKERNEL-EROD14-WAVE2-E-003` is a dominant
residual family and a direct blocker to cohort pass-rate recovery.

## Scope
### Included
- Contract authority updates for EROD14 wave-2 closure semantics where needed.
- Contract-derived tests that reproduce and pin
  `HKERNEL-EROD14-WAVE2-E-003`.
- Pre-implementation contract gate evidence.
- Production runtime/kernel/orchestrator updates required to close this family.
- Required validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Cohort rerun and delta accounting against HILLSTAB02.

### Excluded
- `HKERNEL-WB16-PEAK-E-003` closure (handled in sibling follow-on package).
- Residual slope parser/runtime closure families (handled in follow-on package).
- Watershed CLI feature expansion unrelated to this failure family.

## Deliverables
1. Remediation analysis:
   - `artifacts/hillstab04-erod14-remediation-report.md`
2. Rerun outcomes:
   - `artifacts/hillstab04-rerun-results.json`
   - `artifacts/hillstab04-rerun-delta-report.md`
3. Required package artifacts:
   - `artifacts/hillstab04-contract-implementation-evidence.md`
   - `artifacts/hillstab04-contract-test-implementation-evidence.md`
   - `artifacts/hillstab04-preimplementation-contract-gate.md`
   - `artifacts/hillstab04-implementation-and-test-evidence.md`
   - `artifacts/hillstab04-kernel-profile-compliance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/hillstab04_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab02-parser-failure-remediation-and-stability-rerun-001/artifacts/hillstab02-rerun-delta-report.md`
- `/workdir/openWEPP/docs/work-packages/20260528-hillstab02-parser-failure-remediation-and-stability-rerun-001/artifacts/hillstab02_disposition.md`
- `/workdir/openWEPP/tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- `/workdir/openWEPP/tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/**`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/**`
- `/workdir/openWEPP/crates/openwepp-runner/**`
- `/workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv`
- `/workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv`

## Intended Write Set
- `docs/work-packages/20260528-hillstab04-erod14-wave2-domain-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (if required)
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` (if required)
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs` (if required)
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs` (if required)
- `crates/openwepp-hillslope-orchestrator/**` (if required)
- `crates/openwepp-kernel-contract/**` (if required)
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
- Derive the `HKERNEL-EROD14-WAVE2-E-003` failing case set from HILLSTAB02
  logs and classify dominant domain triggers.

### Phase B - Contract authority updates
- Amend canonical `SC-*` contract text for missing wave-2 domain/guard
  authority required by observed residuals.

### Phase C - Contract-derived tests + pre-implementation gate
- Add failing vectors/tests covering dominant trigger branches.
- Record pre-implementation gate failure evidence before code edits.

### Phase D - Production closure
- Implement runtime/kernel/orchestrator closure for wave-2 failure branches.
- Preserve typed error semantics; do not introduce silent defaults/clamping.

### Phase E - Validation + cohort rerun
- Run required cargo gates.
- Re-run 1166 + watchlist cohort harness.
- Publish delta vs HILLSTAB02 with explicit family counts.

### Phase F - Review, verification, disposition
- Complete dual review and dual verification artifacts.
- Publish GO/HOLD disposition and worker handoff.

## Exit Criteria
- `HKERNEL-EROD14-WAVE2-E-003` is eliminated from rerun cohorts or residuals
  are explicitly bounded with authoritative rationale.
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

