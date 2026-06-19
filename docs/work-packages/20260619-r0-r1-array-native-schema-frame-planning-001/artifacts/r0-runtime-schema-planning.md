# R0 Runtime Schema Planning

Status: complete for planning-only scope.
Evidence mode: Static.

## Disposition

R0 implementation is not complete. This package completes the planning envelope
for R0 while PERFDEEP07 remains in `HOLD`.

The runtime schema must be authored as a typed field schema before direct-mode
implementation. The schema must not be a whole-registry dense mirror and must
not encode hot state as `BoundaryValue` slots.

## Schema Families

| Family | Required schema fields |
|---|---|
| Identity/context | run id, hillslope id, OFE id, day index, calendar fields, water year, contributor count |
| Static topology | lane order, upstream/downstream relation, area ratios, runoff geometry |
| Forcing | borrowed climate/hyetograph/winter hourly views, precipitation/liquid input aliases |
| Core water state | soil water, infiltration, runoff, ET, drainage, seepage, lateral flow, tile flow |
| Soil layers | active layer count, thickness, porosity, field capacity, wilting point, conductivity, storage |
| Frost/snow | frozen water, frost depth, bottom overflow, SWE, routed melt, post-winter rain |
| Plant/residue/decomposition | growth and decomposition persistent scalars needed by phase spans |
| Transfer buffers | surface/lateral hourly carry arrays, upstream flow, subsurface input, downstream outputs |
| Publication operands | HBP/WAT/PASS/loss/manifest typed operands and provenance |
| Guards/diagnostics | field-level finite/domain bounds, diagnostic subject aliases, status message classes |

## Required Column Set

Future schema artifacts must include these columns for each field:

- direct field id;
- Rust type / unit wrapper;
- shape (`scalar`, `[T; N]`, slice, SoA column, projection-only);
- producer phase;
- consumer phases;
- persistence lifetime;
- seed source;
- output/projection destination;
- legacy symbol aliases;
- guard bounds and runtime-derived bound sources;
- canonical contract or decision authority;
- diagnostic subject name;
- validity/absence representation;
- fixture and H2637 evidence obligations.

## Gate

PASS for planning-only R0. Runtime schema implementation remains blocked by the
PERFDEEP07 hold-lift gate or an explicit supersession decision.
