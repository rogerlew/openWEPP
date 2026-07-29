# Hourly Products Reconstruct the Frozen CAL-07 Daily Operands

## Caption

Reconstructed-minus-reported residuals linking current hourly POWER products
to CAL-07's frozen daily fields. Daily Tmin and Tmax reproduce exactly.
Mean dew-point residuals are -0.00417 C, -0.00083 C, and -0.00333 C. The
corresponding contract-VPD residuals are +0.332 Pa, +0.066 Pa, and +0.292 Pa.

## How to read it

The upper panel uses degrees Celsius: blue and orange zero-height marks are
the exact Tmin/Tmax reconstructions, while green bars are mean dew-point
residuals. The lower panel reports the derived VPD difference in Pa. All
temperature residuals are inside the prospectively frozen inclusive 0.01 C
serialized-resolution tolerance, and all daily signs agree.

## Plain-language takeaway

The hourly and frozen daily products align closely enough at their published
resolution to support the aggregation diagnosis. API versions differ and the
daily response exposes only a multi-year aggregate source list, so this is
serialized product compatibility—not proof of identical upstream processing.

## Methods and source binding

Hourly Tmin/Tmax are the extrema of the exact 24 LST `T2M` values; mean dew
point is the arithmetic mean of the exact 24 `T2MDEW` values. CAL-07 VPD is
recalculated from its frozen raw operands rather than copied from its prior
diagnostic.

The embedded SVG metadata binds `artifacts/daily-decomposition.csv`,
SHA-256
`7f23d8c904b9b591d42c7d242ff6f47b282c5b1150299e8b5a3e89b0e47f74c5`.

## Limitations

The 0.01 C threshold is a prespecified serialized-resolution reconstruction
tolerance, not measurement precision or physical uncertainty. The daily API
reports v2.9.5 with aggregate sources `GEOSIT,MERRA2,POWER`; the hourly API
reports v2.9.6 and `MERRA2`. This package classifies that relationship as
`AGGREGATE_OVERLAP_ONLY`.

## Accessibility

Temperature operands use color plus explicit legend labels. Dates and signed
VPD residuals are printed directly. Zero is a visible reference line, and the
SVG includes title, description, units, and source metadata.
