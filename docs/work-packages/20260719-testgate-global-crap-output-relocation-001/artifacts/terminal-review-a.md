# Terminal Review A

Evidence class: `Ran` and `Static`.

Verdict: `PASS`, no open findings after disposition.

Reviewer A independently verified plan and execution-key identity, receipt
bytes, exact planned/executed inventory at 2,188 identities, all 12 DAG nodes,
attempts, and artifact hashes. Both ordinary and coverage JUnits pass
2,170/2,170 with five configured skips and 25/25 publication cases.

Ordinary publication overlap peaks at four. Coverage has four isolated nominal
five-case boundary intervals of exactly 1 ms because JUnit starts and durations
are millisecond-rounded; sustained overlap above four is zero. The former
timeouts pass at `263.426s` and `153.927s` under coverage.

CRAP control strictly parses as PASS/exit zero/fresh and binds report SHA-256
`57b886a2...` to both retained and published report bytes. Threshold remains
`30.0`; outcome is 2 raw / 2 adjudicated / 0 actionable. Source mutation and
all coverage manifests are unchanged.

The accepted finding corrected terminal evidence from one nominal 1 ms
boundary interval to the observed four before final PASS.
