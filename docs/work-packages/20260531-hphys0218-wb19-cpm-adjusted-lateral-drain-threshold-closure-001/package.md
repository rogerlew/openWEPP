# 20260531-hphys0218-wb19-cpm-adjusted-lateral-drain-threshold-closure-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0218 to close persistent `latqcc` residual saturation (and reduce
coupled `Dp` drift where impacted) by enforcing baseline-authoritative
WB19 lateral/drain threshold lineage in openWEPP runtime kernels:
`drfc_equivalent = wb18_perc_fc_#### + (1-cpm_####)*dg_####`.

## Why This Package Exists
Post-HPHYS0216D rerun (`HPHYS0217`) confirmed `ProfileFCStore` remediation,
but `latqcc` and `Dp` remain fully saturated (`39/39`) with early-day
over-withdraw signatures. Baseline `watbal.for` lateral/drain logic applies a
`drfc`-style threshold that includes entrapped-air depth; current WB19 runtime
path consumes `wb18_perc_fc_####` directly without the `cpm` adjustment.
This package closes that symbol-lineage gap under contract-first sequencing.

## Scope
### Included
- Canonical contract amendments for WB19 lateral/drain threshold lineage and
  required `cpm_####` symbol obligations.
- Contract-derived tests for:
  - `drfc`-equivalent threshold application in WB19 lateral withdrawal,
  - coupled WB13 `latqcc`/`Tile`/`Qd` publication continuity.
- Production updates in WB19 helper/kernel logic and seed-state coupling where
  required to preserve consistent threshold semantics.
- Fresh `unpalatable-rind` 39-hillslope rerun + semantic diagnostics.

### Explicitly Out of Scope
- `Total-Soil` / `SoilWaterTotal` closure (`HPHYS0219` scope).
- Integrated hold-lift adjudication (`HPHYS0220` scope).
- Watershed-lane process-authority modifications.

## Closure Measures (Required)
1. `MEASURE-HP218-001`: canonical contracts codify WB19 `drfc`-equivalent
   threshold authority and required runtime-symbol lineage (`cpm_####`, `dg_####`).
2. `MEASURE-HP218-002`: contract-derived tests fail pre-change and pass with
   threshold-corrected WB19 lateral/drain behavior.
3. `MEASURE-HP218-003`: 39-hillslope rerun completes and shows improved
   `latqcc` summary metrics versus HPHYS0217 baseline.
4. `MEASURE-HP218-004`: disposition and handoff publish explicit next actions
   for any residual open families.

## Deliverables
1. `artifacts/hphys0218-contract-implementation-evidence.md`
2. `artifacts/hphys0218-contract-test-implementation-evidence.md`
3. `artifacts/hphys0218-preimplementation-contract-gate.md`
4. `artifacts/hphys0218-implementation-and-test-evidence.md`
5. `artifacts/hphys0218-kernel-profile-compliance-checklist.md`
6. `artifacts/hphys0218-residual-gap-matrix.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0218_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contracts (`SC-WATBAL-001`, `SC-SUBHYD-001`,
   `SC-SYSTEM-001`) for WB19 threshold lineage obligations.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production runtime code.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in `docs/specifications/science-contracts/contracts/SC-*.md`.
- Baseline comparator and migration provenance:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/parity-only substitutions are permitted in production kernels.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0217-post-0216d-coupled-family-rerun-readjudication-001/artifacts/worker-handoff.md`
- `/tmp/hphys0217_20260531T071120Z/parity/reports/hillslope_semantic_summary.json`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0218-wb19-cpm-adjusted-lateral-drain-threshold-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0218_wb19_cpm_threshold_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0217 handoff authorization and freeze HPHYS0218 scope to WB19
  threshold lineage remediation.

### Phase B - Contract amendments
- Amend canonical SC docs for WB19 `drfc`-equivalent threshold obligations and
  runtime symbol lineage.

### Phase C - Contract-derived tests
- Add targeted tests for cpm-adjusted threshold behavior and coupling guards.

### Phase D - Pre-implementation gate
- Record contract/test readiness evidence before production edits.

### Phase E - Production implementation
- Implement WB19 threshold correction and required symbol guards.

### Phase F - Validation and rerun diagnostics
- Run:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Run fresh 39-hillslope rerun + semantic summaries.

### Phase G - Disposition and handoff
- Publish residual matrix, disposition, dual review/verification, and next
  package handoff.

## Exit Criteria
- Closure measures `MEASURE-HP218-001..004` are satisfied and evidenced.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contracts/kernel/test changes only; no external auth/network changes.
