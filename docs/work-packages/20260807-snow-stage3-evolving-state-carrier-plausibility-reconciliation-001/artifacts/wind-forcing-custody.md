# Wind Forcing Custody Freeze

Static: frozen before new attribution execution.

All four fixture manifests describe the climate as observed DAYMET daily
precipitation/temperature, GRIDMET wind, CLIGEN sub-daily storm patterning, and
PRISM revision. The `.cli` file is the runtime boundary; its daily `w-vl`
column is the value consumed by the direct runner and repeated into every
Stage 3 hourly/substep tuple. The retained execution receipt binds the exact
CLI hashes and runfile consumer paths.

The checked-in custody does not identify the GRIDMET source product version,
grid-cell identifier, spatial support, native temporal cadence, aggregation
operation, native reference height, or surface-exposure class. It therefore
does not establish that `w-vl` is open-site, above-canopy, within-canopy, or
sub-canopy wind. Per site the exposure disposition is
`WIND_FORCING_EXPOSURE_UNRESOLVED`.

The contracted `5 m` values are virtual `z_T`, `z_q`, and `z_u` geometry above
the instantaneous modeled snow surface. They are forcing/model metadata and
are not evidence that a physical sub-canopy instrument existed at 5 m.

No wind attenuation, fitted coefficient, or site-specific reinterpretation is
admitted.
