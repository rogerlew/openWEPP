# CQR32 Parser Equivalence

Static: public parser APIs and parser output structures are unchanged.

## Public Surface

- `parse_climate_file`
- `parse_climate_from_str`
- `ParserMode`
- `CompatibilityOptions`
- `ClimateFile`
- `ClimateModeFlags`
- `ClimateMetadata`
- `ClimateMonthlyStats`
- `ClimateDailyRecord`
- `NoBreakpointDay`
- `BreakpointDay`
- `BreakpointPoint`
- `ClimateParseError`

No signatures, visibility, variants, fields, compatibility controls, parser
grammar, token order, validation order, or runtime/kernel-facing field meanings
were changed.

## Refactor Equivalence

Static: `Display for ClimateParseError::fmt` now delegates to private
`ClimateParseError::write_display`. The helper contains the original match
arms and original `write!` format strings. No float expressions, parser
branches, validation thresholds, or parse output construction were changed.

Ran: `cargo test --test infile_climate_parser_contract --no-fail-fast`.

Result: `21` passed, including all pre-existing climate parser fixture tests
plus new exact display-string characterization for every
`ClimateParseError` variant and `source()` behavior.
