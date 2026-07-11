# Obligation-to-test map

Status: complete
Evidence mode: Static and Ran

| Family | Canonical clauses | Exact test-function binding | Pre-implementation state |
| --- | --- | --- | --- |
| A nominal | source grammar, `G-CHN-005` through `G-CHN-013` | `strict_mode_parses_canonical_single_channel_rating_curve_profile`; `exact_numeric_rating_shaped_comment_is_not_reclassified`; WSHED-W5 `watershed_channel_rating_projection_preserves_optional_fields` | PASS |
| B boundary | `INV-CHN-016`, `G-CHN-013` | `final_no_rating_residuals_use_structural_rating_classification`; `multi_channel_extra_rating_is_recognized_only_by_unique_suffix_closure`; `duplicate_rating_after_enabled_branch_remains_generic_extra_input` | PASS |
| C branch | `INV-CHN-016`, `D-CHN-004`, `C-CHN-004` | deleted-only `multi_channel_extra_rating_is_recognized_only_by_unique_suffix_closure`; retained-only `exact_numeric_rating_shaped_comment_is_not_reclassified`; neither `neither_suffix_layout_preserves_the_ordinary_retained_error`; both-layout static arity proof in contract; strict/compat loops in all four | PASS |
| D domain-reject | `G-CHN-001` through `G-CHN-009`, `G-CHN-013` | `strict_mode_rejects_unsupported_datver`; `strict_mode_rejects_ishape_out_of_domain`; `strict_mode_rejects_ienslp_out_of_domain`; `strict_mode_rejects_icntrl_out_of_domain`; `strict_mode_rejects_flgout_out_of_domain`; `strict_mode_rejects_chnn_less_than_chnnbr`; `strict_mode_rejects_rating_curve_rccoef_non_positive`; `strict_mode_rejects_rating_curve_rcoset_negative`; `final_no_rating_residuals_use_structural_rating_classification`; `all_channel_real_domain_families_are_exact`; `integer_enum_and_count_boundaries_are_exact`; `valid_enum_and_nonnegative_boundaries_parse` | PASS |
| E missing-symbol | `G-CHN-005`, `G-CHN-013` | `strict_mode_rejects_missing_rating_curve_line_for_icntrl4`; `strict_mode_rejects_non_numeric_tokens`; `truncated_channel_records_preserve_error_priority`; `integer_and_record_cardinality_errors_are_exact` | PASS |
| F non-finite | `G-CHN-004`, `G-CHN-008`, `G-CHN-009`, `G-CHN-013` | `all_real_token_families_reject_nan_and_infinities` | PASS |
| G conservation/continuity | Section 13 applicability review | reviewed N/A for conservation; WSHED-W5 `watershed_channel_rating_projection_preserves_optional_fields` binds unchanged ordered projection values | REVIEWED N/A |
| H fail-closed | `INV-CHN-016`, `G-CHN-013` | `final_no_rating_residuals_use_structural_rating_classification`; `multi_channel_extra_rating_is_recognized_only_by_unique_suffix_closure`; `exact_numeric_rating_shaped_comment_is_not_reclassified`; `neither_suffix_layout_preserves_the_ordinary_retained_error`; `duplicate_rating_after_enabled_branch_remains_generic_extra_input`; `strict_mode_rejects_extra_trailing_records` | PASS |

Every applicable family passes; G conservation is reviewed N/A because parsing
and frame projection compute no conserved quantity.
