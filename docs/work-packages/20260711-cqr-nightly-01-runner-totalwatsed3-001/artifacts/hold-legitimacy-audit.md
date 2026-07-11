# Hold Legitimacy Audit

Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-COVERAGE-PRECONDITION`.

## Boundary And Evidence

The CQR hard precondition requires coverage closure before decomposition. The
target begins at `667/992` (`67.238%`) lines, below the glue-tier `85%` floor;
the available LCOV has no region counts or per-function region-floor map.
After the attempt, eligible `code` and optional-input date-key functions remain
above CRAP `30`, so reviewer-approved exclusion cannot close them.

## Attempted In-Envelope Route

The package extracted typed column views and row/value readers, preserved exact
fallible-read/mutation/numeric ordering, passed six focused tests, and reduced
`read_wat_batch` from `65.469` to `4.002`. Independent review nevertheless
proved the attempt was sequenced before its required coverage safety net.

## Why CQR Cannot Close Safely Now

Closing the prerequisite requires a dedicated valid/invalid Parquet matrix for
the 1295-line aggregation module: all error variants, column type/null/value and
first-error ordering, optional soil/element inputs, region thresholds, and
per-function floors. That is a coherent module-test-enhancement package, not a
small characterization addition that can be safely hidden inside this nightly
decomposition after implementation has begun.

## Rollback Proof

Ran: the target-only attempt was reversed. `git diff --exit-code e2ff321e --
crates/openwepp-runner/src/totalwatsed3.rs` exits `0`. No test was edited.

## First Actionable Follow-On

Create the dedicated ADR-0021 module-test-enhancement package described in
`worker-handoff.md`, reach coverage closure, then rerun this CQR decomposition.
