# Numeric Equivalence

Evidence label: Static/Ran.

Status: `EXECUTED`

Production code changed: no.

Numeric equivalence basis:

- This package added only `#[cfg(test)]` characterization code.
- No watershed detachment/deposition formulas, thresholds, coefficients,
  floating-point operation ordering, validation guards, or output surfaces were
  modified.
- Focused tests assert existing table interpolation, typed validation failures,
  exact WS20 fall velocity and WS22 shear-distribution values, exact WS23
  case-4 closure fluxes/width, exact WS23 low-shear iterative-loop
  fluxes/width, exact WS23 sum/potential/final-flux helper outputs, exact WS26
  expanding-width detachment/depth/width outputs, exact WS26 midlayer and
  expanding terminal/cap behavior, and exact WS26 low-width-shear
  class-fraction allocations.

Command evidence:

- `cargo test -p openwepp-watershed-orchestrator --lib wshedimpl -- --nocapture`
  - PASS, `16` passed.
- `cargo llvm-cov -p openwepp-watershed-orchestrator --lib --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted.lcov`
  - PASS, `82` lib tests passed.

Conclusion: behavior identity is preserved because there is no production
behavior edit, and the added assertions characterize existing numeric/guard
behavior.
