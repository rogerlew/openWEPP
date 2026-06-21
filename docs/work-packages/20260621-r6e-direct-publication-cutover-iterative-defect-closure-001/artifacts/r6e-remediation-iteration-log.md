# R6E Remediation Iteration Log

Evidence mode: Static + Ran.

Status: executed-held.

## Iteration 1 - Reproduce R6D Hold

- Blocker: R6E-B001.
- Mechanism: cutover consumed retained direct publication rows with only parsed
  climate/calendar/geometry authority.
- Evidence: focused unit and CLI tests failed closed before output writes.
- Decision: retain fail-closed behavior; reduce marker to a specific root
  cause.

## Iteration 2 - Resolve Line-Count Gate

- Blocker: R6E-B002.
- Candidate patch: move direct-publication helper block from
  `00_runner_intake_and_lane_setup.rs` to `04_direct_publication.rs`.
- Line-count impact after final R6E edits:
  - `00_runner_intake_and_lane_setup.rs`: `2787` lines;
  - `04_direct_publication.rs`: `376` lines.
- Decision: retain; touched hard-threshold runner file is below 3000 lines.

## Iteration 3 - Refine Fail-Closed Marker

- Blocker: R6E-B003.
- Candidate patch: change the cutover gate marker from broad R6D parity-grade
  producer absence to
  `HOLD-R6E-PRODUCTION-DIRECT-RUNTIME-INPUT-BINDING-ABSENT`.
- Decision: initial refinement retained, but not accepted as terminal because
  B003 was in-envelope and implementable.

## Iteration 4 - Resolve Production Direct Input Binding

- Blocker: R6E-B003.
- Candidate patch:
  - add `DirectPublicationDayInput`;
  - add `DirectFrameExecutor::run_publication_capture_with_day_inputs`;
  - bind parsed precipitation/effective temperature into direct day frames;
  - build retained cutover execution through direct capture;
  - remove the hand-authored retained-row producer from the compatibility loop.
- Focused tests:
  - `cargo test -p openwepp-runner r6e_cutover_candidate_reaches_direct_input_binding_then_fails_hbp_parity -- --nocapture`;
  - `cargo test -p openwepp-runner r6_direct_publication_cutover_cli_flag_reaches_direct_binding_then_fails_hbp_parity --test r6_direct_publication_cutover_cli_contract -- --nocapture`.
- Decision: retain; B003 resolved.

## Iteration 5 - Identify Next Blocker

- Blocker: R6E-B005.
- Evidence: direct CLI cutover reaches HBP comparison and fails at
  `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`; HBP byte lengths are both
  `1654`, but bytes differ; no outputs are written.
- Decision: hold at direct process parity. Closing this requires
  contract-backed parity-grade direct process migration, not output-writer
  plumbing or compatibility authority aliasing.
