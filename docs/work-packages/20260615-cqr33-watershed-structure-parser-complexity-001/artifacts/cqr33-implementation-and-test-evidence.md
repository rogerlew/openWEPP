# CQR33 Implementation and Test Evidence

## Implementation

Static:

- Added `WatershedStructureParseError::write_display` as a private helper in
  `crates/openwepp-input-contract/src/parsers/watershed_structure.rs`.
- Changed `Display for WatershedStructureParseError::fmt` to delegate to that
  helper.
- Added exact display-string and `source()` characterization tests in
  `tests/integration/infile_watershed_structure_parser_contract.rs`.

## Focused Tests

Ran:

```bash
cargo test --test infile_watershed_structure_parser_contract --no-fail-fast
```

Result: exit `0`; `20` passed.

Ran:

```bash
cargo clippy -p openwepp-input-contract --all-targets -- -D warnings
```

Result: exit `0`.

## Metrics

Ran: before and after workspace LCOV plus `cargo crap`.

Result: `WatershedStructureParseError::fmt` CRAP reduced from `240.0` to
`1.0`; extracted helper `WatershedStructureParseError::write_display` is CRAP
`15.0`.
