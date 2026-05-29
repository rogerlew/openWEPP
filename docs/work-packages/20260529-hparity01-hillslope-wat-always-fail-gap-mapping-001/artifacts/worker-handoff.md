# HPARITY01 Worker Handoff

Status: completed  
Evidence mode: Static + Ran

## Immediate Next Actions
1. Execute `HPARITY02` contract-first closure for profile-capacity lineage:
   - replace placeholder `ProfilePorosityCap` derivation with
     baseline-authoritative profile porosity-capacity computation,
   - reconcile `ProfileDepth`/`ProfileFCStore`/`ProfileWPStore` publication
     parity.
2. Execute `HPARITY03` closure for ET + rain/snow publication family:
   - reconcile `Ep`, `Es`, `RM`, `Snow-Water` runtime publication semantics to
     baseline-authoritative lineage.
3. Execute `HPARITY04` closure for percolation/subsurface/aggregate family:
   - reconcile `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`.
4. Execute `HPARITY05` end-to-end rerun closeout:
   - rerun unpalatable-rind hillslope cohort,
   - require zero always-fail columns before hold lift.

## Handoff Inputs
- Gap matrix:
  `artifacts/hparity01-always-fail-column-gap-matrix.md`
- Baseline semantic evidence root:
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/semantic`
- Contract-derived scaffold tests:
  `tests/integration/hparity01_hillslope_wat_lineage_contract.rs`

## Guardrail Reminder
- Do not close parity columns with surrogate/proxy formulas.
- Migrate baseline-authoritative lineage only; keep typed fail-closed posture.
