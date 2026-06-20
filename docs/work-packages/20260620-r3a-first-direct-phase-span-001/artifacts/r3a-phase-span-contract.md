# R3A Phase-Span Contract

Status: complete.
Evidence mode: Static + Ran.

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

## Selected Span

Static:

- Span name: direct transfer-input accounting.
- Direct phase order:
  `DirectPhaseKind::Normalization -> DirectPhaseKind::LateralTransfer`.
- Authority: typed arithmetic bookkeeping over already-projected direct
  forcing and transfer buffers. No process-physics formula is introduced.

Typed inputs:

- `DirectDayForcing::precipitation_m`
- `DirectDayForcing::effective_temperature_c`
- `DirectTransferBuffers::surface_carry_m`
- `DirectTransferBuffers::lateral_carry_m`
- `DirectTransferBuffers::upstream_flow_m`
- `DirectTransferBuffers::subsurface_input_m`

Direct compute:

- validate finite typed inputs;
- validate nonnegative water-depth style inputs;
- sum surface transfer buffers;
- sum lateral transfer buffers;
- compute transfer input total;
- compute total accounted input.

State mutation:

- mutate a direct phase-span accounting state on `DirectDayFrame`.

Downstream operands:

- produce direct downstream transfer-accounting operands for the span.

Shadow projection:

- produce a typed shadow projection from the downstream operands for exact
  identity checks in focused fixtures.

## Implementation Result

Static:

- Direct phase order implemented by `DIRECT_R3A_INPUT_ACCOUNTING_SPAN`.
- Direct input accounting implemented by
  `DirectDayFrame::run_r3a_input_accounting_span`.
- Direct state, downstream operands, and shadow projection are concrete typed
  structs exported from the orchestrator crate.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r3a_ -- --nocapture`: PASS.
- Forbidden direct-runtime compatibility source scan: no matches.
- Runtime counter tests: PASS.

Disposition: selected span contract satisfied.
