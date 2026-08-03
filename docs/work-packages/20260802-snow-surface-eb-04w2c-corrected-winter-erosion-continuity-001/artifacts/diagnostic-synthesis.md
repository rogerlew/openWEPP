# Diagnostic Synthesis

Status: technical root cause resolved

Evidence mode: **Ran + Static**

## Finding

EB-04W2B did not expose a sediment mass-conservation failure. It changed the
runoff population and the curvature of valid Wave-1 solutions enough that a
second-order, cellwise trapezoid comparison disagreed with the fourth-order
RK4 detachment march on `61/231` storms. The exact publication identity still
closed on accepted storms.

The prior and corrected populations are not a one-for-one set. The retained
partition has 150 shared clean storms, 25 shared refusals, 31 clean-to-refused
transitions, ten refused-to-clean transitions, five newly qualifying refusals,
and ten newly qualifying clean storms. That forcing-sensitive exchange is the
signature of a truncation-error instrument, not a uniform mass leak.

## Correction

`SC-SED-001` revisions 56–57 separate two predicates and bind the numerical
sub-march boundaries of the second:

- `TOL-SED-007` is the hard `1e-9` telescoping sediment mass identity; and
- `TOL-SED-008` is the independent `5e-3` discretization-consistency check.

The latter now integrates the constitutive rate over nonoverlapping,
same-region, unclamped blocks using Simpson `1/3` pairs and Simpson `3/8`
triples. A trapezoid remains only where a region has exactly one interval. The
quadrature order is commensurate with the RK4 march while remaining independent
of the committed load solution.

No snow equation, erosion constitutive equation, RK4/analytic solver, grid,
coefficient, tolerance, exact mass gate, or refusal publication rule changed.

## Empirical result

The corrected fixture has `231` qualifying storms, `227` clean/depositing
solutions, and four explicit diagnostic refusals (`1.7%`). The remaining
refusals occur on days 376, 715, 1036, and 1810 at low peak runoff
(`2.63e-7` to `3.49e-7 m s^-1`). The `<=20%` prospective population gate now
passes without altering its bound.

This result removes the technical erosion prerequisite exposed by W2B. Formal
handoff remains pending the package-required independent reviews and terminal
verifications.
