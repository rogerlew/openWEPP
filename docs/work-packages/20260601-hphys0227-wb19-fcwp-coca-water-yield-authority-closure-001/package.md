# 20260601-hphys0227-wb19-fcwp-coca-water-yield-authority-closure-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Carry out HPHYS0226 immediate next actions by closing WB19 FC/WP + COCA
water-yield coupling authority:

1. `avfca` must use FC theta lineage (`thetfc_####`), and
2. WB19 lateral execution must enforce per-layer FC-store consistency
   (`wb18_perc_fc_#### = (thetfc_####-thetdr_####)*dg_####`).

## Why This Package Exists
HPHYS0226 established a required behavioral WB19 gate but left integrated
residual-family constitutive closure open. The first follow-on target from
HPHYS0226 handoff is FC/WP + COCA coupling authority and FC/WP consistency in
the WB19 `watyld/fcdep` branch.

## Scope
### Included
- Contract-first amendments in `SC-SUBHYD-001` and `SC-WATBAL-001` for WB19
  FC/WP + COCA coupling authority.
- New required Level-4 external-authority suite
  `cas_l4_subhyd_watyld_fcwp_consistency_001` with fixture lock/provenance.
- Production WB19 kernel update to enforce FC/WP consistency and authoritative
  `avfca` lineage.
- Contract-derived integration tests plus fixture-integrity guard (`auth06`)
  wiring.
- Required workspace gates.

### Explicitly Out of Scope
- Full integrated residual-family closeout across all open `H.wat` columns.
- Full `unpalatable-rind` cohort rerun/readjudication.
- Watershed/channel migration scopes.

## Closure Measures (Required)
1. `MEASURE-HP227-001`: canonical contract authority amended for WB19 FC/WP +
   COCA coupling and indexed.
2. `MEASURE-HP227-002`: required Level-4 suite + fixtures + lock/provenance +
   registry linkage are implemented.
3. `MEASURE-HP227-003`: production WB19 kernel enforces FC-store consistency
   and uses `thetfc_####` lineage for `avfca`.
4. `MEASURE-HP227-004`: contract-derived integration tests pass.
5. `MEASURE-HP227-005`: fixture-integrity guard includes the new suite root.
6. `MEASURE-HP227-006`: workspace gates pass (`fmt`, `clippy`, `test`, `deny`).
7. `MEASURE-HP227-007`: explicit HOLD disposition and follow-on handoff are
   published.

## Deliverables
1. `artifacts/hphys0227-residual-authority-gap-matrix.md`
2. `artifacts/hphys0227-contract-implementation-evidence.md`
3. `artifacts/hphys0227-contract-test-implementation-evidence.md`
4. `artifacts/hphys0227-preimplementation-contract-gate.md`
5. `artifacts/hphys0227-implementation-and-test-evidence.md`
6. `artifacts/hphys0227-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0227_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical `SC-*` authority.
2. Add suite metadata/fixtures and contract-derived tests.
3. Capture pre-implementation contract gate evidence.
4. Implement production WB19 kernel updates.
5. Run tests/gates and publish disposition.

## Autonomous Execution Intent (Required)
Execute phases end-to-end through disposition without requesting additional
user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/promotion-protocol.md`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0226-residual-family-constitutive-rederive-bootstrap-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0227-wb19-fcwp-coca-water-yield-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_watyld_fcwp_consistency_001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/fixtures/constitutive/cas_l4_subhyd_watyld_fcwp_consistency_001/*`
- `tests/integration/hphys0227_wb19_fcwp_coca_watyld_authority_contract.rs`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs`
- `tests/integration/hphys0219_wb19_coca_threshold_contract.rs`
- `tests/integration/hphys0221_wb19_water_yield_fcdep_coupling_contract.rs`
- `tests/integration/hphys0224_wb19_withdrawal_soilwater_cap_contract.rs`
- `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs`
- `tests/integration/hphys0226_wb19_lateral_saturated_thickness_response_contract.rs`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
- `Cargo.toml`

## Phase Plan
### Phase A - Intake freeze and scope targeting
- Freeze HPHYS0227 scope to HPHYS0226 immediate-next target:
  FC/WP + COCA coupling authority.

### Phase B - Contract and suite authority updates
- Amend `SC-SUBHYD-001` and `SC-WATBAL-001`.
- Add Level-4 suite doc and registry linkage.

### Phase C - Contract-derived tests and fixture integrity
- Add fixture + lock + provenance sidecars.
- Add and run HPHYS0227 contract test.
- Update `auth06` fixture-integrity suite lists.

### Phase D - Production implementation and validation
- Implement WB19 kernel authority corrections.
- Run required workspace gates.
- Publish review/verification/disposition/handoff artifacts.

## Exit Criteria
- `MEASURE-HP227-001..007` satisfied and evidenced.
- Integrated HPHYS stream remains explicit `HOLD` pending follow-on closure
  packages for remaining residual families.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contracts/tests/fixtures/docs/code only; no credentials/network.
