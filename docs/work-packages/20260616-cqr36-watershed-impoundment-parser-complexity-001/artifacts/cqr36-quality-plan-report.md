# CQR36 Quality Plan Report

Status: complete.

Quality objective: close the CQR36 rank 30 target in
`crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs` without
changing parser API, `IMP-E-*` / `IMP-W-*` IDs, strict/compatibility behavior,
branch arity, domain guards, parsed output shapes, or downstream watershed
runtime semantics.

Target identity:

- Before metrics target: `parse_impoundment`, line `655`, CC `73.0`,
  coverage `69.81132075471697%`, CRAP `219.61488342725883`.
- Same-file pre-existing high row: `WatershedImpoundmentParseError::fmt`,
  line `387`, CC `12.0`, coverage `0.0%`, CRAP `156.0`.
- After metrics target: `parse_impoundment`, line `701`, CC `15.0`,
  coverage `100.0%`, CRAP `15.0`.
- After metrics high-row closure: no unique target-file CRAP row above `30`.

Quality plan:

1. Capture before line counts, LCOV, CRAP, and suppression census.
2. Add characterization for unexercised `.imp` branch variants before
   production decomposition.
3. Extract private helpers from `parse_impoundment` only, preserving parse
   order, context labels, error variants, branch comments, vector consumption,
   and record assembly.
4. Add formatter/source characterization for the same target file so the
   pre-existing display row also closes under current metrics.
5. Re-run LCOV/CRAP and full closeout gates.

Outcome:

- The scoped target and all newly extracted helpers are below CRAP `30`.
- Target-file unique rows above CRAP `30` went from `2` to `0`.
- Target-file line coverage increased from `624/892` (`69.955156950673%`) to
  `877/998` (`87.875751503006%`).
- Target-file function coverage increased from `23/30` (`76.666666666667%`) to
  `37/42` (`88.095238095238%`).

Warning:

- `cargo crap` reported 126 LCOV source-map warnings in before and after runs.
  The target file was represented in both LCOV reports.
