# CAL-07F Science Summary

Evidence class: `Ran + Static`

## Product audit

The retained daily products and transition records are internally coherent.
All 24 product/year/direction/threshold transitions have a same-direction daily
smooth-curve crossing. Twenty-three select the nominal transition date;
`gcc_90` 2024 falling T25 differs by 4.625 days but remains inside its reported
confidence interval.

The `gcc_mean`/`gcc_90` distinction is scientifically material for falling
dates, but it does not change the model diagnosis. Penalized member ranks are
identical across products, and the top-quartile overlap is 100%.

## Model chronology

After excluding wrong-season recovery crossings:

- all 37 members cross falling T10/T25/T50 in both years;
- falling median residuals range from about 38 to 92 days early;
- all members cross rising T10, with medians about 45 days late in 2024 and
  83 days late in 2025;
- only 1/37 members crosses rising T25 in 2024 and 8/37 in 2025;
- no member crosses rising T50 in either year or product; and
- no member is complete across all 12 transitions.

The best joint member is `GSI-4831`. It hits 1/12 confidence intervals under
`gcc_mean` and 0/12 under `gcc_90`. Its penalized mean absolute residual is
59.12 and 65.87 days, respectively.

The absolute `GSI21=0.5` analogy is weaker: only 22 of 148 product-specific
member/event comparisons have a seasonal crossing for each product, with one
`gcc_mean` confidence-interval hit and none under `gcc_90`.

Independent Reviewer B also removed the seasonal-window restriction as a
permissive sensitivity. That makes all 37 members crossing-complete, but still
leaves zero joint uncertainty or direction-coherence passers; the best
member's rising median is about 77 days early because wrong-season recovery
crossings are then admitted. The no-calibration decision is therefore not
created by the seasonal midpoint exclusion.

## Calibration decision

Only two of six required criteria pass:

- operator independence: `PASS`;
- crossing sufficiency: `FAIL`;
- uncertainty fit: `FAIL`;
- direction coherence: `FAIL`;
- parameter plausibility: `FAIL`; and
- empirical role: `PASS` mechanically, but internal-only.

CAL-07D's parameter counterfactuals do not supply a coherent repair. No
constraint-removal scenario retains all 148 event/member matches while keeping
both rising and falling median residuals inside ±21 days.

## Interpretation

The mismatch is robust to the observation-product choice. The current GSI
formulation produces a shorter and seasonally displaced active-canopy period
for this tropical dry-forest lane: leaf-off is early and leaf-on development is
late or absent at higher relative levels.

The admitted evidence cannot determine whether the missing representation is
rainfall response, rooting-zone or stored-water access, species/forest-type
composition, a different seasonal cue, or their interaction. It does show that
another threshold calibration against the same two provisional years is not a
defensible way to resolve the mismatch.
