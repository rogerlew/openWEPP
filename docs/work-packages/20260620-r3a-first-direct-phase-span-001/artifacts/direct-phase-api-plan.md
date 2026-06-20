# R3A Direct Phase API Plan

Status: queued.
Evidence mode: not run.

Plan the API changes before implementation.

Required entries:

- selected phase-span entrypoint;
- typed input view;
- compute function;
- mutation target;
- downstream operand structure;
- shadow projection structure;
- status/error type;
- audit/counter fields.

The API plan must avoid compatibility storage, request, writeback, registry,
hot-table, indexed-surface, dense-refresh, and dirty-flush types in direct
phase execution.
