# 20260531-hphys0219-wb19-coca-threshold-authority-correction-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0219 to correct WB19 lateral/drain threshold authority by
restoring baseline-authoritative `drfc` lineage to `coca_####` (not `cpm_####`)
and re-adjudicate coupled `Dp`/`latqcc` residual behavior on the
`unpalatable-rind` 39-hillslope lane.

## Why This Package Exists
HPHYS0218 improved `latqcc` mean residuals but regressed `Dp` mean residuals
across all 39 hillslopes while fail saturation remained unchanged. Legacy
`watbal.for` defines:

`drfc(i) = fc(i) + ((1 - coca(i)) * dg(i))`

The current openWEPP WB19 path uses `cpm_####` in this threshold lineage.
`cpm` and `coca` are distinct legacy coefficient families; conflating them
creates process-authority risk and likely contributes to observed regression.

## Scope
### Included
- Canonical contract amendments that encode WB19 `drfc` authority to
  `coca_####`.
- Contract-derived tests that hard-fail on missing/invalid `coca_####` and
  validate `drfc` realization behavior in WB19 lateral/drain kernels.
- Runtime input projection updates to publish `coca_####` symbols from
  authoritative corrected-layer lineage.
- WB19 production updates replacing `cpm_####` threshold usage with
  `coca_####`.
- Full gate run and fresh `unpalatable-rind` 39-hillslope rerun with semantic
  comparison vs HPHYS0218.

### Explicitly Out of Scope
- Independent closure of `Total-Soil` / `SoilWaterTotal` families.
- Watershed lane authority changes.
- Non-WB19 process-family refactors.

## Closure Measures (Required)
1. `MEASURE-HP219-001`: contracts codify WB19 `drfc` as
   `wb18_perc_fc_#### + (1-coca_####)*dg_####`.
2. `MEASURE-HP219-002`: contract-derived tests validate `coca_####` authority
   and typed guard posture (missing/non-finite/domain-invalid hard-fail).
3. `MEASURE-HP219-003`: rerun evidence compares HPHYS0219 vs HPHYS0218 and
   quantifies directional change for `Dp` and `latqcc`.
4. `MEASURE-HP219-004`: disposition/handoff publish residual posture and next
   package trigger.

## Deliverables
1. `artifacts/hphys0219-contract-implementation-evidence.md`
2. `artifacts/hphys0219-contract-test-implementation-evidence.md`
3. `artifacts/hphys0219-preimplementation-contract-gate.md`
4. `artifacts/hphys0219-implementation-and-test-evidence.md`
5. `artifacts/hphys0219-kernel-profile-compliance-checklist.md`
6. `artifacts/hphys0219-residual-gap-matrix.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0219_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contracts (`SC-WATBAL-001`, `SC-SUBHYD-001`,
   `SC-SYSTEM-001`) for WB19 `coca_####` threshold authority.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production runtime/kernel code.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in `docs/specifications/science-contracts/contracts/SC-*.md`.
- Baseline comparator/migration provenance:
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
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0218-wb19-cpm-adjusted-lateral-drain-threshold-closure-001/artifacts/worker-handoff.md`
- `/tmp/hphys0218_20260531T075251Z/parity/reports/hillslope_semantic_summary.json`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0219-wb19-coca-threshold-authority-correction-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0219_wb19_coca_threshold_contract.rs`
- WB19-adjacent integration fixtures requiring explicit `coca_####` runtime symbols.

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0218 handoff authorization and freeze scope to WB19 `drfc`
  coefficient-family authority correction (`cpm -> coca`).

### Phase B - Contract amendments
- Amend canonical SC docs for WB19 threshold lineage and required
  `coca_####` symbol family.

### Phase C - Contract-derived tests
- Add tests that prove `coca`-based threshold behavior and guard vectors.

### Phase D - Pre-implementation gate
- Record contract/test readiness evidence before production edits.

### Phase E - Production implementation
- Project `coca_####` symbols and consume them in WB19 threshold calculations.

### Phase F - Validation and rerun diagnostics
- Run:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Execute fresh 39-hillslope rerun + semantic summaries.

### Phase G - Disposition and handoff
- Publish residual matrix, dual review/verification, and concrete next actions.

## Exit Criteria
- Closure measures `MEASURE-HP219-001..004` are satisfied and evidenced.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contracts/kernel/runtime projection/test updates only.
