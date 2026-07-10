# Hold Legitimacy Audit

Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-COVERAGE-TESTABILITY`.

## Exact blocker

ADR-0021 closure cannot be met safely inside this CQR package. The real
29-test watershed CLI suite passes, yet the isolated source measurement is
74.378% lines and 36.451% regions, with 33 production functions below the 75%
floor. One row also remains above CRAP 30.

## Attempted in-envelope route

Private structural extraction preserved CLI/runfile/manifest order and exact
hard-fail behavior. It passed `cargo fmt --check`, focused bin clippy,
`cargo check`, `git diff --check`, and the 29-test public behavior suite. The
after metrics in `coverage-attempt.md` and `crap-attempt.md` prove that existing
real-consumer tests do not characterize enough of the extracted error/guard
branches to close this module.

## Why CQR cannot safely close it

The uncovered paths span command dispatch, TOML input grammar, source-relative
paths, sidecar/groundwater authority, HBP/manifest consumption, publication,
MOFE carry, and topology. Raising all 33 functions and 5100 regions to the
required floors would require a new fixture/CLI-driver testability architecture
and a broad matrix of invalid and valid runfile/manifest cases. That is a
material new test-harness surface, not target-local characterization, and would
increase a 2263-line executable already at the repository WARN threshold.

## Rollback proof

Only this package's uncommitted target source modification is reverted to its
scaffold commit `14514044`. No test or production change is accepted. After
rollback, `git diff --exit-code 14514044 --
crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` exits 0; package
evidence remains for the follow-on.

## First actionable follow-on

Create a new authorized watershed-CLI testability/characterization package that
introduces a cohesive reusable fixture/driver boundary, binds every CLI/runfile
manifest hard-fail branch to real CLI consumption, reaches ADR-0021 coverage,
then repeats this CQR target. It must preserve `SC-SYSTEM-001`, `SC-ROUTE-001`,
and `SC-GWBASEFLOW-001` behavior and remain separate from this hold.
