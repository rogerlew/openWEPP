# Supported Domain And Exclusions

The selected `OPENWEPP_SNOW_FREE_LSE_V1` domain is one positive-duration,
positive-area, snow-free interval with positive finite neutral-domain wind.
It supports open and canopy-covered tiles, bare mineral soil, an explicit
forest-litter layer, day/night, wet/dry states, rain/runon, full/partial water
authorization, liquid evaporation, and liquid condensation.

Typed unsupported branches are:

- snow at either endpoint or a snow-terminal payload;
- frozen or thawing soil/litter/surface water;
- calm wind or nonneutral stability;
- more than one ground surface class in one tile;
- missing water temperature/enthalpy lineage;
- missing litter or soil thermal/optical configuration; and
- hydraulic redistribution or a negative owner withdrawal.

No hidden wind floor, temperature clamp, PMET complement, or legacy fallback
enters the admitted domain.
