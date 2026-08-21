# HOLD legitimacy audit

Status: `LEGITIMATE EXECUTED HOLD`.

Defect: `STAGE3-V11-COVERED-CONSUMER-001`.

Observed evidence:

- `DirectV11RealConsumerStack::construct_interval_envelope_with_duration`
  rejects `snow_present_at_beginning`, `snow_present_at_end`, and
  `snow_terminal_payload_present`.
- `DirectV11SnowStage3OwnerExecutor` is an actual typed executor for the
  snow-free remainder and requires a terminal-handoff request; it is not a
  snow-covered V11 executor.
- `snow_stage3_terminal_handoff.rs::evaluate_shared_carrier` contains the
  Child 2C equations, but the new attachment does not derive its operands from
  current staged V11/Stage-3 owners or invoke it.
- The runner's only Stage-3 persistent state is now explicitly named
  `snow_stage3_historical_evaluation_state`; no sealed 48-support prepared
  capability and new attachment constructor are present in the runner.

In-envelope routes considered:

1. Reuse the existing real consumer unchanged: rejected because it would run
   the snow-free ground/LSE branch over snow, explicitly forbidden by the
   kickoff contract.
2. Reuse the old day-frame/shared-carrier handoff: rejected because it uses
   completed production-frame/live summaries and caller-built handoff state,
   which the package expressly forbids as constitutive custody.
3. Treat the new typed attachment skeleton and typed parcel/owner projections
   as closure: rejected because producer-only, skeleton-only, or shadow-only
   evidence cannot close the real downstream consumer.

The remaining route requires a new typed snow-covered lower-boundary executor
and runner ownership/capability integration. No proxy physics, guessed
conductance, duplicated hourly forcing, or silent fallback is authorized.
The current package therefore holds after safe fail-closed implementation and
focused validation, with first lift `close defect
STAGE3-V11-COVERED-CONSUMER-001`.
