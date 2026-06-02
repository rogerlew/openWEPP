# HPHYS0246 Residual Analysis

Status: completed
Evidence mode: Ran

## Summary
- WB18 aggregate storage discontinuity is closed for the H1/H7/H39 probes.
- Day-1 seed-to-WB18 aggregate deltas now equal `-D` rather than `-(D + seed gap)`.
- The preserved `wb11_soil_water - Σtheta` gap is:
  - H1: `29.401610 mm`
  - H7: `29.790137 mm`
  - H39: `40.410901 mm`
- Day-1 total-soil residual improves by exactly that preserved gap.
- WB19 lateral transfer is now the dominant remaining day-1 storage loss,
  especially H39.

## Day-1 Before/After
| Hillslope | HPHYS0245 Seed-to-WB18 mm | HPHYS0246 Seed-to-WB18 mm | HPHYS0245 Day-1 Total-Soil Delta mm | HPHYS0246 Day-1 Total-Soil Delta mm | Improvement mm | HPHYS0246 Lateral Delta mm |
| --- | --- | --- | --- | --- | --- | --- |
| H1 | -73.646009 | -44.244399 | -113.495625 | -84.094015 | 29.401610 | -19.728001 |
| H7 | -63.642747 | -33.852610 | -117.530510 | -87.740374 | 29.790137 | -37.050899 |
| H39 | -63.391243 | -22.980342 | -166.340456 | -125.929554 | 40.410901 | -79.515092 |

## HPHYS0246 Day-1 Storage Summary
| Hillslope | Seed WB11 mm | Seed Theta mm | Seed Gap mm | Post-WB18 WB11 mm | Post-WB18 Gap mm | WB18 D mm | WB18 Pe mm | WB13 Total-Soil mm | Baseline Total-Soil mm | Day-1 Delta mm |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | 323.346740 | 293.945130 | 29.401610 | 279.102341 | 29.401610 | 44.244399 | 44.244399 | 258.975985 | 343.070000 | -84.094015 |
| H7 | 271.241491 | 241.451354 | 29.790137 | 237.388881 | 29.790137 | 33.852610 | 33.852610 | 199.939626 | 287.680000 | -87.740374 |
| H39 | 363.554235 | 323.143334 | 40.410901 | 340.573894 | 40.410901 | 22.980342 | 22.980342 | 260.660446 | 386.590000 | -125.929554 |

## Phase Residuals
- WB18 day-1 deltas:
  - H1: `-44.244399 mm`
  - H7: `-33.852610 mm`
  - H39: `-22.980342 mm`
- WB17/ET day-1 deltas remain small and common:
  - H1/H7/H39: about `-0.398355 mm`
- WB19 lateral day-1 deltas remain material:
  - H1: `-19.728001 mm`
  - H7: `-37.050899 mm`
  - H39: `-79.515092 mm`
- WB12/storage reconciliation and WB13 publication add no additional day-1
  aggregate discontinuity in these probes.

## Interpretation
- HPHYS0246 resolves the first causal WB18 aggregate-storage defect identified
  by HPHYS0245.
- Remaining H1/H7/H39 residuals should not be chased in WB18 `D`/`Pe` first;
  they are now dominated by WB19 lateral transfer and by broader day-1 initial
  state/baseline surface differences that persist after WB18.
