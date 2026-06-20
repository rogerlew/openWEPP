# R3A Phase-Span Contract

Status: queued.
Evidence mode: not run.

Execution must select one complete direct phase span before Rust edits.

The selected span must include all of the following:

- typed inputs;
- direct compute;
- direct state mutation;
- downstream operands;
- shadow projection.

Gate:

- phase-span identity for the selected fixture(s);
- no-compatibility call-graph proof;
- non-tautological runtime counters.

Runtime counters must record direct phase entry, direct compute, state
mutation, downstream operand production, shadow projection, and zero direct
span compatibility edge invocations. If a compatibility edge counter is added,
tests must prove it is not an always-zero field.

This artifact must name the selected phase(s), input fields, mutated fields,
downstream operands, shadow projection surface, identity fixture(s), and
authority for every direct computation.
