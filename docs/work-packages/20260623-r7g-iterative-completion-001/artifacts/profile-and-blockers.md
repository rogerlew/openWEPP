# Profile And Blockers

Status: HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED.

Performance disposition:

- Retained frost-enabled direct default endpoint timing: `89.88 s`
  (`direct-default-frost11`).
- `<=10x` budget: `91.2 s`.
- Runtime counters preserve the no-compatibility gate:
  `compatibility_edge_invocations=0`.
- This green timing is superseded for closure. Fine-layer carry preservation
  later measured `188.57-195.27 s`, so endpoint performance is not currently
  proven green.

Closed runtime blockers:

- False sidecar-only active snow.
- Real active snow typed partition authority.
- Same-day upstream EROD14 qout handoff.
- Snow liquid-event hyetograph mismatch.

Terminal blocker:

`HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED`.

Concrete source context:

- Production direct day input creates an active frost context and seeds frost
  runtime carry/projection surfaces:
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`.
- Existing active frost compute enters through
  `Wb11HydrologyKernel::compute_active_frost_coupling` and request/symbol
  surfaces:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`.
- Direct R4A can apply a frost partition once supplied:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`.
- Current first frost mismatch begins at H2637 WAT row `76` (year `1`, sim day
  `5`, julian `5`, OFE `1`): compatibility publishes
  `frozwt=0.005660437443662737` and `frdp=0.11700610732602637`, while direct
  still publishes zero.

Next remediation route:

Stop treating frost as a one-day surface projection problem. Scaffold a
follow-up package that migrates active frost to a coupled stateful sub-solver
with canonical persistent lane state. The sub-solver must own fine layers,
layer shadows, front scalars, liquid exchange, no-material carry, downstream
operands, and publication projection as typed state. It must remove
`DirectFrostRunoffSurface`/symbol-map authority from the hot loop and then
rerun H2637 parity/performance gates.
