# Characterization

Status: executed.

Ran:

- Detached test-first proof:
  - Scaffold commit: `010f4ddf`.
  - Worktree:
    `/tmp/openwepp-cqr-b02-t09-testfirst`.
  - Patch:
    `/tmp/openwepp-cqr-b02-t09-testfirst.patch`.
  - Patch SHA-256:
    `0a0dba0f990268378d0a13a269a5903a429c62d57249e667d4a76995ab2614ec`.
  - Command:
    `CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t09-testfirst-target cargo nextest run --manifest-path /tmp/openwepp-cqr-b02-t09-testfirst/Cargo.toml --test infile_slope_parser_contract --profile quick`.
  - Result: 27/27 passed before production refactor.
- Post-refactor focused oracle:
  - Command:
    `cargo nextest run --test infile_slope_parser_contract --profile quick`.
  - Result: 27/27 passed.

Test additions:

- `slope_parser_error_display_strings_remain_stable`
- `parse_slope_str_reports_top_level_record_count_errors`
- `default_options_match_strict_mode_and_directory_open_errors_are_typed`
- `parser_rejects_nonpositive_counts_and_widths`
- `parser_rejects_nonfinite_geometry_fields`
- `parser_rejects_start_and_monotonic_endpoint_violations`
- `parser_accepts_absolute_distance_mode_without_fractional_mix`
