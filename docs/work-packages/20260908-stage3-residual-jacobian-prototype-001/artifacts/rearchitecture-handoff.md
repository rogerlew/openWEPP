# Re-architecture handoff

The next single experiment is an authority-first oracle-resolution study at
`solver_stem_jacobian_tests.rs::select_basin` and the canonical
`evaluate_covered_column` replay seam. It should derive a dimension-aware stem
step from residual conditioning or use bounded higher-precision reference
arithmetic, while retaining canonical binary64 primal probes and the independently
enumerated full row mask.

Early rejection criterion: on the frozen `a05ca090…` corpus, stop if the revised
prospective oracle still admits fewer than 12 of 16 active columns or cannot
bound every affected normalized row without crossing a branch/tie. That work
requires a new contract version because v33 constants are outcome-frozen. Do not
implement the analytic stem tangent, expand to leaf temperatures, or perform
runtime measurements first.
