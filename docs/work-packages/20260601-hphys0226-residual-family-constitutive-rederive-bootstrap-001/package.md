# 20260601-hphys0226-residual-family-constitutive-rederive-bootstrap-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Carry out HPHYS0225 immediate next actions by bootstrapping constitutive
re-derivation authority for open coupled residual families
(`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`) through a
required Level-4 WB19 behavioral gate for lateral saturated-thickness response.

## Why This Package Exists
HPHYS0225 closed WB19 legacy available-pool max-reconciliation but left
integrated residual families open. HPHYS0225 handoff required immediate
continuation under Correctness Authority Model with constitutive/behavioral
gates rather than parity-tuning.

## Scope
### Included
- Contract-first amendments in `SC-SUBHYD-001` and `SC-WATBAL-001` for a new
  WB19 behavioral law (`INV-SUBHYD-018`).
- New required Level-4 external-authority suite
  `cas_l4_subhyd_lateral_saturated_thickness_response_001` with fixture lock
  and provenance sidecars.
- Contract-derived integration test and fixture-integrity guard wiring.
- Required workspace gates.

### Explicitly Out of Scope
- Full residual-family physics closure implementation.
- Full `unpalatable-rind` 39-hillslope rerun/readjudication.
- Watershed/channel migration scopes.

## Closure Measures (Required)
1. `MEASURE-HP226-001`: canonical contract authority amended for WB19
   saturated-thickness response and indexed.
2. `MEASURE-HP226-002`: required Level-4 suite + fixtures + lock/provenance +
   registry linkage are implemented.
3. `MEASURE-HP226-003`: contract-derived integration test exists and passes.
4. `MEASURE-HP226-004`: fixture-integrity gate includes the new suite root.
5. `MEASURE-HP226-005`: required workspace gates pass (`fmt`, `clippy`,
   `test`, `deny`).
6. `MEASURE-HP226-006`: explicit HOLD disposition and follow-on worker handoff
   are published.

## Deliverables
1. `artifacts/hphys0226-residual-authority-gap-matrix.md`
2. `artifacts/hphys0226-contract-implementation-evidence.md`
3. `artifacts/hphys0226-contract-test-implementation-evidence.md`
4. `artifacts/hphys0226-preimplementation-contract-gate.md`
5. `artifacts/hphys0226-implementation-and-test-evidence.md`
6. `artifacts/hphys0226-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0226_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical `SC-*` authority.
2. Add suite metadata/fixtures and contract-derived tests.
3. Capture pre-implementation contract gate evidence.
4. Run tests/gates and publish disposition.

## Autonomous Execution Intent (Required)
Execute phases end-to-end through disposition without requesting additional user
direction unless hard-blocked.

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
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0225-wb19-available-pool-authority-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0226-residual-family-constitutive-rederive-bootstrap-001/**`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_lateral_saturated_thickness_response_001.md`
- `tests/fixtures/constitutive/cas_l4_subhyd_lateral_saturated_thickness_response_001/*`
- `tests/integration/hphys0226_wb19_lateral_saturated_thickness_response_contract.rs`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `Cargo.toml`

## Phase Plan
### Phase A - Intake freeze and scope targeting
- Freeze HPHYS0226 scope to constitutive gate bootstrap from HPHYS0225 handoff.

### Phase B - Contract + suite authority updates
- Amend `SC-SUBHYD-001`/`SC-WATBAL-001` with WB19 behavioral law.
- Add Level-4 suite doc and registry linkage.

### Phase C - Contract-derived test and fixture integrity
- Add fixture + lock + provenance sidecars.
- Add and run contract-derived integration test.
- Update `auth06` fixture-integrity guard suite lists.

### Phase D - Validation and disposition
- Run required workspace gates.
- Publish review/verification/disposition/handoff artifacts.

## Exit Criteria
- `MEASURE-HP226-001..006` satisfied and evidenced.
- Integrated HPHYS stream remains explicit `HOLD` pending follow-on closure
  packages for open residual families.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contracts/tests/fixtures/docs only; no credentials/network.
