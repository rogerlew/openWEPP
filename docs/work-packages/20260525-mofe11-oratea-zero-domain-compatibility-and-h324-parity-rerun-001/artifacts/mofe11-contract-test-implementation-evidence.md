# MOFE11 Contract-Test Implementation Evidence

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Added runtime projection contract-derived tests:
  - `runtime_inputs::tests::management_runtime_projection_allows_zero_oratea_orater_for_legacy_no_decay`
  - `runtime_inputs::tests::management_runtime_projection_rejects_negative_oratea_projection_field`
- Added decomposition guard contract-derived test:
  - `tests::decomposition_boundary_rejects_negative_oratea_with_typed_failure`
- Added parser/runtime seam decomposition no-decay behavior test:
  - `pl17_contract_conformance_scheduler_preserves_seed_masses_when_decomposition_constants_are_zero`

Ran:
- Tests were authored before production guard edits and exercised in the
  pre-implementation gate.
