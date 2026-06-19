# PERFDEEP06 Direct-Frame API Plan

Status: queued.
Evidence mode: not-run.

## Required Content

Define the API shapes for the first direct-frame implementation package:

- `&mut HillslopeDayFrame` or view types passed to migrated phase functions;
- borrowed forcing/context inputs;
- phase-owned mutable outputs;
- guard/fail-closed error attribution;
- validity and dirty-state representation;
- mixed-mode boundary between migrated and non-migrated phases;
- shadow identity comparison strategy;
- rollback/kill criteria if identity or endpoint gates fail.

## Gate

This artifact is complete only when the follow-on package can implement a
bounded direct-frame hydrology fast path without inventing new authority during
execution.
