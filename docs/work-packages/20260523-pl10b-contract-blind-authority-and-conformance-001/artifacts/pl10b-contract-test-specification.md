# PL10b Contract Test Specification

Status: `complete`
Evidence mode: `Static`

## Test Authoring Basis

Tests are derived directly from `SC-PLANT-001` transition-control authority:
- `INV-PLANT-011` annual-extension projection completeness
- `INV-PLANT-012` perennial cardinality/index closure
- `INV-PLANT-013` grazing-window ordering
- `INV-PLANT-014` event-day domain validity
- `INV-PLANT-015` hard-fail/no-silent-default posture

## Contract-Derived Gate Tests

All tests are in
`tests/integration/parser_runtime_seam_integration.rs` and intentionally marked
`#[ignore]` for PL10b conformance execution (non-blocking for pre-PL11 code).

| Contract assertion | Test name | Expected conformance behavior |
|---|---|---|
| Annual extension controls are projected with required symbol family coverage | `pl10b_contract_conformance_requires_annual_extension_projection_symbols` | pass only when annual extension symbols are projected |
| Perennial cutting day arrays are projected with deterministic indexed symbols | `pl10b_contract_conformance_requires_perennial_cutday_indexed_projection` | pass only when `cutday_{index}` symbols exist |
| Perennial grazing cycle day/payload arrays are projected with deterministic indexed symbols | `pl10b_contract_conformance_requires_perennial_grazing_cycle_payload_projection` | pass only when `gday/gend/animal/bodywt/area/digest_{index}` symbols exist |
| Invalid grazing windows (`gday >= gend`) are rejected as typed failures | `pl10b_contract_conformance_rejects_invalid_grazing_window_domain` | pass only when projection returns error |
| Empty grazing cycle cardinality for grazing branch is rejected | `pl10b_contract_conformance_rejects_empty_perennial_grazing_cardinality` | pass only when projection returns error |

## Execution Commands

1. Baseline suite (with PL10b conformance gates ignored):
   - `cargo test --test parser_runtime_seam_integration`
2. PL10b conformance gate execution:
   - `cargo test --test parser_runtime_seam_integration -- --ignored`

## Notes

- These tests are completion gates for PL11.
- PL10b requires running them and reconciling failures, not passing them in
  this package.
