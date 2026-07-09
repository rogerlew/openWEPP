# Numeric Equivalence

Evidence label: Static/Ran.

Status: `TARGETED-PASS`

Static:

- Refactor was helper extraction only. It did not change formulas,
  thresholds, units, guard labels, public output fields, runtime-symbol names,
  or serialization surfaces.
- Accumulation order was preserved for direct sediment assembly:
  contributor hillslopes first, dependency channels second, with class mass and
  top/lateral partitioning unchanged.
- WS11 ipeak branch dispatch and WS12 impoundment route/outflow equations were
  preserved in branch-specific helpers.

Ran:

- `cargo nextest run -p openwepp-watershed-orchestrator` - pass,
  `68 tests run: 68 passed`.
- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract` - pass,
  `18 tests run: 18 passed`.

Output/API evidence:

- `wshedw5_typed_watershed_runtime_contract` validates direct routed state
  publication, non-aliased channel balance operands, ipeak branch closure,
  hourly contributor behavior, transport-capacity response, impoundment
  projection guard classes, and active impoundment outflow composition.
- No public API signatures were changed.
