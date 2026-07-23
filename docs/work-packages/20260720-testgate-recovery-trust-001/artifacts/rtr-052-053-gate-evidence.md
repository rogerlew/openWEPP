# RTR-052 And RTR-053 Gate Evidence

Static: `close-tooling-defect` is a repository-owned append-only operator
command. It accepts only the latest `OPEN` state of one exact defect, requires a
lowercase 40-character correction commit and nonempty review evidence, confines
an optional invalidated root to the ledger's exact `recovery/` namespace, and
uses the canonical durable ledger append path.

Static: resume discovery folds invalidations by defect ID. Only the latest
`CLOSED` record carrying the exact root, correction commit, and closure
evidence suppresses that root. A later reopen revokes the invalidation.
Unrelated explicit roots still require authenticated provenance.

Ran: all 12 `resume::tests` cases pass, including exact-root invalidation,
unrelated-root rejection, malformed closure rejection, and reopen revocation.

Ran: the focused tooling-defect closure command regression passes. It proves
outside-root rejection, canonical closure, audit admission after closure, exact
root persistence, and duplicate-closure rejection.

Ran: planner all-target Clippy with warnings denied passes. Rust formatting
passes.

Static: impact-map generation 11 binds the exact amended testing-strategy
bytes. The policy update is part of the same prospective package write set.

Ran: the initial broad planner library sweep produced 149 PASS, 20 FAIL, and
two ignored cases. Nineteen failures reported the expected
`GATE-POLICY-DIGEST-DRIFT` after the strategy edit; the remaining
clean-checkout case correctly rejected the uncommitted implementation. After
generation 10 rebound the policy digest, the representative deterministic
critical-policy case passed, and both owning integration targets passed 10/10
and 11/11. Passing cases were not repeated unchanged.

Ran: after committing the exact corrected tree, the full planner library sweep
passed 169/169 with two declared ignored child-process cases in 541.30 seconds.
The planner binary suite passed 9/9. Exact package-chain reconstruction from
`9864a94c...` returned `READY`, chain ID
`8d25e476...f6e0943`, and zero unauthorized paths.

## Initial Review Findings

Static: both initial reviewers returned `HOLD`. Their accepted findings were:
malformed defect states could bypass the open-defect fold; correction commit
and review evidence were syntactic only; an invalidation was not associated
with the exact failed record and cause; dot-dot roots were accepted; and the
ledger append followed final or ancestor symlinks.

Static: the correction now shares exact `OPEN`/`CLOSED` validation between
audit and resume, resolves the correction commit in the repository and requires
it to be an ancestor of `HEAD`, trims review evidence, requires an earlier
failed HEAVY record with the same root and cause, accepts only an absolute safe
child of the durable recovery namespace, and rejects symlinked ledger paths
without outside mutation.

Static: the mechanically extracted `pre_heavy_tests.rs` contains the unchanged
inline test module plus the new closure regressions. `pre_heavy.rs` is 1,863
lines, below the 2,000-line warning threshold; the new test-only file is 1,250
lines. This closes the reviewer-requested decomposition without a follow-on
split obligation.

Pending: dual independent implementation review, exact correction commit,
runner-ledger closure, durable RTR-052/RTR-053 closure, and one changed-head
automatic qualification.
