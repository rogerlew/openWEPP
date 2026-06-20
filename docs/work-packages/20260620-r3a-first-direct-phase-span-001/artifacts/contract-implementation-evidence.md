# R3A Contract Implementation Evidence

Status: complete.
Evidence mode: Static + Ran.

R3A implements the selected direct transfer-input accounting span without
changing any `SC-*` contract, output schema, output unit, output metadata, or
publication operand.

Implementation mapping:

- Inputs: `DirectDayForcing`, `DirectTransferBuffers`, direct water state, and
  direct publication frame fields are carried on `DirectDayFrame`
  (`direct_runtime.rs:158`).
- Direct compute: `run_r3a_input_accounting_span` validates domains, sums
  surface/lateral transfer buffers, computes transfer input, and computes total
  accounted input (`direct_runtime.rs:217`).
- State mutation: computed terms mutate `DirectInputAccountingState`
  (`direct_runtime.rs:237`).
- Downstream operands: state projects into `DirectDownstreamOperands`
  (`direct_runtime.rs:249`, `direct_runtime.rs:493`).
- Shadow projection: downstream operands project into `DirectShadowProjection`
  (`direct_runtime.rs:252`, `direct_runtime.rs:533`).
- Typed errors: invalid direct values fail closed as
  `NonFiniteDirectValue`/`NegativeDirectValue` (`direct_runtime.rs:789`).

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r3a_ -- --nocapture`: PASS,
  including valid-span identity, invalid input rejection, and
  non-tautological compatibility-edge counter positive path.

Disposition: PASS. The implemented span is arithmetic bookkeeping over typed
direct inputs only; it introduces no surrogate process physics.
