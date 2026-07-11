# Obligation-to-test map

Status: complete
Evidence mode: Static and Ran

| Family | Current binding | Status |
| --- | --- | --- |
| A nominal | `strict_mode_parses_valid_sprinkler_fixture`, `strict_mode_parses_valid_furrow_fixture` | bound |
| B boundary | `datver_policy_covers_epsilon_thresholds_and_both_irrigation_domains`, `initial_line3_records_cover_missing_arity_token_and_range_errors`, `sprinkler_event_covers_arities_tokens_ranges_and_event_ordering`, `furrow_surges_cover_arity_tokens_range_and_twenty_row_boundary`, `furrow_rows_cover_mode_arities_tokens_and_each_range_constraint` | bound |
| C branch | strict/compat datver, nozzle, furrow arity, and ordering tests | bound |
| D domain reject | header, date/OFE, sprinkler, furrow, ordering, and finite-domain rejection tests | bound |
| E missing-symbol | `empty_and_incomplete_preambles_have_typed_structural_errors`, line3/event arity and token cases, and `event_stream_closure_errors_cover_sprinkler_and_furrow_successors` | bound |
| F non-finite | `every_real_field_rejects_nan_and_infinities_in_both_modes` | bound, green |
| C compatibility provenance | existing warning/default/arity/ordering tests | bound |
| G conservation / continuity | reviewed `N/A`: immutable parser records carry no conservation identity | bound by applicability review |
| H fail-closed | all B-F rejection tests assert `expect_err`, exact variants/IDs, and therefore no partial typed file; compatibility defaults are tested only in authorized C branches | bound |

Additional parser determinism: `comments_blank_lines_and_whitespace_are_normalized_with_physical_line_numbers`, repeated equality assertions in the datver suite, and `event_ofe_expectation_cycles_across_multiple_sprinkler_events`.
