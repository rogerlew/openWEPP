# CQR05 Function Length After

Evidence: Static.

Target file line count after refactor:

- `hydrology_phase_erod14.rs`: `1001` lines

Post-refactor function spans:

| Function | Start | End | Lines |
| --- | ---: | ---: | ---: |
| `From<Erod14RawInputs>::from` | 66 | 87 | 22 |
| `run_erod14_wave2` | 88 | 118 | 31 |
| `erod14_class_count` | 119 | 154 | 36 |
| `erod14_class_count_violation` | 155 | 163 | 9 |
| `erod14_inputs` | 164 | 171 | 8 |
| `erod14_load_raw_inputs` | 172 | 246 | 75 |
| `erod14_validate_raw_inputs` | 247 | 296 | 50 |
| `erod14_validate_flow_inputs` | 297 | 367 | 71 |
| `erod14_validate_case` | 368 | 382 | 15 |
| `erod14_case_number` | 383 | 408 | 26 |
| `erod14_case_matches` | 409 | 425 | 17 |
| `erod14_case_is_zero` | 426 | 429 | 4 |
| `erod14_case_one_matches` | 430 | 435 | 6 |
| `erod14_case_two_matches` | 436 | 441 | 6 |
| `erod14_case_three_matches` | 442 | 448 | 7 |
| `erod14_case_four_matches` | 449 | 455 | 7 |
| `erod14_case_violation` | 456 | 464 | 9 |
| `erod14_theta` | 465 | 486 | 22 |
| `erod14_load_class_state` | 487 | 510 | 24 |
| `erod14_push_class_state` | 511 | 552 | 42 |
| `erod14_zero_outflow_updates` | 553 | 567 | 15 |
| `erod14_pkro` | 568 | 580 | 13 |
| `erod14_project_initial_gend` | 581 | 605 | 25 |
| `erod14_project_class_gend` | 606 | 655 | 50 |
| `erod14_phi` | 656 | 667 | 12 |
| `erod14_attenuation_factor` | 668 | 702 | 35 |
| `erod14_reproportion_to_ldbot` | 703 | 744 | 42 |
| `erod14_reproportion_iteration` | 745 | 792 | 48 |
| `erod14_validate_sumg_and_caps` | 793 | 818 | 26 |
| `erod14_update_transport_fractions` | 819 | 845 | 27 |
| `erod14_enrichment_ratio` | 846 | 870 | 25 |
| `erod14_final_updates` | 871 | 915 | 45 |
| `erod14_push_base_updates` | 916 | 954 | 39 |
| `erod14_push_class_updates` | 955 | 1001 | 47 |

Disposition:

- The crate-visible entry point dropped from 643 lines to 31 lines.
- Longest helper is 75 lines.
