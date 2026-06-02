# HPHYS0244 Focus Recommendations

Static: recommendation synthesized from HPHYS0244 diagnostics and prior HPHYS
artifact review.
Ran: targeted H1/H7/H39 WAT comparisons and source-lineage probes.

## Recommendation
Focus the next implementation package on WB11/WB18 mutable storage continuity
and percolation flux observability before changing production physics.

## Rationale
1. `Total-Soil` and `SoilWaterTotal` are identical residual families in the
   HPHYS0244 slice, matching HPHYS0243 cohort evidence. This is aggregate
   storage lineage, not output aliasing.
2. `Dp` is a first-week overdrain transient, not a uniform full-period positive
   bias:
   - `H1` day-1 `Dp` delta is `+44.004399 mm`; full-period signed mean is
     `-0.077336 mm`.
   - `H7` day-1 `Dp` delta is `+33.612610 mm`; full-period signed mean is
     `-0.096054 mm`.
   - `H39` day-1 `Dp` delta is `+22.740342 mm`; full-period signed mean is
     `-0.064031 mm`.
3. Candidate storage is already severely below baseline during the same first
   week:
   - `H1` day-1..7 mean `Total-Soil` delta: `-191.819552 mm`.
   - `H7` day-1..7 mean `Total-Soil` delta: `-171.992073 mm`.
   - `H39` day-1..7 mean `Total-Soil` delta: `-210.062828 mm`.
4. WB18 `Pe` and layer `st`/`theta` exist internally but are not emitted in
   current artifacts. Without telemetry, any production math change would be
   under-instrumented and likely repeat the HPHYS0212/0213 pattern of local
   guard closure without cohort residual closure.

## Next Package Shape
Recommended next package:
`20260602-hphys0245-wb11-wb18-layer-telemetry-and-storage-continuity-001`.

Initial package scope:
- Add diagnostics-only trace extraction for `H1`, `H7`, and `H39` covering
  day `1..30`.
- Trace the same symbols at post-seed, post-WB18, post-WB19, pre-WB13, and
  post-WB13 boundaries:
  - `wb18_perc_theta_*`
  - `wb18_perc_pei_*`
  - `D`
  - `Pe`
  - `wb11_soil_water`
  - WB13 `Total-Soil`
  - WB13 `SoilWaterTotal`
- Compare trace deltas against baseline-accessible aggregate surfaces first
  (`TSW`, WAT storage), and only then decide whether pinned-baseline layer
  instrumentation is required.
- Keep production process-physics changes out of the first telemetry slice.

## Do Not Prioritize
- Do not prioritize `Q`/`QOFE`; HPHYS0243 closed those semantically.
- Do not prioritize WAT naming/publication formatting for
  `Total-Soil`/`SoilWaterTotal`; source and residual evidence point upstream to
  `wb11_soil_water` lineage.
- Do not tune `Dp` alone. The `Dp` transient is coupled to storage depletion,
  so an isolated percolation clamp would risk masking the storage defect.

## Exit Signal for Follow-On
The follow-on package should produce one table per hillslope/day showing the
storage balance:
`wb11_soil_water_start`, `Σwb18_perc_theta_start`, `Σwb18_perc_pei`,
`D`, `Pe`, `wb11_soil_water_end`, `Total-Soil`, and `SoilWaterTotal`.

That table is the minimum evidence needed to decide whether the next code
change belongs in WB11 seed/carry, WB18 percolation writeback, WB19 aggregate
mutation, or WB13 publication.
