# Ep Magnitude Initialization Diagnosis

Status: completed

Evidence mode: static+ran

## Diagnosis

Ran: H1/H7/H39 day-1 post-`plant_root_uptake` trace rows show:

- `Etp = 0.385294 mm`
- `Ep = 0.385294 mm`
- `ΣUi_#### = 0.385294 mm`
- `Ws = 1.0`
- `lai = 11.874844`
- `rtd = 1.800000 m`
- raw/effective `pltol = 0.100000`

Ran: Baseline WAT day-1 `Ep = 0.150000 mm` for H1, H7, and H39.

Ran: Candidate WAT day-1 `Ep = 0.385294 mm` for H1, H7, and H39.

Ran: Minimum traced `theta/(pltol*ul)` ratios are:

- H1: `2.100182`
- H7: `1.723200`
- H39: `2.663155`

Static: Legacy `evap.for` seeds `ep` from current `lai` before
`watbal_hourly.for` calls daily `ptgrp`/`ptgra`; legacy `swu.for` then consumes
the seeded demand after root/growth state is updated.

## Residual Ownership

The H1/H7/H39 stable `Ep` split is classified as upstream demand or
initialization magnitude, not final publication, SWU stress clipping, or
layer-uptake identity failure.

The next package should compare the exact `evap.for` demand seed inputs
against openWEPP pre-growth and post-growth plant state:

- `eo`/`Eu`
- `lai`
- `cancov`
- `Etp`
- `Ep` before `swu`
- day boundary plant growth/root initialization state
