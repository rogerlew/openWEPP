# Verification Agent A

Status: complete
Evidence mode: ran
Date: 2026-05-26

## Ran
- `rg -n "erod17_contract_" tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract -- --ignored --nocapture`

## Result
- Five EROD17 vectors are present and ignored by default.
- Default targeted suite passes.
- Ignored-vector suite fails as expected for pre-migration gaps.
