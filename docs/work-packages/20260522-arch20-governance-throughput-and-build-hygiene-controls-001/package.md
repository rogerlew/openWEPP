# 20260522-arch20-governance-throughput-and-build-hygiene-controls-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Implement `CRF-008` and `CRF-009` by defining measurable governance-throughput
controls, explicit WIP/closure policy, and workspace build-discipline rules
that reduce process overhead while preserving correctness gates.

## Why This Package Exists
ARCH14 identified risk that process scaffolding could outpace engine delivery,
and noted inconsistent build hygiene signals. ARCH20 establishes explicit
controls and acceptance criteria linking work-package output to delivered engine
capability and reproducible workspace validation behavior.

## Scope
### Included
- Define governance throughput rubric connecting package activity to concrete
  engine capability outcomes.
- Define WIP/closure policy and guardrails for package churn control.
- Define workspace build-discipline policy (canonical command entrypoints,
  crate-local vs workspace-root execution rules).
- Define evidence requirements for package completion claims (`Static` vs
  `Ran`) and minimum gate expectations by package type.
- Produce dual review/disposition/verification artifacts.

### Explicitly Out of Scope
- Implementing kernel/parser/runtime feature code.
- Re-opening ARCH15-ARCH19 technical contract decisions except where
  governance controls reference them.
- CI/tooling automation implementation beyond documented policy.

## Deliverables
1. Governance throughput rubric:
   - `artifacts/governance-throughput-rubric.md`
2. WIP/closure policy:
   - `artifacts/work-package-wip-and-closure-policy.md`
3. Workspace build-discipline policy:
   - `artifacts/workspace-build-discipline-policy.md`
4. Evidence classification and gate policy:
   - `artifacts/evidence-and-gate-policy.md`
5. Worker/governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/arch20_disposition.md`
6. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/remediation-work-package-queue.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch16-scheduler-hot-path-surface-optimization-001/artifacts/arch16_disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch17-parser-to-simulation-seam-ownership-and-integration-closure-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch18-hbp-authority-and-convergence-closure-001/`

## Intended Write Set
- `docs/work-packages/20260522-arch20-governance-throughput-and-build-hygiene-controls-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/remediation-work-package-queue.md`

## Phase Plan
### Phase 0 - Baseline Assessment
- Summarize observed throughput/hygiene pain points from ARCH14-ARCH19 history.

### Phase 1 - Policy Drafting
- Draft throughput rubric, WIP/closure policy, and build-discipline policy.

### Phase 2 - Acceptance Mapping
- Define acceptance criteria and evidence requirements for governance adoption.

### Phase 3 - Review and Closeout
- Run docs-package validation checks and complete dual review/verification.

## Exit Criteria
- Throughput rubric, WIP/closure policy, and build-discipline policy are
  authored and internally consistent.
- Evidence/gate policy clearly distinguishes docs-only vs code-touch packages.
- Dual review and verification artifacts are complete.
- If code surfaces are touched, standard rust gates must pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: governance/docs package only.
