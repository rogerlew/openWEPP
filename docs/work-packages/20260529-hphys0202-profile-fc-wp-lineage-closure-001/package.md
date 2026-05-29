# 20260529-hphys0202-profile-fc-wp-lineage-closure-001

## Status
- state: queued
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Implement baseline-authoritative process lineage closure for
`ProfileFCStore` and `ProfileWPStore` publication surfaces in WB13 outputs,
with physics-correctness as the promotability gate.

## Why This Package Exists
HPARITY02 left residual mismatch on `ProfileFCStore` and `ProfileWPStore`.
Follow-up should verify authoritative process lineage and guard semantics first,
then treat parity deltas as diagnostic evidence.

## Scope
### Included
- Amend canonical contract text for profile FC/WP lineage if required.
- Add contract-derived tests for per-layer aggregation, units, and guards.
- Implement runtime/publication changes needed to satisfy contract authority.
- Run workspace gates and publish parity diagnostics for 39 hillslopes.

### Explicitly Out of Scope
- RM/ET/snow closure (handled outside this package scope).
- Dp/latqcc/SoilWaterTotal/Total-Soil implementation closure.
- Watershed/channel/impoundment runtime changes.

## Closure Measures (Required)
1. `MEASURE-HP202-001`: `ProfileFCStore` and `ProfileWPStore` publication
   lineage is traceably mapped to canonical layer-authoritative aggregation with
   explicit symbol aliases and units.
2. `MEASURE-HP202-002`: contract-derived tests for FC/WP publication lineage
   and guard behavior are implemented and passing.
3. `MEASURE-HP202-003`: workspace validation gates pass:
   `fmt`, `clippy`, `test`, `deny`.
4. `MEASURE-HP202-004`: 39-hillslope semantic rerun is produced and analyzed as
   diagnostic evidence (not primary closure gate).

## Deliverables
1. `artifacts/hphys0202-physics-gap-matrix.md`
2. `artifacts/hphys0202-contract-implementation-evidence.md`
3. `artifacts/hphys0202-contract-test-implementation-evidence.md`
4. `artifacts/hphys0202-preimplementation-contract-gate.md`
5. `artifacts/hphys0202-implementation-and-test-evidence.md`
6. `artifacts/hphys0202-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0202_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index amendments for FC/WP lineage.
2. Implement contract-derived tests for FC/WP publication and guards.
3. Record pre-implementation contract-gate evidence before production edits.
4. Apply production runtime/publication updates for this family only.

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

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0201-physics-first-gate-reframe-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity02-profile-capacity-storage-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/hillslope_semantic_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hphys0202-profile-fc-wp-lineage-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
- `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0202 authorization and freeze scope to FC/WP lineage closure.

### Phase B - Contract/spec authority updates
- Amend SC rows for FC/WP lineage, units, and guard semantics as needed.
- Update science-contract index references.

### Phase C - Contract-derived tests
- Add invariant tests for FC/WP aggregation lineage and typed guard behavior.

### Phase D - Pre-implementation contract gate
- Record contract/test readiness before production edits.

### Phase E - Production implementation
- Implement runtime/publication closure for FC/WP family only.

### Phase F - Validation and diagnostics rerun
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Rerun 39-hillslope semantic comparison and summarize diagnostics.

### Phase G - Dual review, dual verification, disposition
- Complete dual review/verification artifacts and publish disposition.

## Exit Criteria
- Closure measures `MEASURE-HP202-001..004` are satisfied and evidenced.
- FC/WP publication lineage is promotable on contract authority independent of
  parity-only interpretation.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test updates only; no external auth/network
  changes.
