# Aggregate Gate Evidence

Ran: source receipt `64a6f2926d41f2805f1f94fb83ad90d95940a3603e6f2ea8207b5f9bfe026b44`
sealed 14 PASS / 1 FAIL / 0 retries. Ordinary Nextest passed 2,290/2,290 in
1,014.144 seconds. Instrumented Nextest passed 2,290/2,290 in 806.407 seconds;
CRAP adjudication exposed four actionable rows across two modules.

Ran: B03S-1 completed at `0ff8f3407732ecd5fd178e9181a79cb8f15f2883`.
Three targets and five helpers have CRAP 2–4 and 100% focused coverage; focused
binary/integration, Clippy, formatting, docs, diff, dual review, and dual
verification passed.

Ran: B03S-2 completed at `c85c1a15d9b8fdd63f328a125bde345f898ad444`.
The target and three helpers have CRAP 4–5 and 100% focused coverage; focused
tests, Clippy, formatting, docs, diff, dual review, and dual verification
passed.

Ran: RTR-045 closed at `4181e914d07e660ae376ec50feefacd5184f689e`;
append-only superseding CLOSED digest
`b4ab096a3469b04238ca9c5c2e7005115475904b2ad642b4abbd01bf592871a3`
binds the exact commit. Python TESTGATE tests passed 22/22 and dual review plus
dual verification passed.

Static: no unchanged TESTGATE, HEAVY, global Nextest, or global CRAP rerun was
launched during module correction.

Ran: the sole changed-head qualification at exact HEAD
`eadc01459df18e83d94362dc225219232f0a4c65` completed in
`/home/workdir/testgate-recovery-trust-01-final-b03s.oJ1TCz`. Intent plan
`c403fc60...f7b68`, terminal plan `8bd2c1ed...8c684`, and READY audit
`35729c88...009b8` bound the execution. Receipt
`c22fe3f57bb179e62f8460f4acc933c02653811c866bf575eeefed58217f06ca`
sealed 15/15 PASS with zero failures, blocked nodes, or retries. Ordinary and
instrumented Nextest passed 2,293/2,293 in 1,026.563 and 795.488 seconds.
Fresh global CRAP passed with `closure_eligible=true`, zero actionable rows,
two valid adjudications, and zero invalid adjudications. Source mutation was
unchanged. The durable 151-record ledger closed PASS at
`2096272c6a1fe5a5d5d18095a895be070eb688408a2abb6d2f9f42dd5b3b067b`.

Static: the unsigned receipt remains `LOCAL_UNTRUSTED`, and
`observation.json` records `PENDING_GITHUB_ATTESTATION`. The canonical trust
contract forbids this evidence from closing an `INCREMENT` boundary until a
native repository-reviewed attestation envelope for the exact receipt and HEAD
verifies. The earlier absolute-package-path invocation was rejected before
planning, node execution, or ledger mutation; correcting it to the required
repository-relative syntax did not repeat an expensive gate.
