# CQR33 Disposition

Status: accepted-with-warnings

## Decision

Accept CQR33 package for package commit/push. The scoped target and extracted
helper are below the CRAP `30` closure threshold, target-file coverage is above
the ADR-0021 glue-tier threshold, all required closure gates passed, and no
parser public surface or runtime-facing semantics changed.

## Evidence Summary

- Before target: `WatershedStructureParseError::fmt`, CRAP `240.0`.
- After target: `WatershedStructureParseError::fmt`, CRAP `1.0`.
- Extracted helper: `WatershedStructureParseError::write_display`, CRAP
  `15.0`.
- Focused characterization: `cargo test --test
  infile_watershed_structure_parser_contract --no-fail-fast`, exit `0`, `20`
  passed.
- Full workspace tests and workspace clippy passed.
- `cargo deny check` passed.
- Markdown lint passed.
- `git diff --check` passed.

## Warnings

- `cargo crap` reported the established `126` LCOV source-map warnings.
- Same-file out-of-scope parser row remains above CRAP `30`.
