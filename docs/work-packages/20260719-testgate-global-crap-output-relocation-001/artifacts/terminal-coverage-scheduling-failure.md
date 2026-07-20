# Terminal Coverage Scheduling Failure

Ran: exact committed terminal plan
`5fb26a8baf96d9977b1788e899dc5de3452a881c434a385900007f204ba93cab`
from base `450c40a75e11e5197203063b27232874652174ac` to head
`a33fafe67200eed0edee406f33cbeecf8cb44fa4`.

- Two separately invoked plans were byte-identical and independent
  reconciliation passed before execution.
- The normal full-workspace node passed.
- Fresh LLVM coverage selected 2,169 tests: 2,167 passed, two timed out, and five
  configured cases were skipped.
- Both timeouts were in `assurance_v2_publication_contract` at the unchanged
  720-second limit:
  `authority_lifecycle_and_bound_byte_negative_matrix_is_fail_closed` and
  `bootstrap_narrative_empty_directory_and_symlink_drift_fail_closed`.
- Coverage JUnit/log evidence shows many cases in the same subprocess-heavy
  binary running concurrently for 9--12 minutes. Both timed-out cases passed in
  the normal full-workspace node.
- The adapter truthfully wrote `result: FAIL`, `exit_status: 100`, and a null
  report digest because coverage acquisition did not complete. The executor
  emitted receipt
  `93f4394036906302b8df42ca7bedab09cbef5de6ce8af9235338a7aad645999c`
  with 11 passed nodes and one failed node.

Disposition: reproduced integrated tooling defect
`TESTGATE-CRAP-COVERAGE-SCHEDULING-01`. Bound the complete publication contract
binary through canonical Nextest scheduling. Preserve exact inventory,
coverage, timeout, CRAP thresholds, and adjudications; do not resume this stale
plan.
