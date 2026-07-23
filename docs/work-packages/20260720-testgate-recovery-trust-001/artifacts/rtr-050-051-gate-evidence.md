# RTR-050 And RTR-051 Gate Evidence

## Corrections

Static: RTR-050 passes the exact just-appended HEAVY `STARTED` entry digest into
resume discovery. Only that exact ledger record is excluded from prior-attempt
archive inspection. Older explicit recovery records still require complete
hosted provenance and fail closed when it is absent.

Static: RTR-051 sets `include-hidden-files: true` on all four TESTGATE evidence
publications: unsigned execution, authenticated recovery, independently
verified, and authenticated gate evidence. The source contract requires exactly
four bindings.

## Focused Validation

- Ran: the new current-STARTED exclusion regression passed.
- Ran: all 10 resume unit tests initially ran; eight passed and two exposed a
  test-loop bug where absent entry digests compared equal to an absent
  exclusion. After the condition required a present exclusion, only those two
  failed cases were rerun and both passed.
- Ran: all 10 `testgate_ci_executor_contract` quick cases passed.
- Ran: all 25 `tests.python.test_testgate` cases passed.
- Ran: `cargo fmt --all -- --check` and `git diff --check` passed.

Static: the retained run `29984179443` downloaded archive omitted exactly 20
indexed hidden files. No HEAVY node ran, no retry occurred, and no unchanged
expensive gate was launched.

## Review Finding Disposition

- Accepted: reviewer A found the first source contract did not bind the exact
  public call's sixth argument. The renewed exact-call assertion passed 10/10
  owning integration cases, and both reviewers rebound `PASS` to
  `999f0a0b...`.

RTR-050 closed at durable digest `9c1c5901...`; RTR-051 closed at durable digest
`8ba7eb97...`.
