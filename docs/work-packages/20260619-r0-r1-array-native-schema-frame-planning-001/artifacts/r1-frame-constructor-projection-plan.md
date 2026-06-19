# R1 Frame Constructor and Projection Plan

Status: complete for planning-only scope.
Evidence mode: Static.

## Constructor Direction

R1 constructors must build direct frame structures from parsed input and
existing runtime setup without replacing execution.

Allowed planning/implementation boundary for R1 while PERFDEEP07 remains HOLD:

```text
parsed inputs/current setup -> direct frame constructors -> shadow/roundtrip projection
```

Not allowed:

```text
direct frame -> direct executor -> runtime readiness claim
```

## Constructor Surfaces

| Constructor | Input authority | Output |
|---|---|---|
| `DirectRunFrame::from_parsed_inputs` | parsed hillslope/climate/management/run identity | static topology, schema, provenance, reusable buffers |
| `DirectLaneFrame::from_static_ofe` | static OFE lane slices, soil/frost/snow/pmetpara setup | persistent lane state and layer arrays |
| `DirectDayFrame::seed` | lane frame plus borrowed day forcing | mutable day frame and transfer buffers |
| `DirectPublicationFrame::from_day_frame` | terminal direct day frame | typed publication operands and metadata |

## Projection Surfaces

R1 projection must be able to compare typed frame state to current outputs
without making direct execution authoritative:

- direct frame -> shadow logical surface for comparison only;
- direct publication frame -> HBP/WAT/PASS/loss/manifest expected operands;
- direct frame diagnostics -> legacy names at the diagnostic edge only.

## Acceptance for Future R1 Implementation

- roundtrip fixture proves constructor/projection identity for selected seeded
  fields;
- no direct executor activation;
- no H2637 endpoint claim unless a later runtime stage explicitly executes it;
- no publication cutover without promoted ledger and independent operand
  reconstruction requirements.

## Gate

PASS for planning-only R1. Implementation remains blocked to non-activated
constructors/projections until PERFDEEP07 is closed or superseded.
