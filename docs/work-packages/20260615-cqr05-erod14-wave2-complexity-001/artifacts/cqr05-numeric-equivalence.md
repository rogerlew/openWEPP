# CQR05 Numeric Equivalence

Evidence: Static + Ran.

Refactor type:

- Behavior-preserving private helper extraction.

Preserved items:

- EROD14 formulas and constants.
- Arithmetic expression grouping in transport, reproportioning, concentration,
  enrichment, and writeback calculations.
- Guard families and typed error construction.
- Symbol names and writeback field order.
- Case predicates and thresholds.
- Public crate-visible entry point.

Implementation note:

- The previous intermediate `fidel` vector was not retained because its only
  use was immediate multiplication by `theta` to produce `ftheta`. The refactor
  computes the same `fidel_value * theta` at the same class-load point and does
  not change downstream operands.

Ran evidence:

- Focused EROD14 contract tests passed before and after the refactor.
- Full workspace tests passed after the refactor.

Comparator disposition:

- No legacy comparator delta review was run for this mechanical code-quality
  package. The package did not authorize formula, threshold, or physical-model
  changes.
