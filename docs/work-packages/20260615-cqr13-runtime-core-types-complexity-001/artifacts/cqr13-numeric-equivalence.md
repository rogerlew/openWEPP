# CQR13 Numeric Equivalence

Status: complete.

Static: CQR13 made no production Rust edits. Therefore no constants, formulas,
unit conversions, float expression grouping, runtime projection meanings, error
IDs, display strings, or output publication formulas changed.

Ran:

- before and after CRAP/LCOV metric gates, exit `0`;
- `cargo test --workspace`, exit `0`.

Conclusion: numeric and behavior equivalence are preserved by absence of
production change.
