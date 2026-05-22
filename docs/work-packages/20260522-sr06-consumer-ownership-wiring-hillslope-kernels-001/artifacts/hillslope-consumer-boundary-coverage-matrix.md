# Hillslope Consumer Boundary Coverage Matrix (SR06)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Matrix maps SR06 consumer-boundary obligations to executable tests and typed failure outcomes.

Ran:
- All listed tests executed and passed under `cargo test --workspace`.

| coverage_id | path_type | boundary scope | test anchor | expected result | status |
|---|---|---|---|---|---|
| `SR06-HS-CONSUMER-001` | happy path | phase->adapter routing | `consumer_adapter_boundaries_receive_runtime_seam_symbols` | each phase receives the canonical adapter identity | `pass` |
| `SR06-HS-CONSUMER-002` | happy path | slope+soil symbol wiring | `consumer_adapter_boundaries_receive_runtime_seam_symbols` | activated required symbols exist on each phase boundary | `pass` |
| `SR06-HS-CONSUMER-FAIL-001` | typed failure | soil boundary guard | `missing_soil_consumer_symbol_fails_with_typed_missing_input_status` | fails at `normalization` with `MissingRequiredInput` + `HS-CONSUMER-E-001` | `pass` |
| `SR06-HS-CONSUMER-FAIL-002` | typed failure | runoff boundary guard | `missing_runoff_slope_symbol_fails_at_runoff_reconciliation_boundary` | fails at `runoff_reconciliation` with `MissingRequiredInput` + `HS-CONSUMER-E-001` | `pass` |
| `SR06-HS-CONSUMER-UNIT-001` | unit mapping | phase ownership map | `consumer_adapter_mapping_matches_phase_contract` | deterministic phase->adapter map preserved | `pass` |
| `SR06-HS-CONSUMER-UNIT-002` | unit compatibility | SR05 closure preservation guard | `required_consumer_symbols_are_empty_without_slope_or_soil_families` | no forced soil/slope requirement when families are absent | `pass` |
