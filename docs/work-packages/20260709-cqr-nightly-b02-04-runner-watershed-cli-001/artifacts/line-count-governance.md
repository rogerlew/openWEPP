# Line-Count Governance

The unaccepted attempt grew the target from `2263` to `2614` physical lines.
It remains below the `3000`-line blocker but is above the `2000`-line WARN.
Because the attempt is rolled back, no new line-count burden is accepted.

The dedicated follow-on must introduce a cohesive CLI testability boundary or
fixture harness before a future CQR decomposition; it must not add another
large inline test block to this already-warning-sized executable.
