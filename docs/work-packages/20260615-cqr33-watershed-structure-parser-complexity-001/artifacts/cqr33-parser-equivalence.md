# CQR33 Parser Equivalence

Static: public parser APIs and parser output structures are unchanged.

## Public Surface

- `parse_watershed_structure_from_path`
- `parse_watershed_structure_from_str`
- `ParseMode`
- `DatverSource`
- `WatershedStructureParseOptions`
- `WatershedStructureWarningCode`
- `WatershedStructureWarning`
- `WatershedStructureRow`
- `WatershedStructureSummary`
- `WatershedStructureFile`
- `WatershedStructureParseError`

No signatures, visibility, variants, fields, compatibility controls, parser
grammar, token order, validation order, error IDs, or runtime/kernel-facing
field meanings were changed.

## Refactor Equivalence

Static: `Display for WatershedStructureParseError::fmt` now delegates to
private `WatershedStructureParseError::write_display`. The helper contains the
original match arms and original `write!` format strings. No parser branches,
validation thresholds, or parse output construction were changed.

Ran: `cargo test --test infile_watershed_structure_parser_contract --no-fail-fast`.

Result: `20` passed, including all pre-existing watershed-structure parser
fixture tests plus new exact display-string characterization for every
`WatershedStructureParseError` variant and `source()` behavior.
