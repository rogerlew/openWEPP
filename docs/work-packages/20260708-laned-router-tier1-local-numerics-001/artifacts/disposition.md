# Disposition

Status: `EXECUTED-HOLD-APPROXIMATION-ENVELOPE`

Final disposition after implementation, review fixes, and closure gates:

- Landed rev-47 contract and code for analytic local numerics.
- Fixed an H2637 active `NonFiniteState` regression by using closed-form pure
  skin roots and an explicit pre-step branch selection in the skin
  discontinuity gap.
- Fixed review-reported active vegetation non-finite handling by failing closed
  instead of zeroing an active vegetation term.
- Added the missing rev-47 dry/zero-slope, exact Hirsch pow, branch-gap,
  active-vegetation failure, and dust-floor test vectors.
- Focused unit, D10B, H2637 active, named fidelity delta comparison, timing,
  perf, full nextest, clippy, fmt, deny, diff-check, and scoped contract unit
  gates pass.
- Remaining hold is limited to the unimplemented `Re^0.45` approximation.
