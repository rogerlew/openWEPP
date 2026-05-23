# WB16 Downstream Coupling Readiness Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Coupling Payload Emitted by WB16 Runtime
Produced state symbols at closure diagnostics:
- `peakro`
- `watdur`
- `wb16_peak_method_branch`
- `wb16_tstar`
- `wb16_qpstar`
- `wb16_vstar`

Required coupled antecedents consumed from upstream runtime path:
- `Q` (WB14 runoff reconciliation output)
- `I` (WB15 interception output)
- `irrigation.runtime_rate_m_per_s` (IRRIG10 runtime output)
- hyetograph symbols (`timem_####`, `intsty_####`, `ninten`/`nbrkpt`)

## Readiness Validation
- `tests/integration/wb16_peak_runoff_kernel_contract.rs` asserts presence and
  finite/domain-valid WB16 payload symbols in nominal vectors.
- Contract amendments landed in downstream intake contracts:
  - `SC-HYDRAULICS-001`
  - `SC-ROUTE-001`
  - `SC-SED-001`
- Coupling posture is hard-fail on missing/non-finite/domain-invalid payload
  symbols (`HKERNEL-WB16-PEAK-E-001..003`) and does not synthesize fallback
  peak-flow values.
