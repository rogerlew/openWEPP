# EROD17 Contract-Derived Test Matrix

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
| test id | location | contract authority | expected pre-migration posture |
|---|---|---|---|
| `erod17_contract_mshear_dispatch_vector_requires_segment_case_publication` | `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs` | `SC-SED-001` EROD16 route-branch addendum (`mshear` 1..5 dispatch invariant) | fail (missing `mshear` publication) |
| `erod17_contract_deposition_end_vector_requires_xdend_publication` | `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs` | `SC-SED-001` EROD16 deposition-end branch invariant (`depc/depend/depos`) | fail (missing `xdend` publication) |
| `erod17_contract_ndep_followup_vector_requires_post_detachment_deposition_path` | `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs` | `SC-SED-001` EROD16 post-detachment deposition invariant (`ndep != 0` follow-up) | fail (missing `ndep` publication) |
| `erod17_contract_qostar_threshold_vector_requires_upper_boundary_branch_divergence` | `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs` | `SC-SED-001` EROD16 upper-boundary `qostar` threshold branch divergence | fail (missing `dl` publication) |
| `erod17_contract_route_branch_seam_vector_requires_core_publication_family` | `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs` | `SC-SED-001` EROD16 alias continuity set (`mshear`, `xc1`, `xc2`, `du`, `dl`, `xdbeg`, `xdend`, `xdetst`, `ndep`, `lddend`) | fail (route branch publication family absent) |

## Notes
- All five EROD17 vectors are intentionally `#[ignore = "EROD19 route segment migration pending"]`.
- Control suite remains green under default run while migration is pending.

## Ran
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract --no-run`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract`
- `cargo test -p openwepp --test erod14_wave2_multiofe_enrichment_kernel_contract -- --ignored --nocapture`
