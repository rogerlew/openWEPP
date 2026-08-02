# EB-04W Scientific Synthesis

Evidence mode: **Ran + Inference**. All observations remain `DIAGNOSTIC_ONLY`; this is not calibration, efficacy, or promotion.

## Closure

All 16 cells completed. Maximum uncapped component-sum closure was `2.017e-17 m`; maximum applied component-plus-cap closure was `2.027e-17 m`; maximum daily applied-to-retained-raw-melt closure was `6.072e-17 m`; maximum daily accumulation closure was `7.980e-17 m`; maximum active phase-fraction closure was `1.110e-16`; maximum phase-amount closure was `1.214e-17 m`; maximum physical-depth/SWE closure was `6.072e-18 m`; maximum pre-observed-peak mass closure was `2.998e-15 m`; and maximum trace-to-WAT SWE/depth closure was `8.882e-16 m`. Modeled wind redistribution was exactly zero by implementation status; this does not establish that physical redistribution at the SNOTEL sites was zero.

## Frozen Operator Results

| Lane | Operator | Frozen rubric offset (d) | Executed frozen operator (d) | B modeled/observed peak | Diagnostic attribution |
|---|---|---:|---:|---:|---|
| Mica Creek / St. Joe, ID | seasonal_ablation_meltout_date | -35.0 | -35.0 | 0.6191533199355197 | pre-peak input/loss ownership mixed or unresolved |
| Niwot, CO | seasonal_peak_depth_date | -46.5 | -46.5 | 0.47191210270260325 | pre-peak input/loss ownership mixed or unresolved |
| Niwot, CO | seasonal_peak_swe_date | -31.0 | -31.0 | 0.4953842559051843 | pre-peak input/loss ownership mixed or unresolved |
| Paradise, WA | seasonal_ablation_meltout_date | -37.0 | -37.0 | 0.4734826857635193 | realized input-pathway/pre-peak-loss boundary unresolved |
| Snowbird, UT | seasonal_peak_swe_date | -44.5 | -44.5 | 0.3899826506038172 | pre-peak input/loss ownership mixed or unresolved |

## Interpretation Boundary

Baseline seasonal modeled-peak magnitudes are about `0.39-0.62` of the corresponding observed peaks. Separately, baseline modeled SWE retained on the observed SWE-peak dates has lane medians of about `0.21-0.46` of observed SWE, establishing a larger observed-date storage deficit. At Paradise, initial SWE plus realized snowfall SWE and retained rain remain below the observed peak in every evaluated water year and cell. Because retained rain is endogenous to pack state and liquid capacity, this localizes the boundary to the realized input pathway versus pre-peak losses; it does not prove an external forcing defect. Mica Creek, Niwot, and Snowbird sometimes receive enough realized snowpack input to reach the observed peak before recorded losses; their ownership is likewise mixed or unresolved. The ledger cannot separate precipitation representativeness, gauge undercatch, phase, liquid retention, physical redistribution, and pre-peak modeled loss timing.

The four CoE columns are signed empirical melt-depth contributions. They help localize when the current formula removes or retains snow, but they are not separately observed energy fluxes. `bmelt` and `cmelt` mix temperature, cloud, wind, dewpoint, and canopy effects and cannot be treated as unique sensible-heat measurements. The result does not authorize tuning: all four lanes require finer phase-conditioned timing and ownership analysis of the realized input pathway and pre-peak losses before a process amendment is admissible.
