# RTR-052 Through RTR-054 Gate Evidence

Static: `close-tooling-defect` is a repository-owned append-only operator
command. It accepts only the latest `OPEN` state of one exact defect, requires a
lowercase 40-character correction commit and nonempty review evidence, confines
an optional invalidated root to the ledger's exact `recovery/` namespace, and
uses the canonical durable ledger append path.

Static: resume discovery folds invalidations by defect ID. Only the latest
`CLOSED` record carrying the exact root, correction commit, and closure
evidence suppresses that root. A later reopen revokes the invalidation.
The latest prior state must be `OPEN` for the same defect and cause, and the
correction must resolve as an exact ancestor of repository `HEAD`. Unrelated
explicit roots still require authenticated provenance.

Ran: all 13 `resume::tests` cases pass, including exact-root invalidation,
unrelated-root rejection, malformed closure rejection, reopen revocation, and
raw consumer rejection for blank evidence, missing/mismatched cause, relative
or dot-dot paths, unassociated roots, absent/mismatched OPEN lifecycle, and a
nonexistent correction commit.

Ran: the two mechanically moved `resume::coverage_tests` reuse-policy
characterizations pass at their new paths.

Ran: the focused tooling-defect closure command regression passes. It proves
outside-root rejection, canonical closure, audit admission after closure, exact
root persistence, and duplicate-closure rejection.

Ran: planner all-target Clippy with warnings denied passes. Rust formatting
passes.

Static: impact-map generation 12 binds the exact amended testing-strategy
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
lines, below the 2,000-line warning threshold; the new test-only file is 1,253
lines. This closes the reviewer-requested decomposition without a follow-on
split obligation.

Ran: the no-follow ledger guard exposed a pre-HEAVY coverage fixture that built
its durable ledger path through `../..`. RTR-054 records the tooling-only
failure. The fixture now derives a canonical repository root, and its exact
55.70-second regression passes. No HEAVY or coverage traversal was launched.

Static: the final accepted reviewer findings are implemented in the resume
consumer itself: it mirrors the producer's OPEN-lifecycle/cause association
and calls the shared exact-ancestor correction validator.

Ran: dual independent implementation review passed at exact clean head
`94462c30cc57735828d774a963245b82cb964f61`. Package-chain reconstruction was
`READY`, with chain ID
`5884de7cdb317be96ba53e51003143e72fdf60754746424122d7ccb09ee2fdab`
and zero unauthorized paths.

Ran: the release planner built from that reviewed head has SHA-256
`db0a1d8b48e93783c96632391a5f81210ca00f90eb7e078d18951f89bff5a3aa`.
Its canonical runner-local closure appended record
`2852c7ed8076cc53ff8dfae7d68abf53d0820e4a794ce2fbc1860c659477d978`,
closing `AUTO-5f75d58df001e6fe` and invalidating only
`/testgate-history/recovery/29984179443-1`. All eight runner-ledger records
re-hash correctly.

Ran: durable campaign closures for RTR-052, RTR-053, and RTR-054 have digests
`3531837d52d0c1a66b602cff15204cb553f4ca49a872b5e3ea20d5368db530f1`,
`39834102c6a33a9ffa27fae8ba5e89b8fe8760054241289df01e43e156857537`,
and `50a59cc7f86656cd04dc454c7281fecbb17c5ab72c411902528a089bdf60d362`.
The 173-record campaign ledger re-hashes correctly and has zero effective open
defects.

Pending: one changed-head automatic qualification.
