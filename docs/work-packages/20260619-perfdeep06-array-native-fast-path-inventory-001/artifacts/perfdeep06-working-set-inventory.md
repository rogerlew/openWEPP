# PERFDEEP06 Working-Set Inventory

Status: queued.
Evidence mode: not-run.

## Required Content

Classify the H2637 hot-loop state by:

- symbol or field name;
- unit type or raw scalar type;
- source location;
- phase owner and consumers;
- lifetime: lane-persistent, start-of-day seed, phase-local, fixed hourly
  array, layer SoA, borrowed forcing, publication operand, diagnostic-only,
  replay-only, or true I/O edge;
- proposed direct-frame disposition;
- evidence class (`Static:` or `Ran:`).

## Gate

This artifact is complete only when the next implementation package can derive
its initial frame schema and phase API scope from this inventory without
re-searching the whole codebase.
