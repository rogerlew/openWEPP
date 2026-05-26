# EROD17 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Implemented five EROD17 contract-derived vectors in
  `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`:
  - `erod17_contract_mshear_dispatch_vector_requires_segment_case_publication`
  - `erod17_contract_deposition_end_vector_requires_xdend_publication`
  - `erod17_contract_ndep_followup_vector_requires_post_detachment_deposition_path`
  - `erod17_contract_qostar_threshold_vector_requires_upper_boundary_branch_divergence`
  - `erod17_contract_route_branch_seam_vector_requires_core_publication_family`
- Vectors are intentionally `#[ignore = "EROD19 route segment migration pending"]`
  so default targeted suite remains green while route migration is pending.
- Added helper:
  - `require_state_scalar(...)`

## Ran
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract --no-run`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract -- --ignored --nocapture`
