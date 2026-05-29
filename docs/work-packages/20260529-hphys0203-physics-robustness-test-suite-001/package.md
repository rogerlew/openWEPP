# 20260529-hphys0203-physics-robustness-test-suite-001

## Status
- state: queued
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Build and execute a physics-robustness validation suite for hillslope water
balance publication surfaces, emphasizing invariants, conservation-consistent
behavior, domain guards, and perturbation stability.

## Why This Package Exists
Contract-authoritative process correctness needs stronger test evidence than
single-column parity metrics. HPHYS0203 establishes durable robustness gates
for current and follow-on water-balance work.

## Scope
### Included
- Add contract-derived robustness tests for targeted hydrology publication
  families (`Profile*`, `Total-Soil`, `SoilWaterTotal`, `latqcc`, `Dp`).
- Add deterministic regression fixtures from known problematic hillslopes.
- Add/update typed guard assertions for domain boundary behavior.
- Execute full workspace gates and record evidence.

### Explicitly Out of Scope
- Large runtime refactors not required by robustness-test closure.
- Watershed routing/impoundment closure work.
- Performance benchmarking.

## Closure Measures (Required)
1. `MEASURE-HP203-001`: robustness test suite covers conservation-consistent
   checks, monotonic/ordering expectations, unit/domain guard behavior, and
   non-finite protections for targeted surfaces.
2. `MEASURE-HP203-002`: at least one regression fixture per targeted residual
   family is encoded and passing under contract-authoritative expectations.
3. `MEASURE-HP203-003`: workspace validation gates pass:
   `fmt`, `clippy`, `test`, `deny`.
4. `MEASURE-HP203-004`: diagnostic parity rerun artifacts are produced and
   summarized without acting as sole closure gate.

## Deliverables
1. `artifacts/hphys0203-physics-gap-matrix.md`
2. `artifacts/hphys0203-contract-implementation-evidence.md`
3. `artifacts/hphys0203-contract-test-implementation-evidence.md`
4. `artifacts/hphys0203-preimplementation-contract-gate.md`
5. `artifacts/hphys0203-implementation-and-test-evidence.md`
6. `artifacts/hphys0203-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0203_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend/confirm canonical contract test obligations for robustness vectors.
2. Implement contract-derived robustness tests first.
3. Record pre-implementation contract-gate evidence.
4. Apply minimal production/test-harness updates required by those tests.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy comparator provenance anchor remains
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No silent fallback defaults or heuristic substitute physics in production
  execution paths.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0201-physics-first-gate-reframe-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0202-profile-fc-wp-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hphys0203-physics-robustness-test-suite-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/**` (new robustness/fixture tests scoped to targeted
  families)
- `crates/openwepp-runner/src/hillslope/mod.rs` (only if required by
  contract-derived robustness test closure)

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0203 authorization and targeted residual families.

### Phase B - Contract/spec authority updates
- Amend robustness obligations in canonical contracts/index as required.

### Phase C - Contract-derived test implementation
- Implement robustness/property/regression tests for targeted surfaces.

### Phase D - Pre-implementation contract gate
- Record readiness evidence before any production path updates.

### Phase E - Minimal production/harness edits
- Apply only edits required to satisfy contract-derived robustness tests.

### Phase F - Validation and diagnostics
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Produce diagnostic parity rerun summary for context.

### Phase G - Dual review, dual verification, disposition
- Complete review/verification artifacts and publish disposition.

## Exit Criteria
- Closure measures `MEASURE-HP203-001..004` are satisfied and evidenced.
- Robustness-test suite is promotable and reusable for follow-on packages.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/test/runtime guard updates only; no external
  auth/network changes.
