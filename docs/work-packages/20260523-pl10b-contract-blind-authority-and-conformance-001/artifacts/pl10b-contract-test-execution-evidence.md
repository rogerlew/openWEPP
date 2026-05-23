# PL10b Contract Test Execution Evidence

Status: `complete`
Evidence mode: `Ran`

## Commands

1. `cargo test --test parser_runtime_seam_integration`
2. `cargo test --test parser_runtime_seam_integration -- --ignored`

## Results

### Baseline integration target

- Result: `pass`
- Summary: `25 passed; 0 failed; 5 ignored`
- Interpretation: existing parser/runtime integration suite remains green; PL10b
  conformance gates are isolated as ignored tests.

### PL10b ignored conformance gates

- Result: `fail` (expected for current implementation)
- Summary: `0 passed; 5 failed`
- Failed tests:
  1. `pl10b_contract_conformance_requires_annual_extension_projection_symbols`
  2. `pl10b_contract_conformance_requires_perennial_cutday_indexed_projection`
  3. `pl10b_contract_conformance_requires_perennial_grazing_cycle_payload_projection`
  4. `pl10b_contract_conformance_rejects_invalid_grazing_window_domain`
  5. `pl10b_contract_conformance_rejects_empty_perennial_grazing_cardinality`

## Failure Evidence Highlights

1. Missing annual extension projected symbols:
   - panic: `missing projected runtime symbol jdherb`
2. Missing perennial indexed event symbols:
   - panic: `missing projected runtime symbol pl_decomp_slot_0001_crop_0001_cutday_0001`
   - panic: `missing projected runtime symbol pl_decomp_slot_0001_crop_0001_gday_0001`
3. Missing typed invalid-domain guards:
   - `gday >= gend` case returned `Ok(...)` instead of failing.
   - empty grazing cycle cardinality returned `Ok(...)` instead of failing.

## Conformance Verdict

Current implementation is non-conformant with PL10b transition-control contract
obligations. Reconciliation classification and PL11 dependency patches are
recorded in companion artifacts.
