# Obligation-to-test map

Status: complete
Evidence mode: Static and Ran

| Family | Canonical clauses | Exact test-function binding | Status |
| --- | --- | --- | --- |
| A nominal | `INV-CHN-013`, `G-CHN-003`, `G-CHN-007` | `strict_mode_parses_canonical_payload`; `wshedw11d_strict_zero_count_closes_after_three_records_and_retains_dtchr`; `compatibility_retains_raw_count_then_normalizes_closed_record` | PASS |
| B boundary | `G-CHN-003`, `G-CHN-005`, `G-CHN-007` | `strict_domain_boundaries_and_both_dtchr_limits_are_closed`; `strict_enforces_topology_closure_and_compatibility_clamps_count`; `raw_negative_count_is_retained_then_normalized_only_in_compatibility` | PASS |
| C branch | `G-CHN-001`, `G-CHN-007`, `G-CHN-010`, `G-CHN-012` | `ipeak_le_two_returns_not_applicable_without_file_dependency`; `strict_mode_missing_required_file_is_chn_e_009`; `compatibility_missing_file_defaults_with_chn_w_001`; `strict_mode_non_enoent_open_error_is_chn_e_000`; `compatibility_non_enoent_open_error_collapses_with_chn_w_002`; `compatibility_normalization_covers_extremes_capping_and_no_warning_paths`; WSHED-W5 `chaninp_raw_cardinality_is_observable_while_frame_consumes_normalized_count` | PASS |
| D domain-reject | `G-CHN-004` through `G-CHN-008` | `strict_mode_rejects_invalid_ichout_domain`; `strict_rejects_dtchr_out_of_range_and_compatibility_normalizes`; `strict_rejects_negative_cbase_and_compatibility_clamps`; `strict_rejects_unknown_ichnum_and_compatibility_retains_with_w005`; `raw_negative_count_is_retained_then_normalized_only_in_compatibility` | PASS |
| E missing-symbol | `INV-CHN-013`, `G-CHN-003` | `record_shape_failures_cover_missing_short_extra_and_conditional_line4`; `strict_mode_enforces_line4_arity`; `line4_wrong_arity_precedes_invalid_id_token_in_both_modes`; `strict_enforces_topology_closure_and_compatibility_clamps_count` | PASS |
| F non-finite | `G-CHN-002`, `G-CHN-005`, `G-CHN-006` | `token_parse_and_nonfinite_failures_are_field_specific` covers `NaN`, `+inf`, and `-inf` independently for both `dtchr` and `cbase` | PASS |
| G conservation/continuity | Section 13 reviewed applicability | No conserved quantity or continuous state exists in parser/projection scope. | REVIEWED N/A |
| H fail-closed | `INV-CHN-013`, `G-CHN-003`, `G-CHN-011` | `line4_wrong_arity_precedes_invalid_id_token_in_both_modes`; `strict_enforces_topology_closure_and_compatibility_clamps_count`; `custom_invalid_timestep_options_surface_typed_closure_errors`; `prefixed_variant_is_rejected_in_both_modes` | PASS |

All applicable families pass; G is explicitly N/A because this parser and its
topology projection do not compute a conserved quantity or continuous state.
