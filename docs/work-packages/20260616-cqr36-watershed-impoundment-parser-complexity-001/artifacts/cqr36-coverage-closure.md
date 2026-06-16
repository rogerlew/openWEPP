# CQR36 Coverage Closure

Status: complete.

Target file:
`crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`.

| Metric | Before | After | Result |
| --- | ---: | ---: | --- |
| Line coverage | 624/892, 69.955156950673% | 877/998, 87.875751503006% | improved |
| Function coverage | 23/30, 76.666666666667% | 37/42, 88.095238095238% | improved |
| Unique CRAP rows above 30 | 2 | 0 | closed |
| Target CRAP | 219.61488342725883 | 15.0 | closed |

Characterization added:

- `strict_mode_parses_ids2_drop_spillway_and_open_channel_emergency_branch`
  covers the `ids=2` drop-spillway branch and `ies=1` emergency open-channel
  branch.
- `strict_mode_parses_ids3_drop_spillway_without_optional_branches` covers the
  `ids=3` drop-spillway branch with optional branch codes disabled.
- `watershed_impoundment_parse_error_display_strings_are_stable` covers stable
  display strings for all `WatershedImpoundmentParseError` variants, including
  both explicit and missing `datver` display paths.
- `watershed_impoundment_parse_error_source_is_only_input_open_source` covers
  the wrapped source behavior for `InputOpenError`.

Ran: final LCOV and CRAP reports were regenerated after the characterization
and production decomposition.
