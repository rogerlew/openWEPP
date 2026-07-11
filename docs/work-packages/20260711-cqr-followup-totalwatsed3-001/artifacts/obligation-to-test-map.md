# Obligation-to-test map

Status: PASS
Evidence mode: Ran

| Family | Binding | Result |
| --- | --- | --- |
| A nominal | `totalwatsed3_cli_uses_pass_runvol_and_outlet_lateral_flow`; `totalwatsed3_cli_reads_openwepp_per_hillslope_pass_and_wat_surfaces`; `two_day_water_storage_and_sediment_oracle_rejects_wrong_aliases` | PASS |
| B boundary | `aggregated_area_overflow_and_zero_runoff_paths_fail_or_zero_exactly`; `every_wat_float_family_rejects_nonfinite_and_area_rejects_nonpositive`; `two_day_water_storage_and_sediment_oracle_rejects_wrong_aliases` | PASS |
| C branch | `alternate_column_names_and_optional_defaults_preserve_output_identity`; `optional_columns_cover_all_null_mixed_null_nonfinite_and_missing_partitions`; `optional_join_partial_coverage_uses_last_duplicate_wat_key_area` | PASS |
| D domain reject | `every_pass_float_guard_rejects_nonfinite_and_negative_nonnegative_fields`; `every_wat_float_family_rejects_nonfinite_and_area_rejects_nonpositive`; `aggregated_area_overflow_and_zero_runoff_paths_fail_or_zero_exactly` | PASS |
| E missing/type/null | `direct_writer_closes_required_empty_existing_read_and_write_paths`; `required_schema_type_and_null_failures_have_exact_priority`; `totalwatsed3_cli_rejects_missing_explicit_optional_inputs` | PASS |
| F non-finite | `every_pass_float_guard_rejects_nonfinite_and_negative_nonnegative_fields`; `every_wat_float_family_rejects_nonfinite_and_area_rejects_nonpositive`; `optional_columns_cover_all_null_mixed_null_nonfinite_and_missing_partitions` | PASS |
| G conservation/publication | `two_day_water_storage_and_sediment_oracle_rejects_wrong_aliases`; `optional_soil_element_outputs_are_independently_area_reconstructed`; `optional_join_partial_coverage_uses_last_duplicate_wat_key_area` | PASS |
| H fail closed/order | `two_day_water_storage_and_sediment_oracle_rejects_wrong_aliases`; `totalwatsed3_cli_help_and_argument_errors_preserve_cli_contract`; `totalwatsed3_error_codes_display_and_sources_are_stable`; `direct_writer_closes_required_empty_existing_read_and_write_paths` | PASS |

The independent oracle does not derive expected PASS runoff from emitted
output. It sums literal PASS `runvol` cells, independently sums WAT
`depth*Area`, applies the per-hillslope maximum-OFE lateral selector, and uses
the exact WAT date/OFE area lookup for optional soil/element fields. Wrong
columns, denominators, joins, or OFE scopes yield deliberately different
answers.

The optional join intentionally omits `sim_day_index`, and duplicate WAT
`DateOfeKey` entries retain current ordered overwrite behavior. Those are
characterized current semantics, not new authorization to change the key or
deduplicate rows. `for_batch` is the sole reviewed per-function exclusion; its
exact disposition is in `coverage-after.md`.
