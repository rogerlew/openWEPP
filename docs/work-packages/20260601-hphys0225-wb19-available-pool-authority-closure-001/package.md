# 20260601-hphys0225-wb19-available-pool-authority-closure-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute the HPHYS follow-on from HPHYS0224 by closing WB19 available-pool
authority drift: remove legacy max-reconciliation expansion
(`max(layer_pool, legacy_term)`), ratify layer-derived cap authority in
canonical contracts, and land required Level-4 constitutive guard coverage.

## Why This Package Exists
HPHYS0224 closed over-withdrawal subtraction guards but left one condemned
surface from correctness re-anchoring active in runtime source:
WB19 available-pool max-reconciliation against legacy seam scalars.

## Scope
### Included
- Contract-first amendments in `SC-SUBHYD-001` and `SC-WATBAL-001`.
- External-authority suite + fixture/provenance/registry integration:
  `cas_l4_subhyd_layer_pool_withdrawal_cap_001`.
- Scoped runtime remediation in WB19 lateral/drainage phase code path.
- Contract-derived integration tests and required workspace gates.

### Explicitly Out of Scope
- Full `unpalatable-rind` cohort rerun/readjudication.
- FC/WP constitutive re-derivation (`thetfc/thetdr/cpm/coca`) closure wave.
- Watershed routing/channel migration scopes.

## Closure Measures (Required)
1. `MEASURE-HP225-001`: canonical contract authority amendments are implemented
   and indexed.
2. `MEASURE-HP225-002`: required A3 suite coverage is implemented with fixture
   lock/provenance sidecars and registry wiring.
3. `MEASURE-HP225-003`: pre-implementation contract gate evidence is captured.
4. `MEASURE-HP225-004`: production code removes WB19 legacy max-reconciliation
   available-pool expansion.
5. `MEASURE-HP225-005`: contract-derived tests for HPHYS0225 pass.
6. `MEASURE-HP225-006`: required workspace gates pass (`fmt`, `clippy`,
   `test`, `deny`).

## Deliverables
1. `artifacts/hphys0225-residual-authority-gap-matrix.md`
2. `artifacts/hphys0225-contract-implementation-evidence.md`
3. `artifacts/hphys0225-contract-test-implementation-evidence.md`
4. `artifacts/hphys0225-preimplementation-contract-gate.md`
5. `artifacts/hphys0225-implementation-and-test-evidence.md`
6. `artifacts/hphys0225-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0225_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical `SC-*` authority.
2. Add/modify contract-derived tests + suite metadata/fixtures.
3. Capture pre-implementation contract gate evidence.
4. Apply scoped runtime changes and run gates.

## Autonomous Execution Intent (Required)
Execute phases end-to-end through disposition without user intervention unless
hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/promotion-protocol.md`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0224-cam-wb19-soilwater-authority-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0225-wb19-available-pool-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_layer_pool_withdrawal_cap_001.md`
- `tests/fixtures/constitutive/cas_l4_subhyd_layer_pool_withdrawal_cap_001/*`
- `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `Cargo.toml`

## Phase Plan
### Phase A - Scope freeze and authority targeting
- Freeze HPHYS0225 scope to WB19 available-pool authority only.

### Phase B - Contract + suite updates
- Amend `SC-SUBHYD-001` and `SC-WATBAL-001`.
- Add suite metadata, fixture, lock, provenance, and registry entry.

### Phase C - Contract-derived tests + pre-implementation gate
- Add HPHYS0225 integration tests and fixture-driven cases.
- Record static pre-implementation source capture for prohibited expressions.

### Phase D - Runtime remediation
- Remove lateral/drainage `layer_pool.max(legacy_term)` available-pool logic.

### Phase E - Validation
- Run targeted tests and required workspace gates.

### Phase F - Review, verification, disposition
- Publish dual review, dual verification, and HOLD/next-action handoff.

## Exit Criteria
- `MEASURE-HP225-001..006` satisfied and evidenced.
- Package closes WB19 available-pool authority surface while preserving broader
  integrated HPHYS HOLD posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contracts/tests/kernel code only; no credentials or network.
