# CQR31 Behavior Equivalence

Static:

- Public function signatures and call sites are unchanged.
- WB13 output surface symbols and ordering in `SummaryScalarSurface::from_pairs`
  are unchanged.
- Unit conversions remain meters-to-millimeters with the same factors and
  source symbols.
- Routed-runoff `Q` and `QOFE` publication formulas retain the same operands and
  denominator choices.
- `Wb13DailyWaterBalanceRow::from_surface` remains the row-construction
  authority.
- Typed error constructor `wb13_simout_failure` and existing error detail text
  are preserved.
- Julian-day projection validation remains before runtime-surface reads.
- Month, day-of-month, water-year, and sim-day output-key conversions remain
  after WB13 row construction, matching the original error-ordering posture.

Ran:

- `cargo test -p openwepp-runner publication_wb13`: passed, `31` passed,
  `0` failed.
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr31-runner-output-climate-complexity-001/artifacts/lcov_after.info`:
  passed.

Equivalence judgment: accepted for behavior-preserving private decomposition.
