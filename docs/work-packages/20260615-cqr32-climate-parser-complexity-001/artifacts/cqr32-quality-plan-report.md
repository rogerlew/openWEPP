# CQR32 Quality Plan Report

Static: CQR32 targets one quality dimension: CRAP/cyclomatic-complexity
closure for the climate parser error display surface.

## Scope

- Target file:
  `crates/openwepp-input-contract/src/parsers/climate.rs`
- Original tracker row: rank 26, CRAP `240`, CC `15`, coverage `0%`
- Live target from before metrics: `ClimateParseError::fmt`
- Protected surfaces: public parser APIs, grammar, token order,
  compatibility controls, typed errors, error variants, field names, units,
  parser-output shape, and runtime/kernel-facing meanings.

## Plan

1. Capture before LCOV/CRAP and line counts.
2. Add focused characterization for every `ClimateParseError` display branch
   and `source()` behavior before production refactor.
3. Extract the display match body into a private helper while preserving exact
   formatting strings and `write!` calls.
4. Re-run focused tests, after LCOV/CRAP, and full package gates.

## Closure Notes

Ran: focused characterization and workspace LCOV show the scoped target
closed. The package does not attempt to close out-of-scope same-file parser
rows above CRAP `30`; they remain backlog for later ranked CQR rows or future
packages.
