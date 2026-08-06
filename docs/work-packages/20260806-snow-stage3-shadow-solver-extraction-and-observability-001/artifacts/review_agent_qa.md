# QA And Consumer Review

Evidence class: `Static + Ran` against committed producer head `19bd7aa8`.

Disposition: `HOLD` before remediation.

Consumer QA found that the original test formatted only an isolated evaluation
fragment, read a subset of fields, reconstructed no sequential identity, and
did not reject CoE melt, producer residual, row-count, nonzero-hour, or ground
aliases. It also confirmed the partial-hour energy defect, absent closure
guards, incomplete fingerprints, post-exhaustion support zeros, and the public
`complete_carrier_shadow` field removal.

Ran: the original runtime suite passed `22/22`, confirming that the accepted
tests missed these failure modes.

Final re-review evidence class: `Static + Ran` against exact clean commit
`6506da5d4b917c676683613d68e0556d467fed30`.

Disposition: `GO`; no residual findings remain.

Consumer QA confirmed exact exhaustive shapes for all four legacy exported
types, additive evaluation isolation, complete paired and truncated-sequential
schema-v5 reconstruction, intact schema-v4 golden compatibility, and exact
enabled/disabled bytes from the real WAT, HBP, and PASS writers. Real-consumer
tests passed `2/2`, protected-output parity `1/1`, and runtime plus contract
tests `32/32`. The worktree remained clean.
