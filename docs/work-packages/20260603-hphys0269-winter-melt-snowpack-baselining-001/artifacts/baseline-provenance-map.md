# Baseline Provenance Map

Status: completed/HOLD
Evidence mode: static + ran

Static: required source authority was inspected locally.

- Pinned baseline commit: `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- `winter.for` lines 260-367: hourly winter loop calls `radcur`, `hr_tmp`, `stmtim`, frost dispatch, then `snowd`.
- `winter.for` lines 373-464: hourly `hrmlt` capture and daily melt aggregation/post-processing.
- `snowd.for` lines 61-65: `wdayct` settle-day counter increments at hour 1 and resets on snowfall.
- `snowd.for` lines 122-140: cold-day settling uses `setf = exp(-wdayct*2)*0.0416667 + 1`, caps density at `522 kg m^-3`, and reduces snow depth by density ratio.
- `snowd.for` lines 180-230: warm branch calls `melt`, converts positive water melt to snow-depth melt, and caps melt at available snow.
- `snowd.for` lines 240-279: density gate and rain-on-snow holding capacity; snowpack below `350 kg m^-3` stores rain by increasing density and leaves only residual rain as liquid.
- `melt.for` lines 126-180: hourly air temperature in Fahrenheit drives longwave/cloud term and canopy/radiation term.
- `melt.for` lines 223-229: wind/temperature melt term.
- `melt.for` lines 238-262: rain/dewpoint melt term.
- `melt.for` lines 275-301: signed `wmelt`; positive values are bounded to available snow water equivalent, negative values are not clamped.

Static: `/workdir/wepp-forest@924ab16d07edea8b904bcf64d3d7e276fc45d21e` includes commit `03fee4558456535138592630b5dedc4d81ce8d06` (`winter: apply ngtvML/pstvML math fix and close P4 parity lane`), changing the daily negative-melt branch after the pinned baseline. This fix is accepted as superseding authority for the target implementation: the pinned baseline branch is retained only as bug-compatible comparator/counterfactual context, not as physics to reproduce.

Static: openWEPP target mappings after the HPHYS0269 slice.

| Baseline symbol/routine | openWEPP target | Disposition |
|---|---|---|
| `hrrain(hour)` | `snow.hourly.rain_m_####` | Existing input; now retained-rain accounting consumes part of it before liquid forcing. |
| retained rain in `snowd.for` | `snow.hourly.rain_retained_m_####`, `SnowCouplingOutcome::rain_retained` | Implemented in active snow coupling. |
| signed `wmelt` from `melt.for` | `snow.hourly.melt_raw_m_####` | Implemented as trace/publication family. |
| corrected redistributed daily melt | `snow.hourly.melt_m_####`, summed into signed `S` | Implemented as mass-closed daily net-melt redistribution matching the `03fee455` corrected net/sign behavior rather than the pinned baseline bug-compatible branch or raw clamp-to-zero. |
| `snodpt`/`densg` carry state | `snow.runtime_depth_m`, `snow.runtime_density_kg_m3`, `snow.runtime_swe` | Existing carry-state path retained and adjusted for retained rain/net melt closure. |
| `S` | `S` flux | Updated to `melt - snowfall_water_equiv - rain_retained`. |
| residual liquid rain | WB12/WB14 liquid forcing | Updated by subtracting `rain_retained` from hyetograph liquid input. |

Ran: source paths were verified with `find /workdir/wepp-forest_260430_baseline -iname '<routine>'` during package execution.
