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

Ran: at exact clean correction HEAD `85d706ed4fefc2011bd23c56f60688a7ba5e63ba`,
`cargo nextest run -p openwepp-gate-planner --no-fail-fast` passes 155/155
with two skipped in 1,862.964 seconds. The retained log is
`/home/workdir/testgate-history/rtr035-041-planner-nextest-85d706ed.log`,
SHA-256 `16dce4671f22253836c592e0e1bd0d7e214270816ee0a9f0ee0bd1305f23c18f`.
The exact reconstruction case completed below its unchanged 600-second limit.

Ran: final focused Cargo-graph classification passes 2/2; formatting, diff
hygiene, and planner all-target Clippy with warnings denied pass.

Ran: exact package audit against base `afc83394` is `READY`, with zero
unauthorized paths and package-audit ID
`e4aa493265069de63c393037a66dc8526d499e903bdb8314f5e80fc7d76a9e87`.

Static: line counts are `repository.rs` 1,633, `planner.rs` 2,429,
`planner_coverage_tests.rs` 1,535, `executor.rs` 2,979, and
`executor_coverage_tests.rs` 785. The planner and executor warnings retain the
existing recovery-package decomposition disposition; no file reaches the
3,000-line blocker.

Ran: the first delegated post-closure invocation used unsupported boundary
`CAMPAIGN` and stopped before planning with `GATE-PLAN-BOUNDARY`. Artifact root
`/home/workdir/testgate-recovery-trust-01-final-next.6vYzMJ` retains a `READY`
package audit and pre-receipt-failure SHA-256
`a7f8c4bce73e57931f23413e4864aba6c78bb54dd5d9039626a16404cb307234`.
No plan, LIGHT, pre-heavy audit, HEAVY node, receipt, or retry existed. RTR-042
binds the supported `INCREMENT` boundary before another changed-head attempt.

Static: both independent reviews pass the exact RTR-042 documentation correction
at `dcb43397`. RTR-042 is durably closed with ledger digest
`1b65f07f830ffb3691c6251a0921fe8a0c6368bed8cd66779a764d36f9dc6f55`.
