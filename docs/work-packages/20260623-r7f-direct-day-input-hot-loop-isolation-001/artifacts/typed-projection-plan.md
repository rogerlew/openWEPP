# Typed Projection Plan

Status: complete.

## Target

Production direct day/OFE execution consumes typed direct frame/lane/day inputs
without invoking the compatibility-shaped direct publication builder in the hot
loop.

## Typed Sources

- Calendar, precipitation, and effective temperature: `ClimateRunSpanSummary`
  plus a narrow typed climate-runtime accessor for authoritative
  `timem`/`intsty` series.
- Lane state: `DirectRunFrame` / `DirectLaneFrame` fields for water storage,
  transfer buffers, subsurface layers, ET stage state, frost runtime carry, and
  erosion downstream operands.
- Static publication geometry: `DirectLaneFrame` runoff publication geometry
  fields.
- Static process controls: parsed once from lane seed authority during setup
  until later R7/R8 static authority migration replaces that seam.

## Completed Execution Strategy

1. Added a production-only typed day-input path:
   `DirectProductionDayInputBuilder`.
2. Kept compatibility day-input builder available only for explicit
   shadow/cutover modes.
3. Updated tests so production direct asserts zero compatibility-edge
   invocations and static no-builder call graph.
4. Kept material snow/frost unsupported states fail-closed rather than
   inventing replacement physics.

## Residual Risk

Some snow/frost/ET helper functions still expose map-backed direct helper APIs.
R7F isolates the production hot day/OFE loop; it does not finish the broader
static process-control authority migration.
