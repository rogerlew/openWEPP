# Data-Path Proof

Evidence mode: Static.

## Pre-Change Proof

R6B current state:

- `build_direct_publication_artifacts` constructs `DirectRunFrame::skeleton`.
- The runner seeds only lane geometry and calendar metadata before
  `DirectFrameExecutor::run_publication_capture`.
- The captured publication rows therefore contain zero/default direct operands.
- The fail-closed HBP gate reports
  `R6B-DIRECT-PUBLICATION-TYPED-OPERANDS-ABSENT`.
- Cutover manifest production remains compatibility-owned.

## R6C Target

Accepted cutover data path:

`parsed inputs + accepted direct run/lane/day operands -> typed bridge ->
DirectRunPublicationFrame -> direct HBP/WAT/PASS/loss/manifest projections ->
parity gates -> public writes`

Forbidden data path:

`compatibility WB13/runtime/writeback/stale logical state -> wrapper ->
DirectRunPublicationFrame -> public writes`

## Post-Change Proof

Ran:

- `cargo test -p openwepp-runner r6_cutover_candidate_fails_closed_before_skeleton_publication_capture -- --nocapture`
- `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture`
- `cargo test -p openwepp-runner r6a_direct_publication_frame_shadow_runs_without_skeleton_counter -- --nocapture`

Result:

- `DirectPublicationFrameCutover` now returns
  `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT` before
  `DirectRunFrame::skeleton`.
- The cutover focused test observes zero run-frame constructions, zero executor
  constructions, zero skeleton runs, zero publication-capture runs, and zero
  compatibility-edge invocations on the direct-publication failure path.
- The CLI cutover contract still fails closed before HBP/loss/WAT/PASS/manifest
  writes.
- `DirectPublicationFrameShadow` still runs the existing R6A shadow scaffold.

Remaining blocked path:

`production climate lifecycle -> retained direct day/publication producers`
does not exist. The scheduler lifecycle returns compatibility publication
surfaces only.
