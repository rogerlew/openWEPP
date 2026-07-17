# Literature And Intake Reconciliation

Evidence class: `Static` (source and paper inspection)

## Published GSI law

Jolly, Nemani, and Running (2005) define three daily piecewise-linear
constraint indicators:

- minimum temperature: inactive at or below -2 °C and unconstrained at or
  above 5 °C;
- vapor-pressure deficit: unconstrained at or below 900 Pa and inactive at or
  above 4100 Pa; and
- photoperiod: inactive at or below 10 h and unconstrained at or above 11 h.

The instantaneous GSI is their product. The operational GSI is the arithmetic
mean of the trailing 21 daily products. The paper uses a 0.5 crossing as a
diagnostic onset/offset threshold and explicitly proposes scaling potential LAI
with the continuous index. It reports Harvard Forest onset/offset comparisons
and tests the same general parameterization across global Northern and Southern
Hemisphere sites.

Primary citation: <https://doi.org/10.1111/j.1365-2486.2005.00930.x>.

## Existing openWEPP surface

- Native `landuse=3` forest input, typed growth operands, runtime projection,
  and canonical YAML are already implemented.
- Native forest currently enters the same `imngmt=2` crop-style perennial
  growth equations as cropland perennial inputs.
- Current senescence is heat-unit based. That is unsuitable as the autumn
  forest cue because warmer sites accumulate heat units sooner.
- The pinned legacy frost/daylength decline is reachable only through the
  legacy rangeland branch. It corroborates the physical drivers but does not
  define a first-class forest implementation.
- Dynamic residue mass/depth and litter input exist, but the recurring forest
  drop window remains anchored to `jdharv` pending physical phenology.

## Scope consequence

The process kernel can be implemented exactly now. Production integration
cannot be completed honestly until the next package ratifies how continuous
GSI changes native canopy cover and LAI while separating foliar, persistent
structural/evergreen, and litter pools. This package therefore closes the
driver/state law and leaves that integration visibly open.
