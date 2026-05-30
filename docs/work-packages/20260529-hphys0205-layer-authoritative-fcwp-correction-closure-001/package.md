# 20260529-hphys0205-layer-authoritative-fcwp-correction-closure-001

## Status
- state: completed
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Resolve the remaining FC/WP residuals by keeping WB13 publication
layer-authoritative (`thetfc_####`/`thetdr_####`) while migrating
baseline-authoritative correction lineage into those layer symbols.

## Why This Package Exists
HPHYS0202 correctly removed FC/WP seed publication authority but exposed that
current layer symbols are not yet carrying the corrected baseline lineage.
HPHYS0205 closes that process-authority gap without reverting to seed-authority.

## Scope
### Included
- Amend canonical contract text so `thetfc_####`/`thetdr_####` layer symbols are
  explicitly bound to baseline-corrected lineage, not raw parser values.
- Add contract-derived tests that enforce corrected-layer lineage and explicit
  no-seed-override WB13 publication behavior.
- Implement runtime projection changes required to publish corrected FC/WP via
  layer symbols.
- Re-run workspace gates and 39-hillslope diagnostics with predecessor-delta
  reporting against HPARITY02 and HPHYS0202 evidence.

### Explicitly Out of Scope
- RM/ET/snow/runoff closure outside FC/WP lineage scope.
- Watershed/channel/impoundment behavior changes.
- Broad robustness package objectives already queued under HPHYS0203.

## Closure Measures (Required)
1. `MEASURE-HP205-001`: canonical contracts explicitly bind authoritative
   `thetfc_####`/`thetdr_####` layer symbols to baseline-corrected lineage
   required for WB13 FC/WP publication.
2. `MEASURE-HP205-002`: contract-derived tests prove:
   - WB13 FC/WP publication consumes corrected layer symbols (not seed
     overrides),
   - invalid corrected-layer state hard-fails with typed guards.
3. `MEASURE-HP205-003`: workspace validation gates pass:
   `fmt`, `clippy`, `test`, `deny`.
4. `MEASURE-HP205-004`: 39-hillslope diagnostic rerun is produced with explicit
   predecessor deltas vs:
   - HPARITY02 (`ProfileFCStore 27/39`, `ProfileWPStore 1/39`),
   - HPHYS0202 (`ProfileFCStore 39/39`, `ProfileWPStore 39/39`).

## Deliverables
1. `artifacts/hphys0205-physics-gap-matrix.md`
2. `artifacts/hphys0205-contract-implementation-evidence.md`
3. `artifacts/hphys0205-contract-test-implementation-evidence.md`
4. `artifacts/hphys0205-preimplementation-contract-gate.md`
5. `artifacts/hphys0205-implementation-and-test-evidence.md`
6. `artifacts/hphys0205-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0205_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index amendments for corrected
   `thetfc_####`/`thetdr_####` authority.
2. Implement contract-derived tests for corrected-layer lineage and WB13
   no-seed-override behavior.
3. Record pre-implementation contract-gate evidence before production edits.
4. Apply production projection/publication updates for FC/WP lineage only.

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
- No rollback to seed-authoritative FC/WP publication is allowed in this
  package; publication authority remains layer-authoritative.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0202-profile-fc-wp-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0202-profile-fc-wp-lineage-closure-001/artifacts/hphys0202_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0202-profile-fc-wp-lineage-closure-001/artifacts/claude-code-review-findings.md`
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

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hphys0205-layer-authoritative-fcwp-correction-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
- `tests/integration/**` (new HPHYS0205 lineage-closure vectors)

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0205 authorization and freeze to FC/WP corrected-layer lineage.

### Phase B - Contract/spec authority updates
- Amend SC contracts/index for corrected layer-symbol authority and alias
  lineage obligations.

### Phase C - Contract-derived tests
- Add tests for corrected layer-symbol projection and WB13 FC/WP publication
  behavior (no seed override).

### Phase D - Pre-implementation contract gate
- Record contract/test readiness before production updates.

### Phase E - Production implementation
- Implement corrected-layer projection closure and keep WB13 publication
  layer-authoritative.

### Phase F - Validation and diagnostics rerun
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Re-run 39-hillslope diagnostics and publish predecessor-delta summary.

### Phase G - Dual review, dual verification, disposition
- Complete dual review/verification artifacts and publish disposition.

## Exit Criteria
- Closure measures `MEASURE-HP205-001..004` are satisfied and evidenced.
- FC/WP correction lineage is promotable on process authority with explicit
  diagnostic deltas documented.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test updates only; no external auth/network
  changes.
