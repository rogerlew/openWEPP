# CLI/API Equivalence

Evidence label: Static/Ran.

Status: `PASS`

This package preserves CLI/API behavior rather than changing numeric formulas.

Identity evidence:

- argument parse order and error precedence are asserted in
  `run_with_args_preserves_command_error_precedence`;
- `--help` short-circuiting for top-level, common snowbench, and Jennings paths
  is asserted without requiring missing arguments;
- missing-value and unknown-argument error strings are asserted exactly;
- command-specific guard strings are asserted exactly before heavy work starts;
- the Jennings success-path wrapper test proves valid parsed arguments still
  reach `run_jennings_phase_validation` and produce the expected report files;
- existing focused integration contracts still pass for snowbench diagnostic
  confinement and physics-bulk offline behavior.

No floating-point expressions, accumulation order, science constants, report
schemas, default activation selectors, or stdout format strings were changed.
