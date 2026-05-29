# 20260529-hparity01-hillslope-wat-always-fail-gap-mapping-001

## Status
- state: hold
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPARITY01 to establish contract-authoritative provenance, ownership,
and measurement baselines for the 12 hillslope `H.wat` always-fail columns
observed in `unpalatable-rind` semantic parity evidence.

## Why This Package Exists
Ran evidence from `/tmp/unpalatable_parity_20260529T192707Z` shows row-key
alignment but `39/39` semantic failures for the same 12 columns:
`Dp`, `Ep`, `Es`, `ProfileDepth`, `ProfileFCStore`, `ProfilePorosityCap`,
`ProfileWPStore`, `RM`, `Snow-Water`, `SoilWaterTotal`, `Total-Soil`,
`latqcc`. Implementation closure should not proceed as ad hoc edits; we need
canonical contract lineage and staged closure gates first.

## Scope
### Included
- Contract amendments for the 12 failing columns across canonical SC contracts
  with baseline symbol continuity and alias mapping.
- Gap-matrix evidence linking each failing output column to:
  contract rows, baseline routines/symbols, runtime owner(s), and guard IDs.
- Contract-derived test scaffolding that encodes column-lineage and invariant
  expectations for follow-on implementation packages.
- Baseline measurement artifact updates capturing fail counts and top deltas
  from current `unpalatable-rind` parity run.

### Explicitly Out of Scope
- Production kernel math changes in runtime/orchestrator code.
- Silent fallback/heuristic proxy equations for any failing column family.
- Hold-lift disposition claims for parity closure.

## Closure Measures (Required)
1. `MEASURE-HP01-001`: `artifacts/hparity01-always-fail-column-gap-matrix.md`
   covers all 12 failing columns with contract IDs, baseline symbol names,
   runtime writers, and current residual fingerprints.
2. `MEASURE-HP01-002`: canonical contracts encode explicit alias continuity for
   `Total-Soil` / `Total-Soil Water` / `SoilWaterTotal` and equivalent column
   name surfaces where applicable.
3. `MEASURE-HP01-003`: baseline residual metrics are recorded for all 12
   columns (`fail_count`, `max_abs_diff`) from the current run root.
4. `MEASURE-HP01-004`: contract-derived tests compile and are queued with
   explicit expected-preimplementation behavior.

## Deliverables
1. `artifacts/hparity01-always-fail-column-gap-matrix.md`
2. `artifacts/hparity01-contract-implementation-evidence.md`
3. `artifacts/hparity01-contract-test-implementation-evidence.md`
4. `artifacts/hparity01-preimplementation-contract-gate.md`
5. `artifacts/hparity01-implementation-and-test-evidence.md`
6. `artifacts/hparity01-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hparity01_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index amendments for the 12-column
   lineage and invariants.
2. Implement contract-derived test scaffolding for column lineage/invariants.
3. Record pre-implementation contract-gate evidence before production edits.
4. Limit production edits to parity scaffolding/tests (no physics closure in
   this package).

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy migration authority defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Do not introduce provisional or surrogate process-physics equations in
  production paths.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/hillslope_semantic_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hparity01-hillslope-wat-always-fail-gap-mapping-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/hparity01_hillslope_wat_lineage_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm authorization from direct user request for staged always-fail-column
  reconciliation planning.
- Freeze HPARITY01 as contract/gap-map package only.

### Phase B - Contract/spec authority updates
- Amend canonical SC contracts for each failing column family with explicit
  symbol lineage and invariants.
- Update index rows to capture HPARITY01 gap-map scope.

### Phase C - Contract-derived test scaffolding
- Add tests for lineage surfaces and required column-invariant seams.
- Ensure test scaffolding is executable and truthfully classified.

### Phase D - Pre-implementation contract gate
- Record gate evidence proving contracts and tests are in place before
  production math edits.

### Phase E - Validation and disposition
- Execute scoped validation (`cargo fmt --check`, targeted tests for authored
  scaffolding).
- Complete review/verification/disposition artifacts and worker handoff for
  HPARITY02/03/04 implementation packages.

## Exit Criteria
- All 12 failing columns are contract-mapped and test-scaffolded.
- Closure measures `MEASURE-HP01-001..004` are satisfied and evidenced.
- Package disposition clearly enumerates implementation-ready next actions.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: contract/test/gap-map updates only; no auth/network surfaces.
