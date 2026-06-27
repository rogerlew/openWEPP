# Disposition

Evidence mode: Ran.

Closure: `COMPLETE-10-3-4-MARITIME-OVERACCUMULATION-DIAGNOSED`.

Package disposition: `PARTITION-THAW-FIRST`.

## Result

The maritime over-accumulation signal is real on paired Sleepers and Harvard
surfaces:

- `sleepers_south_field`: fail fraction `0.838542`, mean residual `0.410817 m`.
- `sleepers_w9_hardwood`: fail fraction `0.740933`, mean residual `0.325021 m`.
- `harvard_hardwood`: fail fraction `0.796875`, mean residual `0.383192 m`.
- `harvard_open`: fail fraction `0.833333`, mean residual `0.391934 m`.

The diagnostic ranking is:

1. `snow_rain_partition_near_zero_c` - `DEFECT-ELIGIBLE`.
2. `winter_thaw_melt_response` - `DEFECT-ELIGIBLE`.
3. `sub_canopy_longwave_or_forest_energy` - `DEFECT-ELIGIBLE`.
4. `rain_on_snow_heat` - `DEFECT-ELIGIBLE`, but not first.
5. `precipitation_bias` - `FORCING-LIMITED`.
6. `representativeness` - `FORCING-LIMITED`.
7. `wind_undercatch` - `NOT-SUPPORTED`.

## Boundary

No production snow/frost physics changed. This package authorizes the next
diagnostic route; it does not authorize a production correction.
