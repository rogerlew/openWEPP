# Real consumer path proof

Status: intake path frozen / implementation evidence pending

Evidence mode: Static

Producer: `Wb11HydrologyKernel::evaluate_stage3_persistent_day_with_terminal_event`
returns `DirectSnowStage3PersistentDayResult` with
`DirectSnowTerminalEventResult`; the solver localizes the event in
`stage3_solver/terminal_event.rs`, and `evaluation.rs` censors later snow time.

Current runner handoff: `00a_snow_frost_authority_impl.rs` evaluates persistent
Stage 3 only after the authoritative CoE partition. The day builder in
`00c_day_input_builder_impl.rs` sends only `snow_evaluation.authoritative`
(CoE) to production hydrology and advances Stage 3 through an external
`RefCell`. No Stage 3 terminal liquid reaches a receiving owner today.

Receiving owner path: `DirectFrameExecutor` performs the real publication day
and invokes `DirectV10RealConsumerShadow::execute_prepared_gsi_day` on a clone.
The V10/V9 consumer executes 48 intervals through
`execute_v8_lse_runtime_shadow_internal`, the surface-liquid ingress owner,
real hydrology, and one complete-owner commit.

Implementation acceptance must show one new coupled candidate owns Stage 3 and
the V10 owner set, joins event support to the correct half-hour, transfers the
terminal parcel once, executes the actual receiver for only remaining support,
and commits once. Negative proof must show production continues using CoE,
while skeletons, snowbench, trace-only consumers, caller-supplied physics,
external Stage 3 cells, and old full-interval snow-free paths do not carry the
mechanical handoff claim.
