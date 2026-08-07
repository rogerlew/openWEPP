# Wind Forcing Custody Freeze

Static: frozen before new attribution execution.

All four fixture manifests describe the climate as observed DAYMET daily
precipitation/temperature, GRIDMET wind, CLIGEN sub-daily storm patterning, and
PRISM revision. The `.cli` file is the runtime boundary; its daily `w-vl`
column is the value consumed by the direct runner and repeated into every
Stage 3 hourly/substep tuple. The retained execution receipt binds the exact
CLI hashes and runfile consumer paths.

User-supplied source documentation identifies GRIDMET `vs` as wind velocity at
`10 m` and NLDAS-2 air temperature as `2 m`; Abatzoglou (2013) likewise
describes the `10-m gridded wind field` and height adjustment for station
comparisons. The supplied sources are the Google Earth Engine GRIDMET catalog,
the NLDAS-2 forcing documentation, and DOI `10.1002/joc.3413`. This resolves
the nominal native variable heights, subject to direct source admission in the
follow-on package.

The checked-in custody still does not identify the GRIDMET source product
version, grid-cell identifier, spatial support, native temporal cadence,
aggregation operation, or surface-exposure class. A nominal 10 m product
height alone does not establish that `w-vl` is open-site, above-canopy,
within-canopy, or sub-canopy wind. Per site the exposure disposition is
`WIND_FORCING_EXPOSURE_UNRESOLVED`.

The contracted `5 m` values are virtual `z_T`, `z_q`, and `z_u` geometry above
the instantaneous modeled snow surface. They are forcing/model metadata and
are not evidence that a physical sub-canopy instrument existed at 5 m.

No wind attenuation, fitted coefficient, or site-specific reinterpretation is
admitted.

## Executed Result

Ran: exact climate hashes, receipt consumer paths, tuple forcing fingerprints,
and cross-lane fixed wind values passed at all four sites. This proves custody
from each retained CLI `w-vl` value into Stage 3 tuples. It does not add missing
native GRIDMET product, cell, cadence transformation, or exposure metadata.
User-supplied references establish the nominal `10 m` wind and `2 m`
temperature heights, but have not yet been admitted as exact package
authority. Every site therefore remains
`WIND_FORCING_EXPOSURE_UNRESOLVED`; the `5 m` virtual geometry remains model
metadata, not an instrument or sub-canopy exposure claim.
