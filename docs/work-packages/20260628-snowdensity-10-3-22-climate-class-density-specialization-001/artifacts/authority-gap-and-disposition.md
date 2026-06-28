# Authority Gap And Disposition

Evidence mode: `Static + focused runtime unit evidence`

## Authority Checked

Sturm 2010 Table 4 and Eq. 6 provide a class-density trajectory form:

`rho = (rho_max - rho_0) * (1 - exp(-k1 * depth_cm - k2 * day_of_year)) + rho_0`

The available local table covers:

| Class | rho_max | rho_0 | k1 | k2 |
|---|---:|---:|---:|---:|
| alpine | 0.5975 | 0.2237 | 0.0012 | 0.0038 |
| maritime | 0.5979 | 0.2578 | 0.0010 | 0.0038 |
| prairie | 0.5940 | 0.2332 | 0.0016 | 0.0031 |
| tundra | 0.3630 | 0.2425 | 0.0029 | 0.0049 |
| taiga | 0.2170 | 0.2170 | 0.0000 | 0.0000 |

Ephemeral is part of the six-class Sturm snow-class system, but the local
Sturm 2010 authority states ephemeral measurements were excluded and supplies no
parameter row.

## Blocking Gap

The package requirement is forcing-derived class assignment from the run's own
wind, precipitation, and air temperature. The local authority set did not expose
the numeric Sturm 1995 binary decision-tree thresholds needed to implement that
assignment. Implementing thresholds from fixture behavior, site names, NSIDC
raster lookup, or visual inference would violate `INV-SNOWFREEZE-077`.

## Disposition

The selector is reserved and fail-closed. The candidate is not promoted and the
default is not changed.

Required follow-on authority:

- Numeric Sturm 1995 decision-tree thresholds or equivalent cited authority.
- Ephemeral density parameters or a separately ratified fallback.
- A real cross-SNOTEL direct-production WAT/trace rerun after those authorities
  are present.
