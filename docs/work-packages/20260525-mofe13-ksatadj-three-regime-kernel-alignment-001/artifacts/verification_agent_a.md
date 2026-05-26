# Verification Agent A

Status: complete
Evidence mode: Ran

Verified:
- Regime tests pass:
  - `wb14_contract_conformance_applies_ksatadj_9001_regime`
  - `wb14_contract_conformance_applies_ksatadj_9002_regime`
  - `wb14_contract_conformance_applies_ksatadj_9003_burn_floor`
  - `wb14_contract_conformance_rejects_active_9001_zero_ksatrec`
- Runtime seam `ksatadj` projection tests pass in `openwepp-hillslope-orchestrator`.
- Required gates passed (`fmt`, `clippy`, `test`, `deny`).
