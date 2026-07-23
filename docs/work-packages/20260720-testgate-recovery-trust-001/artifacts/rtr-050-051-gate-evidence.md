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
