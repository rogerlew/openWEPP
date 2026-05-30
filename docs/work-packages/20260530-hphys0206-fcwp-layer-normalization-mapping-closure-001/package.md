# 20260530-hphys0206-fcwp-layer-normalization-mapping-closure-001

## Status
- state: queued
- date: 2026-05-30
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Close the remaining FC/WP residual after HPHYS0205 by aligning authoritative
corrected-layer projection with baseline-authoritative layer normalization and
deterministic layer mapping semantics, while preserving layer-authoritative WB13
publication (`thetfc_####`/`thetdr_####`).

## Why This Package Exists
HPHYS0205 moved correction lineage into authoritative layer symbols and reduced
FC/WP residual magnitudes, but semantic fail-count closure remains saturated
(`ProfileFCStore 39/39`, `ProfileWPStore 39/39`). Follow-on evidence indicates
remaining gaps are likely in layer-set normalization/mapping and fallback
authority semantics rather than missing correction activation.

## Scope
### Included
- Amend canonical contracts to make layer-set normalization and layer-index
  mapping obligations explicit for corrected FC/WP authority surfaces.
- Add contract-derived tests for:
  - normalized layer-set consistency between corrected FC/WP and profile-capacity
    authorities,
  - deterministic mapping between projected corrected layers and emitted
    `thetfc_####`/`thetdr_####` symbols,
  - typed fail-closed behavior where corrected-lineage authority is required.
- Implement projection/runtime changes needed to satisfy the above authorities.
- Re-run workspace gates and 39-hillslope diagnostics with predecessor deltas
  vs HPHYS0205 and HPARITY02.

### Explicitly Out of Scope
- RM/ET/snow/runoff closure outside FC/WP lineage scope.
- Watershed/channel/impoundment behavior changes.
- Broad robustness package objectives queued under HPHYS0203.

## Closure Measures (Required)
1. `MEASURE-HP206-001`: canonical contracts explicitly encode authoritative
   corrected-layer normalization/mapping semantics for FC/WP publication
   surfaces.
2. `MEASURE-HP206-002`: contract-derived tests enforce corrected-layer
   normalization/mapping and typed fail-closed authority behavior.
3. `MEASURE-HP206-003`: workspace validation gates pass:
   `fmt`, `clippy`, `test`, `deny`.
4. `MEASURE-HP206-004`: 39-hillslope rerun evidence is produced with explicit
   predecessor deltas vs:
   - HPHYS0205 summary
     (`/tmp/hphys0205_20260530T022235Z/parity/reports/hillslope_semantic_summary.json`),
   - HPARITY02 disposition baseline counts
     (`ProfileFCStore 27/39`, `ProfileWPStore 1/39`),
   and includes both fail-count and residual-magnitude reporting for FC/WP.

## Deliverables
1. `artifacts/hphys0206-physics-gap-matrix.md`
2. `artifacts/hphys0206-contract-implementation-evidence.md`
3. `artifacts/hphys0206-contract-test-implementation-evidence.md`
4. `artifacts/hphys0206-preimplementation-contract-gate.md`
5. `artifacts/hphys0206-implementation-and-test-evidence.md`
6. `artifacts/hphys0206-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0206_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index amendments for corrected-layer
   normalization/mapping authority.
2. Implement contract-derived tests for normalization/mapping closure and
   fail-closed authority behavior.
3. Record pre-implementation contract-gate evidence before production edits.
4. Apply production projection/runtime updates for FC/WP lineage scope only.

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
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0205-layer-authoritative-fcwp-correction-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0205-layer-authoritative-fcwp-correction-closure-001/artifacts/hphys0205_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0205-layer-authoritative-fcwp-correction-closure-001/artifacts/claude-code-review-findings.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/tmp/hphys0202_20260530T003833Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0205_20260530T022235Z/parity/reports/hillslope_semantic_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260530-hphys0206-fcwp-layer-normalization-mapping-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
- `tests/integration/parser_runtime_seam_integration.rs`
- `tests/integration/**` (HPHYS0206-specific vectors)

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0206 authorization and freeze scope to FC/WP
  normalization/mapping closure.

### Phase B - Contract/spec authority updates
- Amend canonical contracts/index for corrected-layer normalization/mapping
  authority and fail-closed requirements.

### Phase C - Contract-derived tests
- Add tests for corrected-layer normalization/mapping and typed fail-closed
  authority behavior.

### Phase D - Pre-implementation contract gate
- Record contract/test readiness before production updates.

### Phase E - Production implementation
- Implement corrected-layer normalization/mapping closure while retaining
  layer-authoritative WB13 publication.

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
- Closure measures `MEASURE-HP206-001..004` are satisfied and evidenced.
- FC/WP normalization/mapping authority and implementation are fully traceable;
  unresolved residuals (if any) are explicitly dispositioned with follow-on
  ownership.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test updates only; no external auth/network
  changes.
