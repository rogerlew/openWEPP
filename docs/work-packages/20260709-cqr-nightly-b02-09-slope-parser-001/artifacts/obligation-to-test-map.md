# Obligation-to-Test Map

No `SC-*` process-physics obligation is altered. The package is
behavior-preserving slope parser CQR.

| Obligation / behavior | Test binding |
|---|---|
| Strict canonical and Peridot slope profiles parse with stable datver source, OFE count, elevation, and distance mode. | `strict_mode_accepts_canonical_datver_profile`; `strict_mode_accepts_peridot_2023_3_profile` |
| Strict mode rejects missing datver; compatibility mode accepts legacy missing-datver input. | `strict_mode_rejects_missing_datver_header`; `compatibility_mode_accepts_missing_datver_header` |
| Shared-geometry compatibility form remains compatibility-only. | `strict_mode_rejects_shared_geometry_multi_ofe_form`; `compatibility_mode_accepts_shared_geometry_multi_ofe_form` |
| Compatibility endpoint and cross-OFE boundary behavior remains permissive where previously permissive. | `compatibility_mode_accepts_near_endpoint_terminal_distance`; `compatibility_mode_accepts_cross_ofe_boundary_discontinuity` |
| Datver thresholds and unsupported-datver typed errors remain stable. | `strict_mode_rejects_non_canonical_datver`; `compatibility_mode_accepts_legacy_threshold_datver`; `compatibility_mode_rejects_datver_below_threshold` |
| Distance-mode, endpoint, cross-OFE, nslpts, token, record-count, file-missing, file-open, count, width, and non-finite guards remain typed fail-closed errors. | Existing parser rejection tests plus `default_options_match_strict_mode_and_directory_open_errors_are_typed`; `parser_rejects_nonpositive_counts_and_widths`; `parser_rejects_nonfinite_geometry_fields`; `parser_rejects_start_and_monotonic_endpoint_violations`; `parser_accepts_absolute_distance_mode_without_fractional_mix`; `parse_slope_str_reports_top_level_record_count_errors` |
| Every public `SlopeParserError` display string remains stable. | `slope_parser_error_display_strings_remain_stable` |
