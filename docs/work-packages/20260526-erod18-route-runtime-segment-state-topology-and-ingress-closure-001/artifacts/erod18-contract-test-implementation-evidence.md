# EROD18 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
Added/updated integration coverage in:
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`

Coverage added:
- `erod18_contract_route_topology_rejects_missing_required_symbol`
- `erod18_contract_route_topology_rejects_non_finite_required_symbol`
- `erod18_contract_route_topology_rejects_domain_violation`
- `erod17_contract_route_branch_seam_vector_requires_core_publication_family`
  moved from ignored to active seam assertion.

Coverage retained as ignored (EROD19 scope):
- `erod17_contract_mshear_dispatch_vector_requires_segment_case_publication`
- `erod17_contract_deposition_end_vector_requires_xdend_publication`
- `erod17_contract_ndep_followup_vector_requires_post_detachment_deposition_path`
- `erod17_contract_qostar_threshold_vector_requires_upper_boundary_branch_divergence`

## Ran
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract -- --ignored --nocapture`
