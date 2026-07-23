# RTR-049 Empty History Restore Evidence

## Failure

Ran: automatic push run `29983039718` passed checkout, intent admission,
toolchain verification, dependency bootstrap, and planner build. It failed in
`Restore and verify newest durable attempt history`; gate execution was skipped
and the authenticated archive contains no TESTGATE node or attempt.

Ran: live runner inspection found one regular zero-byte
`/testgate-history/attempts.jsonl`. The prior automatic run's authenticated
archive also contains a valid zero-byte ledger. The helper reproducer rejected
the sole placeholder with `history restore destination is not empty`.

Static: defunct pre-pivot Omarchy runner records `29673299308`,
`29672334757`, and `29672149962` have zero jobs, artifacts, and logs and do
not exclude the current forest1 runner. They are historical non-blocking
metadata.

## Correction

Static: restore first validates the complete absolute durable-history directory
chain without following symlinks, then accepts only one existing entry with
exact name `attempts.jsonl`, regular-file type, and zero size. It installs the
independently verified source through the existing atomic no-follow copy
helper. Any nonempty ledger, extra entry, directory, leaf symlink, symlinked
history root, or symlinked ancestor remains rejected.

## Focused Validation

- Ran: all 25 `tests.python.test_testgate` cases passed.
- Ran: all 10 `testgate_ci_executor_contract` quick cases passed.
- Ran: all 11 `testgate_align_authority_contract` quick cases passed.
- Ran: `cargo fmt --all -- --check` passed.
- Ran: `git diff --check` passed.
- Ran: scoped `markdown-doc lint` passed six files with zero errors or warnings.

No expensive gate or manual TESTGATE dispatch ran.

## Review Finding Disposition

- Accepted: reviewer A demonstrated that the first correction followed a
  symlinked history root and could write an outside ledger. The correction now
  validates the full absolute directory chain and adds root/ancestor symlink
  regressions that assert no outside write.
- Accepted: reviewer A requested an exact zero-byte archive to zero-byte
  placeholder positive case. The success regression now exercises both empty
  and nonempty independently verified source ledgers.
- Passed: both independent reviewers rebound `PASS` with no remaining finding
  to exact correction commit `36327cb5...`.

RTR-049 closed in the append-only durable ledger at digest
`9b82798d0638c73f5d849ac927f64919815de5c56c8e82bf2ba23b7bf567beac`.
