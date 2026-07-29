# CAL-07C Science Summary

Evidence class: `Ran`

## What changed scientifically

CAL-07 failed closed because the Alerce daily-summary VPD operand was negative
on three dates. CAL-07B showed that, on those three dates, paired hourly POWER
products were positive while the daily-summary operator went negative.

CAL-07C tested the bounded correction implied by that diagnostic: for Alerce
only, derive daily VPD as the arithmetic mean of the exact 24 LST hourly
paired-product VPD values. Beza stayed on the original CAL-07 daily-summary
operator.

## Source-admission result

The full Alerce POWER hourly inventory spans 1,666 complete days and 39,984
hourly rows per operand. CAL-07C retained 349 negative hourly paired-product
components as signed source evidence. None were clipped. Their daily signed
means were all nonnegative, so the Alerce daily GSI input-domain blocker was
lifted for this package-local bounded execution.

This does not replace `SC-PLANT-001` OBL-PLANT-P-013. Production still rejects
negative daily-summary VPD without clamping.

## Execution result

CAL-07C produced 123,284 daily kernel rows: 37 frozen CAL-04B members, two
Southern Hemisphere sites, and 1,666 days per site/member. The independent
validator reconstructed source VPD and proved the executor output consumed the
admitted forcing exactly.

Key results:

- Alerce forcing-domain blocker: `LIFTED_FOR_BOUNDED_EXECUTION`.
- Daily foliar mass closure: `SUPPORTED`, max independently reconstructed
  residual `2.082e-17 kg m-2`.
- Producer-state cyclic phase invariance: `SUPPORTED`.
- Real downstream consumer ordering/common-state lineage: `SUPPORTED`.
- Relative seasonal shape: `BOUNDED`.
- Persistent evergreen realization: `BOUNDED`.
- Deciduous transition chronology: `CONTRADICTED`.
- Signed-latitude calendar and seasonal direction: `CONTRADICTED`.
- Absolute canopy amplitude, quantitative evergreen floor, phase-transformed
  real-consumer chronology, and litter/decomposition consequences:
  `NOT_EVALUATED`.

Shape medians were positive for both sites and scoring years:

- Beza 2024: median Pearson `0.737415052`;
- Beza 2025: median Pearson `0.681574141`;
- Alerce 2024: median Pearson `0.469413933`; and
- Alerce 2025: median Pearson `0.491646450`.

The transition chronology contradiction is material. Only 11 of 148 Beza
member/event rows found a same-direction modeled GSI 0.5 crossing, and the
available residuals had median `-52.479 days`.

## Disposition

CAL-07C closes the immediate CAL-07 forcing-domain blocker only for a bounded
research execution. Order 7 does not advance to complete because transition
chronology remains contradicted and several non-forcing cells remain
not-evaluated.
