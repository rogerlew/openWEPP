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
