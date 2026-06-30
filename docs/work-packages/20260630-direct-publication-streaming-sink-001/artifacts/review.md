# Review

Evidence class: Static

No blocking findings.

Notes:

- The production path no longer retains publication rows, but the direct
  runtime still exposes retained capture for tests and diagnostic callers. That
  is intentional and outside this package's production RSS gate.
- Full-output H2637 RSS is not as low as required-only because output writing
  still does real parquet/file work. The relevant retained-row slope is removed:
  required-only H2637 is close to the longer-day W9 single-OFE case despite
  emitting about `14.4x` as many rows.
- The package does not resume typed setup/symbol-map carrier deletion; that
  remains the separate compatibility-deletion follow-on.
