# EROD17 Pre-Migration Failure Baseline

Status: complete
Evidence mode: ran
Date: 2026-05-26

## Static
- Objective: confirm EROD17 route-branch vectors fail on current runtime before
  EROD18/EROD19 migration packages.

## Ran
- Command:
  - `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract -- --ignored --nocapture`
- Result: failed as expected (`0 passed; 5 failed`).
- Observed blocking failures:
  - `erod17_contract_mshear_dispatch_vector_requires_segment_case_publication`
    - panic: `missing expected state symbol mshear`
  - `erod17_contract_deposition_end_vector_requires_xdend_publication`
    - panic: `missing expected state symbol xdend`
  - `erod17_contract_ndep_followup_vector_requires_post_detachment_deposition_path`
    - panic: `missing expected state symbol ndep`
  - `erod17_contract_qostar_threshold_vector_requires_upper_boundary_branch_divergence`
    - panic: `missing expected state symbol dl`
  - `erod17_contract_route_branch_seam_vector_requires_core_publication_family`
    - panic: `missing expected state symbol mshear`
- Control run:
  - `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`
  - Result: pass (`6 passed; 0 failed; 5 ignored`).
