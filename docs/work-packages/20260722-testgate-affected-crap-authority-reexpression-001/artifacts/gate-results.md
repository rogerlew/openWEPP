# Gate Results

Ran: Cargo production/reverse classification plus root-global, ordinary affected,
and graph-union planner regressions pass 4/4 in 70.482 seconds.

Ran: test-only/out-of-tree global escalation plus the four re-expressed
stage/pass/fail/mutation fixture regressions pass 5/5 in 258.946 seconds.

Ran: the checker byte-sealing increment passes Python 30/30, integration 19/19,
Bash syntax, and target Clippy. No coverage acquisition or HEAVY gate ran.

Ran: package audit at `04f3b619` against clean pre-edit base `afc83394` is
`READY` with zero unauthorized paths; audit ID `2a4762fa...cada2`.

Ran: the first changed-head owning target reached 146/154 with 146 PASS and 2
skipped before the exact public-audit reconstruction test timed out at 600.312
seconds; seven cases were canceled. Total wall was 884.663 seconds. RTR-041 is
open and no unchanged rerun occurred.

Static: RTR-041 removes filesystem canonicalization for every Cargo target on
each reconstructed graph load. Locked metadata already supplies absolute paths;
lexical strip-prefix confinement and the exact target-kind/source predicates
remain unchanged.

Ran: at changed HEAD `2010fc5a`, the exact previously timed-out public-audit
reconstruction test passes 1/1 in 476.228 seconds, below its unchanged 600-second
ceiling. No unchanged rerun occurred.
