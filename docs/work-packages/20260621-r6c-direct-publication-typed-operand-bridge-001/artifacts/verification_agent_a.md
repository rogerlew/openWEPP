# Verification A

Evidence mode: Static + Ran.

Verified commands:

- `cargo test -p openwepp-runner r6_cutover_candidate_fails_closed_before_skeleton_publication_capture -- --nocapture`
- `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture`

Result: PASS.

Verified behavior:

- cutover error contains
  `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`;
- no direct frame/executor/publication capture is constructed on the failure
  path;
- no public outputs are written.
