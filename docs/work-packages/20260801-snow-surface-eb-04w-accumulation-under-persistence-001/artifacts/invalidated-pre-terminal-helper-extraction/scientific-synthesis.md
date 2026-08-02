# EB-04W Scientific Synthesis

Evidence mode: **Ran + Inference**. All observations remain `DIAGNOSTIC_ONLY`; this is not calibration, efficacy, or promotion.

## Closure

All 16 cells completed. Maximum hourly melt-component closure was `2.027e-17 m`; maximum daily accumulation closure was `7.980e-17 m`; maximum active phase-fraction closure was `1.110e-16`. Modeled wind redistribution was exactly zero by implementation status; this does not establish that physical redistribution at the SNOTEL sites was zero.

## Frozen Operator Results

| Lane | Operator | Median offset (d) | Modeled/observed peak | Diagnostic attribution |
|---|---|---:|---:|---|
| Mica Creek / St. Joe, ID | seasonal_ablation_meltout_date | -35.0 | 0.6487538869893656 | pre-peak mass/forcing boundary dominant |
| Niwot, CO | seasonal_peak_depth_date | -46.5 | 0.47191210270260325 | pre-peak mass/forcing boundary dominant |
| Niwot, CO | seasonal_peak_swe_date | -31.0 | 0.4855566494959412 | pre-peak mass/forcing boundary dominant |
| Paradise, WA | seasonal_ablation_meltout_date | -37.0 | 0.5114815355286422 | pre-peak mass/forcing boundary dominant |
| Snowbird, UT | seasonal_peak_swe_date | -44.5 | 0.3767796654285547 | pre-peak mass/forcing boundary dominant |

## Interpretation Boundary

All five controls enter the modeled ablation/peak transition with a large mass deficit: median modeled-to-observed peak ratios range from about `0.38` to `0.65`. That is strong diagnostic evidence that the chronology failures are already established before post-peak ablation, so an ablation-physics amendment is not the first admissible remedy. The ledger cannot distinguish precipitation representativeness, gauge undercatch, rain/snow phase error, and physical wind redistribution without independent forcing/process observations.

The four CoE columns are signed empirical melt-depth contributions. They help localize when the current formula removes or retains snow, but they are not separately observed energy fluxes. `bmelt` and `cmelt` mix temperature, cloud, wind, dewpoint, and canopy effects and cannot be treated as unique sensible-heat measurements. A pre-peak modeled mass deficit routes first to precipitation representativeness, phase, gauge undercatch, and unmodeled redistribution at the openWEPP/wepppy boundary; it is not authority to tune snow physics.
