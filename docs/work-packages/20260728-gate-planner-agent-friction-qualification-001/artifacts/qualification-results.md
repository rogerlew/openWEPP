# Qualification Results And Stop-Loss

Evidence class: Ran + Static.

## Disposition

`DELETE_ADVISORY_LINTER`

The final label-blind packet contained 18 valid paired cases and 36 plans. Two
independent scorers produced 498 obligation records and 54 deterministic
finding records. A third agent reconciled disagreements before labels were
restored. Blinding remained valid.

Retention required every threshold to pass. Four failed:

| Metric | Result | Required | Disposition |
| --- | ---: | ---: | --- |
| Reviewer-confirmed critical omissions, linter arm | 24 | 0 | FAIL |
| Non-actionable deterministic findings | 18 / 54 (33.3%) | at most 10% | FAIL |
| Adjusted median planning-time reduction | 26.8% | at least 30% | FAIL |
| Median interaction reduction | 0% | at least 50% | FAIL |
| Maximum cold latency | 1.273 s | at most 15 s | PASS |
| Maximum warm latency | 0.912 s | at most 5 s | PASS |
| Non-test production lines | 1,011 | at most 3,000 | PASS |
| Tool-originated lifecycle/write/execute violations | 0 | 0 | PASS |
| Forbidden control-plane machinery | 0 | 0 | PASS |

Manual median planning time was 10.654 seconds. Linter-arm median planning time
was 7.801 seconds. No scored linter-originated maintenance or interruption
time was added. The deterministic 10,000-resample bootstrap interval for the
planning-time reduction was -20.1% to 50.5%; the point estimate missed the
prospectively frozen threshold. Manual and linter-arm median interactions were
both 1.0; the bootstrap interval for interaction reduction was exactly 0%.

Every historical detached snapshot produced
`WP-IDENTITY-DETACHED-HEAD`. Both scorers marked all 18 instances
non-actionable because detachment was imposed by the frozen trial and could not
change the plan. The other 36 deterministic findings were actionable.

The stop-loss was applied mechanically. The command, source, focused tests,
and tool README were deleted. Operative guidance now points only to direct
manual planning. No repair package or modeling prerequisite was created.

## Bound Raw Evidence

Raw files remain ignored under `target/order5-qualification/`:

- P1 valid rerun:
  `9d20c21dba61fc86421991d40a953983d4eeef2b9f2f02b949a6a75400f96514`
- P2 valid rerun:
  `1a093281eb41ce4541aa7406513d2112457c4922da6d6db815cf6b2d9f2fbe11`
- P3 valid run:
  `e7b1979eb9f1a31b5f68b8ec4179632a71d5b5592e72c74dd67241e1fe12f975`
- final blinded packet:
  `feab59fa0a95c77dc6228b6afa77481dd037f75d06746810151d6a0e7716a9ae`
- scorer C:
  `7805140a841b7def899e52d2ce38ebd15e810e4f3ecfc6bcc9fdfe60f1ae5a0a`
- scorer D:
  `cbea19bb84ed9ee62ac4d4d30411a0550c74fb5ea49ef7431e990348aaf3d231`
- blind reconciliation:
  `8a553befd334207590b8668d87fa92dba9691869860b042d279227319f52a06c`
- unblinded metric reduction:
  `436cfa32b7675841f8c76f1c0d16275d7a57cefd8d6f431bf4b456257dd9162b`

`paired-results.csv` is the tracked compact case-level reduction.
