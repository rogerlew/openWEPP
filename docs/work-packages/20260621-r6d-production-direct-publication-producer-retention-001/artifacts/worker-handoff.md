# Worker Handoff

Status: executed-held.
Evidence mode: Static + Ran.

Target: lift `HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`.

Superseded by R6E disposition:
`HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

R6D lifted `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT` for cutover.
The production climate lifecycle now retains a `DirectRunPublicationFrame` for
`DirectPublicationFrameCutover`, sourced from parsed climate/calendar and slope
geometry. Cutover consumes that retained frame and does not call skeleton direct
frame construction or post-hoc publication capture.

First actionable item after R6E: close contract-backed HBP direct process
parity. R6E already binds parsed climate into typed direct publication day
inputs and builds retained cutover publication through direct executor capture.
Keep cutover fail-closed until each output family has anti-alias and
independent reconstruction evidence.

Do not:

- populate missing direct publication fields from `SimulationOwnedWb13Row`;
- read compatibility `HillslopeWritebackSurface` publication values as direct
  authority;
- wrap `KernelWritebackPayload`, stale logical state, or compatibility rows in
  direct-named structures;
- accept climate/calendar/geometry row retention as public-output cutover;
- claim full R6 closure while any touched `.rs` file remains above the 3000-line
  governance threshold without a documented split/exception.

Reuse tests:

- `cargo test -p openwepp-runner r6e_cutover_candidate_reaches_direct_input_binding_then_fails_hbp_parity -- --nocapture`
- `cargo test -p openwepp-runner r6_direct_publication_cutover_cli_flag_reaches_direct_binding_then_fails_hbp_parity --test r6_direct_publication_cutover_cli_contract -- --nocapture`
- `cargo test -p openwepp-runner r6b_absent_operand_detector_suppresses_marker_for_nonzero_direct_operands -- --nocapture`
