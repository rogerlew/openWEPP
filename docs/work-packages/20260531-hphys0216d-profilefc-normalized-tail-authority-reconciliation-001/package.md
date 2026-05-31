# 20260531-hphys0216d-profilefc-normalized-tail-authority-reconciliation-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0216D contract-first closure for `ProfileFCStore` by reconciling
WB13 FC publication authority to include normalized-profile tail contribution
without reintroducing seed/fallback publication authority.

## Why This Package Exists
HPHYS0216 restored layer-authoritative FC publication
(`Σ(thetfc_i*dg_i)*1000`) but regressed `ProfileFCStore` to `39/39` fails on
the `unpalatable-rind` cohort. HPHYS0216C isolated deterministic profile-static
negative offsets consistent with normalized-tail omission. HPHYS0216D closes
that omission by making tail contribution explicit, typed, and test-guarded.

## Scope
### Included
- Canonical contract amendments for FC layer+tail authority in:
  - `SC-WATBAL-001`
  - `SC-SOIL-001`
  - `SC-PERC-001`
  - `SC-SYSTEM-001`
- Contract-derived tests in:
  - `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs` unit coverage
- Production changes in:
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs`
- Workspace gates and targeted package evidence updates.

### Explicitly Out of Scope
- `Dp` / `latqcc` / `Total-Soil` / `SoilWaterTotal` closure packages.
- Watershed lane process-authority changes.
- Heuristic/parity-only formula substitutions.

## Closure Measures (Required)
1. `MEASURE-HP216D-001`: contracts codify FC publication as
   `Σ(thetfc_i*dg_i)*1000 + wb13_profile_fc_tail_mm` with fail-closed tail
   guards.
2. `MEASURE-HP216D-002`: contract-derived tests hard-fail on missing/invalid
   FC tail symbol and assert layer+tail reconciliation behavior.
3. `MEASURE-HP216D-003`: runtime-input and WB13 publication code paths publish
   and consume explicit FC tail symbol with typed error posture.
4. `MEASURE-HP216D-004`: workspace gates pass (`fmt`, `clippy`, `test`,
   `deny`) and package artifacts/disposition are updated truthfully.

## Deliverables
1. `artifacts/hphys0216d-contract-implementation-evidence.md`
2. `artifacts/hphys0216d-contract-test-implementation-evidence.md`
3. `artifacts/hphys0216d-preimplementation-contract-gate.md`
4. `artifacts/hphys0216d-implementation-and-test-evidence.md`
5. `artifacts/hphys0216d-kernel-profile-compliance-checklist.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/gate-results.md`
8. `artifacts/hphys0216d_disposition.md`
9. `artifacts/worker-handoff.md`
10. `artifacts/review_agent_a.md`
11. `artifacts/review_agent_b.md`
12. `artifacts/verification_agent_a.md`
13. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contracts (`SC-*`) for FC layer+tail authority.
2. Amend/add contract-derived tests that fail on authority regression.
3. Record pre-implementation contract gate evidence.
4. Modify production runtime-input / WB13 publication code.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority is `docs/specifications/science-contracts/contracts/SC-*.md`.
- Baseline comparator provenance:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No provisional/surrogate process-physics substitutions are allowed.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0216c-profilefc-normalized-tail-delta-analysis-001/artifacts/worker-handoff.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0216d-profilefc-normalized-tail-authority-reconciliation-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`

## Phase Plan
### Phase A - Contract amendments
- Add explicit FC layer+tail authority and guard language in canonical contracts.

### Phase B - Contract-derived test amendments
- Add/update tests for `wb13_profile_fc_tail_mm` guard and reconciliation.

### Phase C - Production implementation
- Publish `wb13_profile_fc_tail_mm` from runtime-input layer/profile lineage.
- Consume tail symbol in WB13 FC publication assembly.

### Phase D - Validation and disposition
- Run required gates.
- Update artifacts and publish hold-lift posture.

## Exit Criteria
- Closure measures `MEASURE-HP216D-001..004` are satisfied and evidenced.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test updates with no external interface change.
