# CAL-07B Science Summary

Evidence class: `Ran + Static`

## Diagnostic question

CAL-07 failed closed because three Alerce Costero daily forcing rows produced
negative VPD under OBL-PLANT-P-013. CAL-07B asked whether those negative
values are present in NASA POWER's published hourly-average paired operands,
or whether they are introduced by combining separate daily temperature extrema
and mean dew point in the daily contract operator.

## Result

The three cases are all classified as
`DAILY_SUMMARY_OPERATOR_MISMATCH`.

All 72 published hourly-average operand pairs reconstruct positive hourly
product VPD:

| Date | Minimum hourly-product VPD (Pa) | Mean hourly-product VPD (Pa) | Reconstructed contract-daily VPD (Pa) | CAL-07 contract-daily VPD (Pa) |
| --- | ---: | ---: | ---: | ---: |
| 2022-07-22 | 17.884672 | 33.808484 | -58.528839 | -58.860502 |
| 2022-09-15 | 30.632806 | 40.573439 | -70.426646 | -70.492437 |
| 2025-09-09 | 27.473561 | 57.875183 | -0.710389 | -1.002242 |

Hourly reconstruction also reproduces the frozen CAL-07 daily operands within
the prespecified serialized-resolution tolerance: Tmin and Tmax are exact, and
mean dew-point residuals are -0.00417 C, -0.00083 C, and -0.00333 C. Daily
signs agree for all three dates.

## Driver decomposition

The negative daily sign is driven by the temperature-extrema summary term, not
by hourly paired operand negatives and not by dew-point nonlinearity.

| Date | Temperature-extrema term (Pa) | Dew-point nonlinearity term (Pa) | Contract minus hourly mean (Pa) |
| --- | ---: | ---: | ---: |
| 2022-07-22 | -99.143594 | 6.806270 | -92.337324 |
| 2022-09-15 | -116.886999 | 5.886914 | -111.000085 |
| 2025-09-09 | -59.955276 | 1.369704 | -58.585572 |

The additive identity closes to floating-point noise; the largest absolute
closure residual is `8.53e-14 Pa`, well inside the frozen `1e-9 Pa`
reconstruction gate.

## Interpretation boundary

This evidence concerns published POWER product/operator combinations. It does
not establish instantaneous site atmospheric state, prove identical daily and
hourly upstream processing, or authorize replacing OBL-PLANT-P-013 with an
hourly aggregation route. The daily source exposes an aggregate source list
while the hourly source reports MERRA2; CAL-07B classifies lineage as
`AGGREGATE_OVERLAP_ONLY`.

CAL-07 therefore remains held. CAL-07B fills the immediate source/operator
diagnostic gap: the observed negatives are not present in paired hourly POWER
products, and the contract-daily negative values arise when the daily summary
operator combines Tmin/Tmax and mean dew point.
