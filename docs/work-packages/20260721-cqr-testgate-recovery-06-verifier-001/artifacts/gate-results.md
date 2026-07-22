# Gate Results

Ran: focused characterization passed at exact clean HEAD `9970ac32`.

Ran: the authoritative matching-module measurement passed 138 tests with no
failures, closed 85/85 aggregate coverage, the 75% function floor, and CRAP at
most 30. `cargo fmt --all -- --check` and `git diff --check` passed before the
test commits. No HEAVY gate or TESTGATE was selected or run inside rank 6.

Static: dual implementation review found a shared terminal-audit oracle gap and
one full-verdict/message exactness gap at `9970ac32`. Both are corrected in the
next test-only increment. The `9970ac32` metric remains truthful historical
evidence but cannot close the corrected head; focused validation and exactly
one new changed-head metric remain pending before renewed review.
