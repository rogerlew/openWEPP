# Worker Handoff

Status: HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED.

Active marker:

`HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED`.

What changed:

- Production direct no longer treats snow sidecar presence as active snow.
- Production direct now carries typed snow state and computes active snow
  partition from typed hourly winter forcing.
- Snow liquid forcing for WB14/WB16 uses post-winter rain, interception, and
  routed melt consistently.
- EROD14 same-day upstream `qout` handoff is separate from sediment publication
  authority.
- Production direct now runs active frost to full H2637 endpoint with zero
  compatibility-edge counters.
- Active-frost performance was reduced from `163.88 s` to a retained
  `89.88 s`, passing the `<=10x` budget of `91.2 s`.
- Later fine-layer carry preservation proved that the green timing was not a
  stable architecture result. `direct-default-frost29` and
  `direct-default-frost30` measured `188.57-195.27 s`, still with zero
  compatibility edges.
- Current source state includes focused safeguards for active zero-material
  frost carry, no-freeze fine/shadow carry, material-gate removal, and
  no-material R4A consumer behavior. Full H2637 has not been rerun after the
  final no-material consumer safeguard.

What remains:

- Protected HBP/WAT/PASS parity is not green.
- First material remaining blocker is frost architecture, not a single scalar
  tweak. The one-day request/symbol-surface frost path cannot cleanly own
  persistent fine-layer liquid/frozen state, no-material no-op carry, coarse
  layer mutation, publication projection, and performance simultaneously.
- `day_input_and_helpers.rs` and `00_core_frames.rs` need mechanical splits
  before more direct-runtime expansion.

Follow-up objective:

1. Scaffold a new work package for a coupled stateful frost sub-solver with
   rich persistent lane state.
2. Define `DirectFrostLaneState` as the canonical owner of frost front scalars,
   fine-layer state, layer shadow state, no-material carry, solver timers, and
   publication-relevant state.
3. Replace hot-loop `DirectFrostRunoffSurface`/symbol-map authority with typed
   frost inputs:
   hourly winter forcing, prior snow/residue thermal context, layer topology,
   residual/field-capacity/porosity controls, liquid storage, and previous
   frost lane state.
4. Implement a stateful solver API shaped like:
   `advance_day(&mut DirectFrostLaneState, DirectFrostDayForcing,
   DirectFrostLayerInputs, DirectSnowRuntimeCarry) -> DirectFrostDayOutcome`.
5. Make the outcome explicitly separate:
   persistent carry, liquid delta, coarse layer mutation, downstream operands,
   hydrology projection, publication operands, and trace/proof fields.
6. Enforce a hard invariant: persistent fine/shadow carry never implies coarse
   layer projection. Coarse layer mutation is emitted only when material liquid
   exchange/frozen storage is explicitly closed.
7. Preserve the legacy ordering: frost sees prior snowpack, then snow partition
   mutates same-day snow state.
8. Rerun targeted unit/source-scan gates, then full H2637 direct-default and
   compatibility comparison. Required gates remain:
   `compatibility_edge_invocations=0`, HBP/WAT/PASS/loss/plot/manifest parity,
   independent operand reconstruction, and direct default `<=10x` legacy.

Current evidence anchors:

- Compatibility day-5 trace:
  `/tmp/r7g-cont-h2637/traces/frost28-compat-day5.jsonl`.
- Direct traces:
  `/tmp/r7g-cont-h2637/traces/frost28-direct-day5.jsonl`,
  `/tmp/r7g-cont-h2637/traces/frost29-direct-day5.jsonl`, and
  `/tmp/r7g-cont-h2637/traces/frost30-direct-day5.jsonl`.
- Latest measured manifest before the final consumer safeguard:
  `/tmp/r7g-cont-h2637/manifests/direct-default-frost30.json`.
- Retained compatibility capture:
  `/tmp/r7g-cont-h2637/capture/compat/`.
