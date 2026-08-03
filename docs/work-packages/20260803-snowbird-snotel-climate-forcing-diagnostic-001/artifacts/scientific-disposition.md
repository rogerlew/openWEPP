# Snowbird SNOTEL Climate-Forcing Diagnostic

Status: `COMPLETE / BOUNDED DRY-FORCING EVIDENCE / NO CORRECTION`

Ran: the checksum-bound `barred-pro` `p8.cli` climate was compared with the
normalized official NRCS AWDB record for Snowbird station `766:UT:SNTL` over
the common period 1989-08-24 through 2024-12-31. SNOTEL cumulative
precipitation was differenced only across consecutive dates within the same
water year; gaps and water-year resets were not bridged.

## Primary Results

| Population | Precip n | Fixture total (mm) | SNOTEL total (mm) | Ratio | Daily r | Wet-day agreement |
|---|---:|---:|---:|---:|---:|---:|
| All common guarded increments | 5,180 | 21,857.0 | 25,765.8 | 0.848 | 0.903 | 87.1% |
| Snow season, Oct-Jun | 4,125 | 18,700.9 | 22,321.5 | 0.838 | 0.904 | 86.9% |
| Wet winter, Nov-Mar | 2,337 | 10,885.1 | 12,971.8 | 0.839 | 0.913 | 87.8% |

The eight intervals satisfying the stricter exact October 1 and September 30
boundary rule have a mean fixture/SNOTEL precipitation ratio of `0.850` and a
median of `0.857`; none is below `0.75`. These are matched boundary differences
covering October 2 through September 30, not complete water-year totals.

The fixture reproduces storm chronology well but misses some precipitation and
the upper event tail. In wet winter, positive-event medians are `7.9 mm`
fixture versus `5.08 mm` SNOTEL, while the 90th percentiles are `19.1` versus
`22.86 mm` and the 99th percentiles are `37.04` versus `50.88 mm`. The fixture
has 1,120 wet days versus 1,298 SNOTEL wet days on the guarded common records.

| Wet-winter temperature | n | Fixture mean (C) | SNOTEL mean (C) | Bias (C) | MAE (C) | Daily r |
|---|---:|---:|---:|---:|---:|---:|
| Tmax | 3,309 | 1.905 | 1.288 | +0.617 | 1.076 | 0.975 |
| Tmin | 3,308 | -6.512 | -6.136 | -0.375 | 1.226 | 0.949 |

## What We Learned

The retained climate is materially dry relative to the colocated gauge. Across
the guarded common record it supplies about `84%` of SNOTEL precipitation, a
deficit near `16%`. Its daily precipitation chronology is strong (`r=0.91` in
wet winter), while it has fewer wet days and a lighter extreme-event tail.

Temperature is also not grossly decorrelated. Wet-winter maxima are about
`0.62 C` warmer and minima about `0.38 C` colder than SNOTEL. The warmer daily
maxima could contribute to early melt or phase differences, but this diagnostic
does not quantify that snow response. NRCS documents a known SNOTEL air-
temperature sensor bias, so these values are comparison evidence rather than
truth without uncertainty.

The evidence strengthens a bounded forcing-representativeness finding: the
fixture underestimates measured precipitation and misses part of the upper
storm tail. Within the admitted guarded samples, it does not support a roughly
`0.5` fixture/SNOTEL gauge ratio as the sole explanation for Snowbird's snow
deficit. Nonrandom missingness and unresolved gauge-catch/scale uncertainty
mean it cannot reject a broader twofold true-precipitation mechanism. The
remaining gap can include gauge/catch and scale mismatch, phase partition,
redistribution, canopy processes, retention/loss, observation footprint, and
snow physics. No forcing correction, gauge-undercatch attribution, provider
admission, or snow-model improvement is claimed.

Only 5,180 daily precipitation increments pass the consecutive-date guard, and
only eight intervals meet the exact boundary rule. Missing SNOTEL records may
make these subsets nonrandom; that limitation is retained explicitly.
