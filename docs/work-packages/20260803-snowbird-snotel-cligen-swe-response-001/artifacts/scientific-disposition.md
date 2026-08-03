# Snowbird SNOTEL-Conditioned CLIGEN SWE Response

Status: `COMPLETE / MEDIATED RESPONSE IMPROVES BUT REMAINS INSUFFICIENT / NO CORRECTION`

Ran: five baseline-physics Snowbird cells over 1990–2024 through the real
snowbench consumer. Four complete observed-mode `.prn` inputs drove the same
CLIGEN binary, Alta station, interpolation, and RNG burn. Missing SNOTEL values
fell back to exact-date original fixture precipitation or temperature.

## Response

| Variant | Median peak SWE ratio | Peak-date offset (d) | Melt-out offset (d) | Effective input ratio | Storage ratio | Pre-peak loss (m) |
|---|---:|---:|---:|---:|---:|---:|
| Original fixture | 0.390 | -44.5 | -20.0 | 0.697 | 0.211 | 0.459 |
| CLIGEN control | 0.313 | -50.0 | -27.0 | 0.647 | 0.102 | 0.515 |
| SNOTEL precipitation | 0.324 | -50.0 | -27.5 | 0.667 | 0.099 | 0.539 |
| SNOTEL temperature | 0.330 | -45.0 | -25.5 | 0.676 | 0.169 | 0.515 |
| SNOTEL precipitation + temperature | 0.360 | -45.0 | -22.0 | 0.699 | 0.162 | 0.527 |

Relative to the CLIGEN control, the SNOTEL-P input intervention raises median peak
SWE ratio only `0.011`, leaves the peak 50 days early, and makes median melt-out
half a day earlier. The SNOTEL-T input intervention raises peak ratio `0.017`, moves
the peak five days later, and moves melt-out 1.5 days later. Their combination
raises peak ratio `0.047`, moves peak timing five days later, and delays
melt-out five days, but still reaches only `0.360` of observed median peak SWE
and remains 22 days early at melt-out.

The observed-mode CLIGEN control itself differs materially from the unchanged
fixture: peak ratio falls `0.077`, peak timing becomes 5.5 days earlier, and
melt-out becomes seven days earlier. This control effect includes `.prn`
quantization and CLIGEN regeneration of storm shape, radiation, wind, and
dewpoint. Therefore SNOTEL effects are interpreted primarily against the
CLIGEN control, not by naively comparing each variant only with the original
fixture.

SNOTEL precipitation was eligible and assigned before quantization on 5,164 of
12,784 days (`40.4%`) and paired Tmax/Tmin on 8,103 days (`63.4%`). Terminal-v2
changes 2,672 precipitation `.prn` fields, 7,483 Tmax fields, and 6,310 Tmin
fields after quantization. The P-input bundle consequently changes realized
`.cli` precipitation/duration/tp/ip on 2,672/4,568/4,334/2,336 days. The T-input
bundle changes Tmax/Tmin/ip/dewpoint on 7,483/6,310/129/6,818 days; joint input
changes produce 2,349 `ip` differences. All other days use
explicit original-fixture fallback. Sparse observation coverage limits the
intervention and may be nonrandom.

Terminal-v2 binds the experiment tool and all dynamically imported W1/EB-04W/
EB-04R analysis and harness tools, corrects the original-reference header to
1990/35, and reproduces the same five response rows. The controlling freeze,
receipt, and result SHA-256 values are `dc11e735...e695`,
`3d0cb61d...c498`, and `5517a168...7121`.

## What We Learned

The joint SNOTEL input intervention moves both accumulation and chronology in
the right direction relative to a like-for-like CLIGEN control. The T-input
bundle has more chronology response than the P-input bundle, but CLIGEN-mediated
secondary-field changes prevent attribution to temperature or precipitation
alone. It does not
resolve the Snowbird discrepancy. Peak SWE remains far below observed and melt
remains about three weeks early.

The experiment also exposes a large climate-generation-path sensitivity:
passing the original daily precipitation and temperature through observed-mode
CLIGEN does not replay the original WEPPcloud climate response. Secondary
weather generation and input quantization materially affect snow results. That
means a future forcing correction cannot be justified by swapping only daily
precipitation and temperature without controlling radiation, humidity,
wind/storm structure, and generator provenance.

All five cells close below `1.3e-15 m`. The SNOTEL record is calibration
evidence, not independent validation. No production climate correction,
provider admission, transferability, or snow-model promotion is claimed.
