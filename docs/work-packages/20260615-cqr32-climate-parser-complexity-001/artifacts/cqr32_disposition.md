# CQR32 Disposition

Status: accepted-with-warnings

## Decision

Accept CQR32 package for package commit/push. The scoped target and extracted
helper are below the CRAP `30` closure threshold, all required closure gates
passed, and no parser public surface or runtime-facing semantics changed.

## Evidence Summary

- Before target: `ClimateParseError::fmt`, CRAP `240.0`.
- After target: `ClimateParseError::fmt`, CRAP `1.0`.
- Extracted helper: `ClimateParseError::write_display`, CRAP `15.0`.
- Focused characterization: `cargo test --test infile_climate_parser_contract
  --no-fail-fast`, exit `0`, `21` passed.
- Full workspace tests and workspace clippy passed.
- `cargo deny check` passed.
- Markdown lint passed.
- `git diff --check` passed.

## Warnings

- `cargo crap` reported the established `126` LCOV source-map warnings.
- Same-file out-of-scope parser rows remain above CRAP `30`.
- Target-file line coverage improved to `81.034482758621%` but remains below
  the ADR-0021 glue-tier threshold.
