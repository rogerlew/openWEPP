# Contract Amendment Evidence

Status: implemented

Evidence mode: **Static + Ran**

`SC-SED-001` revision 56 was amended before the initial production code.
Revision 57 incorporates the independent-review corrections directly into the
binding invariant, algorithm, guard map, and test-vector surfaces. Revisions 58–60
adds the mandatory algorithm-state, branch/guard, constants/provenance,
unit-governance, and calibration-not-applicable profile surfaces after the
first terminal verifiers found those governance sections incomplete. Together
they:

- name exact publication mass closure as `TOL-SED-007`;
- name the independent discretization check as `TOL-SED-008`;
- retain the `5e-3` relative discretization tolerance and `1e-9` mass
  tolerance;
- require nonoverlapping Simpson `1/3` and `3/8` blocks over contiguous,
  unclamped same-region intervals within one recorded numerical sub-march,
  with trapezoid only for a single interval;
- prevent blocks from crossing coefficient, critical-shear, RK4/analytic,
  region, or clamp boundaries and bind the eligible denominator population;
- preserve typed refusal, zero sediment contribution for a refused quantum,
  and the surfaced refusal count.
- classify every touched value as a fixed numerical/governance constant and
  bind `CALIBRATION_NOT_APPLICABLE`; no user coefficient or fitted parameter
  is introduced.

The original contract-derived quadrature test was authored and observed red with
the expected missing-helper compile error. See
`pre-implementation-contract-gate.md` and `logs/05-contract-test-red.log`.

Initial review found that the original affine vectors did not distinguish
Simpson from trapezoid and that sub-march boundaries were not recorded.
Revision 57 and the seven corrected tests close those findings. Revisions 58–60 are
documentation authority only and does not change the validated runtime/test
identities. The amendment does not install new process physics. It corrects the numerical
order of an independent diagnostic so that it can evaluate the existing
fourth-order/analytic solution without conflating truncation disagreement with
mass nonclosure.
