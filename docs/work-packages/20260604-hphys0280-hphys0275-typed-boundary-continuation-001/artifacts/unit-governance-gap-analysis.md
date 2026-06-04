# Unit Governance Gap Analysis

Status: completed
Evidence mode: static

Static:
- Closed: direction-degree boundary wrapper and `BoundaryValue` variant for `wind`/`hs{n}_wind`.
- Closed: watershed-prefixed HPHYS0275 climate aliases for precipitation, duration, radiation, temperature, wind speed/direction, breakpoint time, and intensity series.
- Closed: selected snow runtime/trace non-negative depth, density, unit-interval branch, dewpoint, and wind-speed publication.
- Closed: executable unit-registry posture now marks migrated HPHYS0280 aliases `TypedRequired` and adds `DirectionDegrees` domain metadata.
- Remaining: `snow.runtime_settle_day_count` remains scalar count exception.
- Remaining: `snow.hourly.melt_raw_m_{idx4}` remains scalar/follow-up until a signed water-depth wrapper is specified; negative raw melt is retained intentionally.
- Remaining: melt equation trace terms in inches/Fahrenheit/mph remain scalar/follow-up outside this package.
- Remaining: broader soil/WB13 runtime and output metadata alignment remains outside HPHYS0280.

Ran: not-run; gap analysis is static.
