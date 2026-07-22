# Characterization

Static: the extraction safety net combines the exact package-base binary-unit
baseline with retained real-consumer Attempt 15 evidence. No new or materially
changed test is required before decomposition.

Ran: the `62cb1086` baseline executed all five existing binary unit tests. These
lock option pair/cardinality handling, fail-closed dispatch, unauthenticated
standalone HEAVY rejection, transition output preflight/collision handling, and
confined atomic output behavior.

Ran: retained Attempt 15 at source HEAD
`a348488efa006652fca4eabbd6b2fd139d1f2fff` exercised the in-process transition
and HEAVY lifecycle through the real CLI consumer. Its receipt
`f60dcdb651226a2f6b136fd92db9ebb9bd09d1b6340e40d1754dd45c827ed4df`
records:

- LIGHT `PASS` with stage receipt
  `4cadc9eef3c7706f3ebc452f39010e5ccdb238d896273f2bf565e0f9b9507e36`;
- a READY audit with all ten checks passing and audit ID
  `22a6eab55269c507de9cad8e9cc205a1a2f704f7f355a1a52f7d0b14586088cb`;
- HEAVY execution with 14 pass, one CRAP fail, zero blocked/skipped/retried;
- an authoritative FAIL receipt, successful source-mutation check, and no
  resume decisions.

The retained failure is the CQR discovery event, not a transition/lifecycle
failure. This package will not repeat Attempt 15. The master ExecPlan owns one
changed-head TESTGATE after all seven CQR packages close.
