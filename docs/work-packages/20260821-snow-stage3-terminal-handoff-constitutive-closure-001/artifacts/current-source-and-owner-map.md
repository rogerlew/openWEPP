# Current source and owner map

Status: `IMPLEMENTATION AUDIT COMPLETE; CLOSURE BLOCKED`.

`Static:` Current Stage 3 persistence authority is
`crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`:
`DirectSnowStage3PersistentState` carries layers, detached retained liquid,
interval order, cumulative mass/energy ledgers, model identity, and a state
fingerprint. `DirectSnowStage3PersistentDayResult` carries the accepted
ending state, mass/energy closure, unresolved liquid, and terminal result.
The actual sequential transition is in
`hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`;
the actual adaptive terminal enthalpy implementation is in
`stage3_solver/terminal_event.rs`, reached through
`Wb11HydrologyKernel::solve_terminal_enthalpy_event` from the sequential
transition. The current public entry point is day-shaped and loops 24 fixed
3,600-second rows.

`Static:` The selected closure must extract that same transition over a typed
1,800-second support. The current `DirectSnowStage3ShadowAttachment` instead
constructs `TerminalStateRates`, uses configured event day/lane/tick and live
day-frame summaries, and stages after a completed production day. Those are
closure defects, not accepted physical inputs.

`Static:` Runner custody currently lives in
`openwepp-runner/.../00_builders_and_authority.rs` and
`00c_day_input_builder_impl.rs`. A `RefCell<Vec<Option<...>>>` is passed into
day-input construction, and the builder installs `persistent_next_state`
before the scheduler transaction. This is the frozen custody defect. The
attachment, not the day-input builder, must own accepted Stage 3 state.

`Static:` The actual V11/LSE owner surfaces are
`v11_vegetation_consumer.rs` and `v9_real_consumer_shadow.rs`:
`DirectV11RealConsumerStack` imports the actual V10 stack and stages typed V11,
LSE, hydrology, BGC, soil-thermal, surface-liquid, GSI/provider, and root
hydraulic receipts. The existing stack is snow-free-boundary oriented and
cannot be invoked unchanged for snow-covered ground. A typed snow-covered
boundary adapter is required.

`Static:` Current ordinary scheduler call sites are
`direct_runtime/03_executor.rs` lines 875 and 905, which call
`stage_snow_stage3_shadow` and `commit_snow_stage3_shadow` around publication.
The invocation point can remain, but the attachment must execute its own
ordered 48-parent shadow chronology and cannot use `DirectDayFrame` as its
physical beginning state.

`Static:` The new typed attachment is
`crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment.rs` and
the new scheduler bridge is
`direct_runtime/snow_stage3_v11_scheduler.rs`. It owns a cloned candidate,
Stage-3 state by lane, a V11 parent, a coupled clock, event ordinal, and typed
owner-byte receipt chain. The runner-side RefCell is now explicitly named
`snow_stage3_historical_evaluation_state`; it remains a compatibility surface,
not the constitutive attachment owner.

`Static:` The current released `DirectV11RealConsumerStack` rejects
`snow_present_at_beginning`, `snow_present_at_end`, and terminal snow payloads.
`DirectV11SnowStage3OwnerExecutor` is the existing actual owner executor for a
snow-free remainder only. No current consumer invokes Child 2C's
`evaluate_shared_carrier` from the new attachment, and no runner builder emits
the required sealed 48-support V11/coupled-owner capability. This is the exact
unclosed owner/consumer boundary.

`Static:` Surface-liquid state has configuration-bound canonical bytes and a
typed continuation/ingress API in `direct_runtime/surface_liquid_owner.rs`.
The current attachment incorrectly limits receiver selection to one record;
the closure path must use the declared OFE/tile topology and exact area split.

`Ran:` the focused current-contract guard profile passed 56/56 after the
assertion census. No full workspace gate has yet been run in this phase.
