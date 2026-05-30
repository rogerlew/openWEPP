# 20260530-hphys0207-fcwp-depth-authority-tail-closure-001

## Status
- state: completed
- date: 2026-05-30
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Close the FC/WP depth-authority gap identified after HPHYS0206 by making
profile-capacity and profile-store aggregation depth domains explicit and
consistent, including canonical policy for normalized-tail handling.

## Why This Package Exists
HPHYS0206 delivered deterministic normalized-layer overlap mapping and typed
fail-closed guards, but residuals stayed saturated (`ProfileFCStore 39/39`,
`ProfileWPStore 39/39`) and worsened in mean magnitude versus HPHYS0205.
Review evidence indicates a depth-authority mismatch between normalized profile
surfaces and parser-layer FC/WP aggregation extent, plus silent unconsumed
normalized-tail behavior.

## Scope
### Included
- Amend canonical contracts to define authoritative depth domain for:
  - `ProfileDepth`
  - `ProfilePorosityCap`
  - `ProfileFCStore`
  - `ProfileWPStore`
- Encode explicit normalized-tail policy (consume/project/reject) with typed
  behavior and no silent truncation.
- Add contract-derived tests for depth-authority closure and tail-policy
  behavior.
- Implement runtime/publication changes required by those contracts.
- Re-run workspace gates and 39-hillslope diagnostics with predecessor deltas
  versus HPHYS0206, HPHYS0205, and HPARITY02.

### Explicitly Out of Scope
- RM/ET/snow/runoff closure outside FC/WP lineage scope.
- Watershed/channel/impoundment behavior changes.
- Broad robustness package goals owned by HPHYS0203.

## Closure Measures (Required)
1. `MEASURE-HP207-001`: canonical contracts explicitly define FC/WP depth
   authority and normalized-tail handling policy.
2. `MEASURE-HP207-002`: contract-derived tests enforce depth authority, tail
   handling, and typed failure behavior.
3. `MEASURE-HP207-003`: workspace gates pass:
   `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo deny check`.
4. `MEASURE-HP207-004`: 39-hillslope rerun evidence includes explicit
   predecessor deltas and must show no FC/WP residual-magnitude regression
   versus HPHYS0205.

## Deliverables
1. `artifacts/hphys0207-physics-gap-matrix.md`
2. `artifacts/hphys0207-contract-implementation-evidence.md`
3. `artifacts/hphys0207-contract-test-implementation-evidence.md`
4. `artifacts/hphys0207-preimplementation-contract-gate.md`
5. `artifacts/hphys0207-implementation-and-test-evidence.md`
6. `artifacts/hphys0207-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0207_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index amendments for depth authority
   and normalized-tail policy.
2. Implement contract-derived tests for that authority and policy.
3. Record pre-implementation contract-gate evidence before production edits.
4. Apply production runtime/publication updates for FC/WP depth-authority scope.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Physics and symbol provenance must trace to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No surrogate/proxy equations or silent fallback defaults are permitted.
- No rollback to seed-authoritative FC/WP publication is allowed.
- Variable naming continuity with legacy WEPP symbols remains required.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0206-fcwp-layer-normalization-mapping-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0206-fcwp-layer-normalization-mapping-closure-001/artifacts/hphys0206_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0206-fcwp-layer-normalization-mapping-closure-001/artifacts/claude-code-review-findings.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/tmp/hparity02_20260529T204555Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0205_20260530T022235Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0206_20260530T032538Z/parity/reports/hillslope_semantic_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260530-hphys0207-fcwp-depth-authority-tail-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
- `tests/integration/parser_runtime_seam_integration.rs`
- `tests/integration/**` (HPHYS0207-specific vectors)

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0207 authorization and freeze scope to FC/WP depth authority and
  normalized-tail policy closure.

### Phase B - Contract/spec authority updates
- Amend canonical contracts/index for depth-authority and tail-policy authority.

### Phase C - Contract-derived tests
- Add depth-authority and tail-policy tests with typed fail behavior checks.

### Phase D - Pre-implementation contract gate
- Record contract/test readiness before production updates.

### Phase E - Production implementation
- Implement depth-authority and tail-policy closure for FC/WP publication path.

### Phase F - Validation and diagnostics rerun
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Re-run 39-hillslope diagnostics and publish fail-count + residual-magnitude
  predecessor deltas.

### Phase G - Dual review, dual verification, disposition
- Complete dual review/verification artifacts and publish disposition.

## Exit Criteria
- Closure measures `MEASURE-HP207-001..004` are satisfied and evidenced.
- FC/WP depth-authority and normalized-tail policy are fully traceable in
  canonical contracts, tests, and implementation.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test updates only; no external auth/network
  changes.
