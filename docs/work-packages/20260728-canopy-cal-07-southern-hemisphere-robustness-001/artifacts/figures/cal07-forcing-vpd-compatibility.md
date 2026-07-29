# Contract-Defined VPD from Frozen Southern Hemisphere Forcing

## Caption

Daily vapor-pressure deficit reconstructed from the frozen NASA POWER
temperature and dew-point series for Beza Mahafaly, Madagascar (top), and
Alerce Costero, Chile (bottom), from 2022-01-01 through 2026-07-24. The blue
line is the contract-defined VPD. The red horizontal line is the zero lower
bound, and red circles identify the three Alerce days that fall below it.

## How to read it

Each panel has its own vertical range so the seasonal structure remains
visible; compare timing and boundary crossings, not line height between
panels. Values above the red line are admissible. Any point below it is a hard
input-domain failure under the current plant contract. The two 2022 failures
occur during warm-up, but the third occurs on 2025-09-09 inside a planned
observational scoring year.

## Plain-language takeaway

The Beza forcing is compatible with the kernel, but the Alerce forcing is not.
Only three of 3,332 source days fail, yet one failed day is enough to stop the
continuous stateful run. Silently clipping those values to zero would change
the input and violate the prespecified science contract.

## Methods and source binding

For every retained source row, VPD was independently calculated in Pa as
`1000 * (0.5 * (es(Tmax) + es(Tmin)) - es(Tdew))`, where
`es(T) = 0.6108 * exp(17.27 * T / (T + 237.3))`. This duplicates the mandated
equation over raw operands rather than reading executor output.

The embedded SVG metadata binds
`artifacts/forcing-diagnostics.csv`, SHA-256
`a705f8c935f6aa5486f3a28ab63a85e5289e8a9776a474d9dedb8093528a1719`.
The underlying POWER source objects and queries are bound in
`artifacts/source-manifest.csv`.

## Limitations

NASA POWER is gridded/reanalysis forcing, not site meteorology. Its returned
grid elevation differs from both camera sites, especially Alerce (99.4 m grid
versus 840 m site). This plot diagnoses contract compatibility; it does not
measure forcing accuracy or establish why the source operands are
inconsistent.

## Accessibility

The zero boundary is encoded by both position and a red rule. Failures are
encoded by red circular markers as well as falling below the boundary.
The SVG provides a title, long description, high-contrast marks, and exact
source metadata.
