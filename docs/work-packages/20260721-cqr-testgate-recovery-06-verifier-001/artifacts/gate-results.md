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
one new changed-head metric remained pending before renewed review.

Ran: the corrected terminal-audit test first failed closed at canonical package
admission because the scaffold's split-file bullet contained trailing prose and
was not part of the parsed base write set. The declaration was corrected alone
at `05baef7f`; the subsequent test/package increment now requires exactly one
package authority. No admission bypass or unchanged metric rerun was used.

Ran: focused terminal READY-audit characterization passed at clean
`223b034e`. One new corrected-head matching-module traversal passed 138 tests,
87.2557% line, 85.8841% region, all per-function floors, and all CRAP bounds.
No HEAVY gate or TESTGATE ran. Renewed dual review then proceeded.

Static: renewed Review B passed. Renewed Review A accepted all technical and
metric evidence and requested only two stale documentation corrections, now
applied. At that point, final Review A docs-only re-audit remained pending.

Static: final Review A and Review B both pass. RTR-028 is durably CLOSED with
ledger entry SHA-256
`f1083bfd8674033b535405ed534d0f7ae6cfe923191a10115a0d0cd4c8411f9b`.
Dual terminal verification is the only remaining rank-6 gate.
