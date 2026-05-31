# 20260531-hphys0222-wb19-solwpv-branch-authority-dp-regression-closure-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Close the WB19 branch-authority defect where `fcdep/unsdep` mutation was
applied for `solwpv != 2006` instead of baseline-authoritative
`solwpv < 2006`, and incorporate this law into external-authority governance
as a required hard-fail constitutive suite.

## Why This Package Exists
HPHYS0221 improved coupled families but retained a `Dp` regression and left a
branch-authority mismatch in WB19 lateral coupling. Baseline static provenance
(`watbal.for`) shows `fcdep` mutation is guarded by `solwpv.lt.2006`.

## Scope
### Included
- Contract-first authority corrections in:
  - `SC-WATBAL-001`
  - `SC-SUBHYD-001`
- Contract-derived branch-law tests.
- Production WB19 branch fix in `run_lateral_transfer`.
- External-authority governance integration:
  - suite spec,
  - fixture lock/provenance sidecars,
  - registry wiring in required/hard-fail lane,
  - integration gate test.
- Workspace validation gates (`fmt`, `clippy`, `test`, `deny`).

### Explicitly Out of Scope
- 39-hillslope rerun and semantic parity readjudication.
- Watershed/channel remediations unrelated to WB19 branch authority.
- Comparator-threshold policy changes.

## Closure Measures (Required)
1. `MEASURE-HP222-001`: canonical contracts explicitly constrain WB19
   `fcdep/unsdep` mutation to `solwpv < 2006`.
2. `MEASURE-HP222-002`: contract-derived tests fail on pre-fix behavior and
   pass post-fix for `solwpv = 2005/2006/9002`.
3. `MEASURE-HP222-003`: production WB19 implementation applies corrected branch
   law without relaxing typed guards.
4. `MEASURE-HP222-004`: external-authority governance includes an active
   required/hard-fail suite for this branch law with fixture lock/provenance.
5. `MEASURE-HP222-005`: workspace gates pass.

## Deliverables
1. `artifacts/hphys0222-contract-implementation-evidence.md`
2. `artifacts/hphys0222-contract-test-implementation-evidence.md`
3. `artifacts/hphys0222-preimplementation-contract-gate.md`
4. `artifacts/hphys0222-implementation-and-test-evidence.md`
5. `artifacts/hphys0222-kernel-profile-compliance-checklist.md`
6. `artifacts/hphys0222-residual-gap-matrix.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0222_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Sequencing (Required)
1. Amend canonical `SC-*` contracts.
2. Add contract-derived tests and external-authority suite scaffolding.
3. Record pre-implementation contract gate evidence (including expected failing
   vector on pre-fix behavior).
4. Modify production code and run closure gates.

## Autonomous Execution Intent (Required)
Execute package phases end-to-end without additional user direction unless
hard-blocked.

## Truthfulness Labeling Requirement
Artifacts must label evidence as `Static:` vs `Ran:`.

## Provenance and Authority Posture
- Canonical contract authority:
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Baseline migration comparator/provenance:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- External-authority suite schema/registry authority:
  `docs/specifications/external-authority/*`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0221-wb19-water-yield-fcdep-coupling-implementation-001/artifacts/worker-handoff.md`

## Intended Write Set
- `Cargo.toml`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_solwpv_fcdep_branch_001.md`
- `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
- `tests/integration/auth07_fc_authority_cohort_contract.rs`
- `tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs`
- `tests/integration/hphys0221_wb19_water_yield_fcdep_coupling_contract.rs`
- `tests/fixtures/constitutive/cas_l4_subhyd_solwpv_fcdep_branch_001/*`
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0222-wb19-solwpv-branch-authority-dp-regression-closure-001/**`

## Phase Plan
### Phase A - Contract authority
- Amend SC contract text and revision history for corrected mutation scope.

### Phase B - Contract-derived tests + external-auth scaffolding
- Add suite spec/fixture/registry wiring and integration tests.
- Capture pre-fix failing vector evidence.

### Phase C - Production implementation
- Correct WB19 lateral mutation gate to `solwpv < 2006`.

### Phase D - Validation + disposition
- Run full workspace gates and package artifact closeout.

## Exit Criteria
- `MEASURE-HP222-001..005` are evidenced in artifacts.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: deterministic kernel logic/tests/docs only.
