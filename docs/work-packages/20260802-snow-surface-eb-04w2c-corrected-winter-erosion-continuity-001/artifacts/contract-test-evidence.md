# Contract-Test Evidence

Status: pass

Evidence mode: **Ran**

The corrected contract-derived suite contains seven focused tests:

- curved one/two/three/four/five/six/seven-interval vectors that distinguish
  Simpson from trapezoid and exercise every composition branch;
- synthetic zone, region, and clamp partitioning with matched eligible scale;
- actual one-interval and seven-interval closure runs with asserted returned
  scale;
- explicit inside/outside vectors for the `32 * f64::EPSILON` grid-alignment
  boundary;
- non-grid-aligned coefficient, critical-shear, and analytic-deposition zone
  boundaries;
- an injected committed-load inconsistency that fails exactly as
  `erosion.wave1.flux_closure`; and
- the production hourly fold, proving a flux refusal publishes zero sediment
  plus its counter while publication mass closure remains hard-fail.

The original helper test failed before implementation with the expected
`E0425` missing-helper errors (`logs/05-contract-test-red.log`). Initial review
then established that its affine vectors were trapezoid-equivalent, so that
evidence is historical rather than sufficient closure. The initial corrected
five-test suite passes in log 21. Fresh review exposed a remaining non-grid
straddling interval. Second fresh review required asserted denominator
exclusion and alignment edges; the final seven-test suite passes in log 31,
while the per-cell/boundary EROD16 fixture passes in log 32.
