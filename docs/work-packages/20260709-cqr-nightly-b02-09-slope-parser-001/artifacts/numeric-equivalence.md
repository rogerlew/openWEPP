# Numeric / API Equivalence

Static:

- The target module parses slope input files and validates typed guard
  invariants. It does not perform process-physics calculations or conservation
  output publication.
- Numeric thresholds, tolerances, guard IDs, parse ordering, fallback ordering,
  and validation decisions are unchanged.
- Floating-point comparisons still use the same `approx_eq` helper, values, and
  short-circuit order.
- Public APIs, enum variants, file grammar, data structures, and typed errors
  are unchanged.

Ran:

- Test-first detached proof passed on scaffold source with only the test diff.
- Focused post-refactor suite:
  `cargo nextest run --test infile_slope_parser_contract --profile quick`;
  27/27 passed.

Disposition: behavior identity is preserved for parser success cases, typed
errors, display strings, guard IDs, distance modes, compatibility fallback, and
strict/compatibility datver handling.
