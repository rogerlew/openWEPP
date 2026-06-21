# Verification A

Evidence mode: Static + Ran.

Ran:

- `cargo fmt --check` -> PASS.
- `cargo test -p openwepp-runner r6d_cutover_candidate_fails_closed_after_retained_direct_publication -- --nocapture` -> PASS.
- `cargo test -p openwepp-runner r6b_absent_operand_detector_suppresses_marker_for_nonzero_direct_operands -- --nocapture` -> PASS.

Verified:

- focused cutover failure marker is
  `HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`;
- no direct runtime audit counters are incremented on the cutover failure path;
- all required public outputs remain absent when cutover fails;
- the all-zero detector no longer conflates climate-only retained rows with
  zero-only publication rows.
