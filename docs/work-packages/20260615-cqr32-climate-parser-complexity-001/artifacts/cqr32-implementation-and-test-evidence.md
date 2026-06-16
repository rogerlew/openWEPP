# CQR32 Implementation and Test Evidence

## Implementation

Static:

- Added `ClimateParseError::write_display` as a private helper in
  `crates/openwepp-input-contract/src/parsers/climate.rs`.
- Changed `Display for ClimateParseError::fmt` to delegate to that helper.
- Added exact display-string and `source()` characterization tests in
  `tests/integration/infile_climate_parser_contract.rs`.

## Focused Tests

Ran:

```bash
cargo test --test infile_climate_parser_contract --no-fail-fast
```

Result: exit `0`; `21` passed.

Ran:

```bash
cargo clippy -p openwepp-input-contract --all-targets -- -D warnings
```

Result: exit `0`.

## Metrics

Ran: before and after workspace LCOV plus `cargo crap`.

Result: `ClimateParseError::fmt` CRAP reduced from `240.0` to `1.0`; extracted
helper `ClimateParseError::write_display` is CRAP `15.0`.
