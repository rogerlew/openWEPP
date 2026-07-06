# Consumer Path Evidence

Status: held-no-builder-consumer
Evidence mode: Static

Current consumer path:

1. `05_runner_execution_and_outputs.rs` creates `LanedShadowCollector` only
   when `OPENWEPP_LANED_SHADOW=1`.
2. `laned_shadow_geometry()` supplies geometry only: length, width, and mean
   slope.
3. The collector observes streamed `DirectPublicationDayRow` records and
   reconstructs source depths from `dc01_surface_hourly_weights × runvol/area`.
4. `commit_day()` constructs `CascadeSegment` meshes with
   `CellParameters::bare(geom.mean_gradient, LANED_SHADOW_KO)`.
5. `CascadeForcing` receives live source rates but a shadow-local `I=0`
   rainfall-intensity closure.
6. `run_cascade` consumes the real routing cascade.

Negative proof:

- There is no current builder object or payload between publication rows and
  `CellParameters`.
- The real shadow consumer still reads the hardcoded bare policy for friction.
- Existing H2637 shadow evidence proves live water-row consumption and
  protected-output identity, not friction operand consumption.

Consumer-facing claim disposition:

D11 makes no claim that active friction operand consumption is closed. The
consumer path remains a hold boundary until a real builder is implemented and
exercised by a friction-sensitive downstream test.
