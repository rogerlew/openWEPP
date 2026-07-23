# Runner Role And Closeout Correction

Evidence classes: Static + Ran.

## Corrected runner model

Static: the protected TESTGATE execution path is intentionally split by role:

- `execute-increment` runs TESTGATE, including HEAVY, on the trusted
  self-hosted forest1 runner;
- `verify-increment` performs bounded independent verification on a
  GitHub-hosted runner and does not execute HEAVY; and
- `increment-gates` consumes verified immutable evidence and performs the
  minimal repository attestation and authority checks without running
  candidate gate code.

Static: the three provider records previously described only as "defunct
self-hosted runners" belong to the retired pre-pivot Omarchy runner. They do
not describe forest1 availability.

## Corrected attempt record

Ran: GitHub run `30002884134` targeted exact pushed head
`aeddd4b4be322dd706f9dd311c0a8961a9bfbb36`. Its execution job ran on
`forest1-openwepp-01` with labels `self-hosted`, `Linux`, `X64`, `openwepp`,
`forest1`, and `trusted`.

Ran: the forest1 job passed setup, checkout, trusted comparison admission,
pinned toolchain verification, dependency bootstrap, planner build,
durable-history restoration, and the superseded-head check. It was canceled
during `Execute content-verifiable increment gates`. Finalization and unsigned
evidence upload completed.

Ran: the GitHub-hosted `verify-increment` job downloaded and re-ingested the
evidence, built and verified the recovery provenance, then failed closed at
`Fail closed when execution did not succeed`. The aggregate job also failed
closed. The run therefore produced no passing repository attestation.

## Closeout interpretation

Static: the earlier statement that hosted attestation was unavailable because
the self-hosted runner was defunct is superseded. Forest1 was active; the
attestation was absent because the forest1 execution was canceled before a
successful receipt could reach the hosted verifier.

Static: the operator-accepted exception closes the implementation, review, and
documentation work package using the retained exact-head comparator receipt
and dual terminal verification. It does not promote that
`LOCAL_UNTRUSTED` receipt, certify an `INCREMENT`, campaign, or release trust
boundary, or claim that GitHub-hosted infrastructure should execute HEAVY.
No unchanged expensive rerun is authorized by this correction.
