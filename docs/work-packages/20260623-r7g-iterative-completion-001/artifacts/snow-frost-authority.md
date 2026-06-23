# Snow/Frost Authority

Status: HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED.

## Contract Authority

- `snow.options.snow_file_present` is provenance only and does not activate
  snow coupling by itself.
- Active snow coupling starts when runtime SWE is present, or when a day is
  thermally snow-active and projected controls are present.
- `Snow-Water` publication derives from runtime SWE after daily snow coupling.
- `RM` publication derives from post-winter rain plus routed melt plus
  irrigation.
- Active frost requires baseline-authoritative frost/thaw heat-flow and
  fine-layer storage semantics from `SC-SNOWFREEZE-001`.

## Closed In This Package

- Sidecar-only snow is no longer treated as active.
- Runtime SWE activates snow without sidecar presence.
- Wet, thermally active snow days use typed direct hourly winter forcing and
  typed direct snow partition.
- Snow carry is persisted in `DirectSnowRuntimeCarry` and projected through
  R4G/R4PQZ.
- Snow liquid forcing for WB14/WB16 now uses post-winter rain, canopy
  interception, and routed melt consistently.

## Terminal Blocker

`HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED`.

Production direct now executes active frost and preserves the no-compatibility
runtime gate, but projection parity is still red. The retained H2637 direct
default run (`direct-default-frost11`) completed in `89.88 s` under the
`91.2 s` budget with zero compatibility edges, while WAT/PASS residuals remain
material:

- `frozwt`: `34363` differing rows, max `11.12017732034371 mm`.
- `frdp`: `34363` differing rows, max `264.39519767438975 mm`.
- `Snow-Water`: `21305` differing rows, max `183.04425009202413 mm`.
- `QOFE`: `219591` differing rows, max `492.56198359095436`.

Later fine-layer carry preservation runs showed that the retained `89.88 s`
timing was not a stable closure point. Preserving the zero-material fine/shadow
state increased the latest measured H2637 endpoint to `188.57-195.27 s` while
parity stayed red.

Closure now requires a frost architecture migration:

- persistent fine/shadow state is canonical lane state, not reconstructed from
  coarse layers each day;
- no-material no-op days preserve carry without emitting coarse layer
  projection;
- coarse layer mutation is emitted only with explicit liquid/frozen storage
  closure;
- publication operands read typed solver outcome fields;
- the hot loop has no map-backed request/surface frost authority.
