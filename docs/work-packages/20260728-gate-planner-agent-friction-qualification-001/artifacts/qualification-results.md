# Qualification Results And Stop-Loss

Evidence class: Ran + Static.

## Disposition

`DELETE_ADVISORY_LINTER`

The final label-blind packet contained 18 valid paired cases and 36 plans. Two
independent scorers produced 498 obligation records and 54 deterministic
finding records. A third agent reconciled disagreements before labels were
restored. Blinding remained valid.

Retention required every threshold to pass. The recorded reduction produced
four failures:

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

The arm sequence was not interleaved: each participant completed three
manual-first pairs followed by three linter-first pairs. Timing and interaction
effects are therefore aliased with learning, fatigue, and sequence. They are
reported exactly because the protocol froze them, but they are not treated as
independent representative-work proof.

Every historical detached snapshot produced
`WP-IDENTITY-DETACHED-HEAD`. Both scorers marked all 18 instances
non-actionable because detachment was imposed by the frozen trial and could not
change the plan. The other 36 deterministic findings were actionable.
Accordingly, the 33.3% protocol-defined noise failure is dominated by an
artificial reconstruction condition and is not generalized beyond this trial.

The stop-loss was applied mechanically because the 24 linter-arm critical
omissions independently violate the zero-omission requirement. That basis does
not depend on timing, interaction, or detached-HEAD noise interpretation. The
command, source, focused tests, and tool README were deleted. Operative
guidance now points only to direct manual planning. No repair package or
modeling prerequisite was created.

## Bound Raw Evidence

The exact final files are tracked under `artifacts/evidence/`:

- `participant-p1.json`:
  `9d20c21dba61fc86421991d40a953983d4eeef2b9f2f02b949a6a75400f96514`
- `participant-p2.json`:
  `1a093281eb41ce4541aa7406513d2112457c4922da6d6db815cf6b2d9f2fbe11`
- `participant-p3.json`:
  `e7b1979eb9f1a31b5f68b8ec4179632a71d5b5592e72c74dd67241e1fe12f975`
- `blinded-cases.json`:
  `feab59fa0a95c77dc6228b6afa77481dd037f75d06746810151d6a0e7716a9ae`
- `scorer-c.json`:
  `7805140a841b7def899e52d2ce38ebd15e810e4f3ecfc6bcc9fdfe60f1ae5a0a`
- `scorer-d.json`:
  `cbea19bb84ed9ee62ac4d4d30411a0550c74fb5ea49ef7431e990348aaf3d231`
- `reconciled.json`:
  `8a553befd334207590b8668d87fa92dba9691869860b042d279227319f52a06c`
- `final-metrics.json`:
  `436cfa32b7675841f8c76f1c0d16275d7a57cefd8d6f431bf4b456257dd9162b`

`blinding-map.json` preserves the unblinding relation, and
`paired-results.csv` is the compact case-level reduction.
