# Round-2 Review C — Correctness, Governance, And Lifecycle

Evidence class: `Static` plus `Ran` source-identity and repository-state checks

Disposition: `HOLD`

I performed a fresh adversarial review of ADR-0039, the testing/gate standard,
the complete package and prior review record, and the current authorities they
claim to preserve. I did not consult or coordinate with Reviewer D. The current
ADR and standard hashes match those recorded by terminal verifier B, so the
findings below apply to the previously verified closure candidate rather than
to later unverified authority edits.

## Findings

### C-001 — Blocking — Ordinary test work still triggers the full global CRAP path

The main friction reduction is defeated by the coverage rule for test changes.
The standard says an ordinary bounded production increment uses affected
coverage/CRAP and does not require global CRAP
(`testing-and-gate-strategy.md:215,228-231`), but it then requires immediate
full-workspace coverage and adjudicated CRAP for “materially changing tests”
(`testing-and-gate-strategy.md:739-750`). Most scientific increments add or
materially revise regression, boundary, or contract tests; the Rust scientific
coding standard requires exact changed-behavior tests and applicable A–H
families. Under the current words, those increments still pay the dominant
full-workspace instrumented run that ADR-0039 is intended to remove. The phrase
also has no mechanical predicate, so two planners can classify the same test
edit differently.

The affected-function rule compounds the problem: it calculates CRAP from
“selected” tests (`testing-and-gate-strategy.md:712-737`) without requiring the
complete known test set that contributes coverage to each affected function.
A narrow suite can therefore report an artificial CRAP regression merely
because a still-valid covering test was not selected.

Required remedy:

- replace “materially changing tests” with deterministic coverage-loss reason
  codes and closed predicates;
- allow purely additive tests and bounded test edits to use affected
  measurement when test inventory plus prior/new coverage contribution proves
  that no coverage outside the affected closure was removed;
- make delete, disable, ignore, filter, or unbounded loss of prior coverage
  global/critical, with unknown mapping failing to global;
- require affected CRAP to use the complete mechanically known covering-test
  closure for each affected function, not merely the initially selected
  behavior suites; and
- add acceptance scenarios for additive tests, a modified test whose former
  coverage is preserved elsewhere, a coverage-reducing edit, and an unknown
  contribution map.

### C-002 — Blocking — Non-blocking authority outcomes cannot be represented or dispositioned

The preserved correctness-authority model distinguishes execution from
adjudication: A1/A3 failures block; A2/A6 deviations are investigation signals;
and A4/A5 are non-blocking unless explicitly promoted
(`correctness-authority-model.md:31-50`). The new standard has only receipt
results `PASS`, `FAIL`, `BLOCKED`, and `INVALID`
(`testing-and-gate-strategy.md:587-606`), reduces any required-gate failure to
aggregate `FAIL` (`testing-and-gate-strategy.md:478-482`), and prohibits campaign
closure with a `FAIL` ledger obligation
(`testing-and-gate-strategy.md:671-690`). Although it says diagnostic failures
may remain investigation debt (`testing-and-gate-strategy.md:876-880`), neither
the gate schema nor the ledger has a state or transition for that debt.

Consequently, an A6 comparator divergence or an unpromoted A4/A5 miss must
either be mislabeled `PASS`, block contrary to current authority, or disappear
outside the content-addressed ledger. The same ambiguity affects a hard-fail
versus investigation suite in the existing external-authority registry.

Required remedy: define separate machine-readable execution integrity and
scientific/adjudication outcome axes. A required investigation suite should
block when it fails to execute or produce its complete inventory, while a valid
divergent result creates an owned `INVESTIGATION_PENDING` record that follows
the correctness-authority verdict/disposition vocabulary without becoming a
false pass or an automatic blocking failure. Define promotion of A4/A5 or an
investigation-class suite to blocking as a plan-bound policy decision, add
ledger transitions for disposition, and state exactly which unresolved
investigations block increment, campaign, and release transitions. Add fixtures
for A2, A4, A5, and A6 divergent outcomes and both external-suite failure
classes.

### C-003 — High — A0 authority is not an explicit kernel-increment gate

The current correctness authority makes A0 mandatory for every
kernel-affecting package and requires `HOLD` when canonical authority is missing
or ambiguous (`correctness-authority-model.md:31,41`). The increment minimum
explicitly derives A1 and A3, but not A0
(`testing-and-gate-strategy.md:200-222`). Reading contract bindings as planner
input does not prove that every affected kernel/process surface has a current,
unambiguous canonical contract; critical full regression cannot cure missing
authority. The handoff acceptance list tests an applicable A3 binding but has no
missing, ambiguous, or stale A0 case (`implementation-handoff.md:168-198`).

Required remedy: add a non-deferrable A0 authority-admission gate for every
kernel/process increment. The plan must map every affected process surface and
public process behavior to canonical contract IDs and versions, prove the
contract/index/obligation bindings are current, and produce `BLOCKED`/`HOLD`
when the map is missing, ambiguous, provisional, or stale. A critical risk
escalation must not convert missing A0 into a testable pass. Add positive,
missing, ambiguous, and stale-contract acceptance scenarios.

### C-004 — High — Assurance report selection is an under-selection evasion path

The assurance rules operate on “each selected report” and every selected
report's watches (`testing-and-gate-strategy.md:779-804`). Unknown impact blocks
all *release-selected* reports (`testing-and-gate-strategy.md:818-824`), and
release qualification checks selected reports
(`testing-and-gate-strategy.md:277-289`). Campaign declarations likewise name
the reports in scope (`testing-and-gate-strategy.md:637-649`). Nothing proves
that the selected set equals every approved/published report, catalog entry,
snapshot, export, or generated public page actually carried by the candidate.
An operator could omit an affected report from selection while its public bytes
remain in the repository or distribution.

That would violate the dossier lifecycle, which requires incomplete transfer
to keep the report out of the release snapshot and requires only published
reports in public discovery. It also undermines ADR-0039's promise not to
weaken publication and release-transfer requirements.

Required remedy: distinguish the mechanically discovered impacted-report set
from the release inclusion set. Derive the latter from the exact public catalog,
snapshot, export, vendoring, and distribution inventories; require set equality
between shipped public report identities and current transfer records; and
fail closed on any public/generated assurance object with no selected current
report. Campaign impact planning should record matches for every registered
report dependency, not only an operator-preselected subset, while scientific
work may remain deferred to its governed boundary. Add an attempted omission
fixture and a fixture proving explicit historical exclusion removes all public
release surfaces without rewriting the historical report.

### C-005 — High — The certified-head and evidence-persistence protocol is circular

Campaign closure certifies an exact clean commit
(`testing-and-gate-strategy.md:85-87,255-275`), while every receipt records the
head/tree identity (`testing-and-gate-strategy.md:589-603`) and the append-only
ledger records receipts and creates a new version when the campaign head
advances (`testing-and-gate-strategy.md:651-656`). The architecture and handoff
do not define where plans, receipts, ledgers, and the final campaign certificate
live or how they are durably attached to a commit without changing that commit.
If these records are committed under the repository, recording the passing
receipt changes `HEAD`; if they are untracked, the authority does not define
their durable store, atomic publication, access control, recovery, or binding.

Required remedy: define a two-phase certification protocol and evidence-store
boundary. Freeze candidate commit `C`; execute and verify against `C`; atomically
publish immutable receipts, ledger transition, and certificate in a named
content-addressed store or non-tree Git reference; then bind the certificate to
`C` without modifying `C`. Define crash recovery, duplicate publication,
partial artifact upload, authorization/signature or trusted-producer identity,
garbage-collection retention, and later in-repository archival semantics. Add
acceptance scenarios proving that evidence publication cannot move or silently
redefine the certified source head.

### C-006 — High — Concurrent admitted increments have no deterministic rebase or invalidation rule

The ledger may admit increments before edits and advances the campaign head
after a closed terminal plan (`testing-and-gate-strategy.md:651-669`), but it
does not say whether two increments may be admitted against the same campaign
head. If they can, the first merge changes the base for the second; its terminal
diff, reverse-dependency closure, receipts, and deferred-obligation assumptions
may no longer describe the new integration head. The only rebase language
addresses the campaign base generally and merely says differing receipts become
stale (`testing-and-gate-strategy.md:655-656`). If increments must serialize,
that major workflow and friction constraint is also unstated.

Required remedy: choose and specify one deterministic model. Either permit only
one active admitted increment per campaign head, or assign each admission an
expected-parent identity and require terminal replan/reconciliation against the
current campaign head before head advancement. Define conflict, supersession,
abandonment, changed risk, newly selected gate, and formerly deferred obligation
transitions. Add two-increment fixtures for disjoint edits, overlapping edits,
and a first increment that changes the second increment's impact selection.

### C-007 — Medium — Enforcement cutover has no measurable safety or friction acceptance contract

The authority's performance numbers are explicitly non-binding diagnostic
objectives (`testing-and-gate-strategy.md:882-897`). The handoff nevertheless
says blocking selection starts only after “acceptance thresholds are met”
without defining those thresholds (`implementation-handoff.md:145-159`). It
also calls for retained-campaign replay but does not define the comparator
population, maximum missed-gate rate, false-narrowing disposition criterion,
maximum unmapped-path rate, receipt-verification reliability, or required
before/after latency improvement. Those are policy decisions, not merely coding
details, and leaving them open permits a cutover that adds planner overhead
without reducing the 30–40 minute maintenance path that motivated the ADR.

Required remedy: define a cutover scorecard before implementation: retained
campaign corpus and risk strata, zero tolerated missed non-deferrable/critical
gates, treatment of broad-suite discoveries, maximum unresolved mapping and
inventory mismatch rates, receipt/replay determinism, rollback trigger, and
median/p95 increment wall-time and setup-time improvement relative to the
current runner. The implementation package may measure the values, but should
not invent the acceptance policy after seeing results.

### C-008 — Medium — The package currently claims completion while round-2 gates are open

The package status correctly says `IN_PROGRESS-REVIEW-ROUND-2` and Phase 6
requires disposition of new findings and repeated terminal verification after
authority edits (`package.md:1-7,116-123`). However,
`artifacts/final-disposition.md:5-16` still declares `EXECUTED-COMPLETE`, says
all findings are corrected, and cites only the first review round. The gate
results likewise contain no round-2 evidence. This violates the package's own
closure truthfulness and can mislead a catalog reader while the new review is
active.

Required remedy: mark the old final disposition as superseded or reopen it
during review, add a round-2 disposition artifact covering every C/D finding,
rerun two terminal verifications if authority bytes change as Phase 6 requires,
and only then issue one final disposition and update the catalog status.

## Review Result

The strategy has a strong overall separation between affected increment work,
campaign integration, and release transfer, and the first-round remediation
substantially improved determinism. It is not yet safe to close. C-001 preserves
the central friction failure for ordinary test-bearing science work; C-002 and
C-003 leave the A0–A6 authority contract mechanically incomplete; C-004 permits
public assurance under-selection; and C-005/C-006 leave certification and
multi-increment lifecycle behavior unresolved.

`HOLD`

## Remediation verification

Evidence class: `Static` plus `Ran` scoped documentation, identity, spelling,
and diff checks

I re-read the remediated ADR, standard, implementation handoff, reopened
package artifacts, and round-2 disposition. The remediated authority identities
reviewed here are:

- testing/gate standard SHA-256
  `c09e60bbf37c50327ffb45ca2deedc0f232e77dabc5646b1f9eb71bf26dbe7e9`;
- ADR-0039 SHA-256
  `71654ba25088eb2a8768608a77b584858abfc86afe21d1d60f14329d9e283946`;
  and
- implementation handoff SHA-256
  `f7237ce7456909af587a03214f5b79edc7cd0de6bec0670ba239465d5dd2eb52`.

### Remedy status

| Finding | Verification | Result |
| --- | --- | --- |
| C-001 | Closed coverage-loss reason codes replace the broad test-edit trigger; additive and bounded edits use affected measurement only with contribution proof; covering-test closure, conservative expansion, global fallback, and four handoff scenarios are explicit. | PASS |
| C-002 | Execution and scientific outcome are now separate, investigations are owned and plan-bound promotion is prospective. Residual outcome-routing ambiguity remains below. | PARTIAL |
| C-003 | A0 admission is explicit, non-deferrable, uniquely contract/index/obligation-bound, and fail-closed for missing, ambiguous, provisional, or stale authority; positive and negative fixtures are required. | PASS |
| C-004 | Campaign discovery covers the complete versioned registry; release inclusion is mechanically derived and must equal current transfer identities; omission and complete historical-exclusion fixtures are required. | PASS |
| C-005 | The source subject is separated from evidence and the two-phase protocol covers staging, authentication, crash/retry, partial upload, retention, and fresh-clone verification. The authoritative ref topology still does not implement its stated cross-head ledger compare-and-swap, as described below. | PARTIAL |
| C-006 | Admissions bind expected parent/head and replan against the current source head with conflict, abandonment, supersession, and stale-receipt handling. The persistence-layer compare-and-swap remains ambiguous because each subject uses a different ref. | PARTIAL |
| C-007 | The authority now fixes the observation population, zero-miss criteria, deterministic replay, planner p95, matched median/p95 improvement, discrepancy policy, rollback triggers, and provider-side migration order. | PASS |
| C-008 | Package, summary, catalog, old final disposition, and old gate record now truthfully show reopened/superseded state pending round-2 closure. | PASS |

### RV-C-001 — High — `INCONCLUSIVE` and `NOT_EVALUATED` authority outcomes have no required reduction

Section 5.1 introduces four scientific outcomes, but only `DIVERGES` is routed:
A2/A6 divergence and unpromoted A4/A5 divergence create an investigation
(`testing-and-gate-strategy.md:189-208`). It does not say what
`INCONCLUSIVE` or `NOT_EVALUATED` does for any class. Nor does it state that
A1/A3 require the exact `CONFORMS` outcome rather than merely avoiding a named
“nonconformance.” The plan records an `outcome_policy`, but the authority does
not give closed class defaults or require that policy to reject unrouted
outcomes (`testing-and-gate-strategy.md:495-511`).

This leaves the original C-002 evasion path partly open: a complete process can
return execution `PASS` plus `NOT_EVALUATED`, create no investigation record,
and avoid an aggregate failure. For A1/A3 that can admit missing correctness
evidence; for A2/A4/A5/A6 it can silently discard the very uncertainty the
investigation axis was added to preserve.

Required remedy: define the exhaustive authority-class/outcome reduction.
A1/A3 must block unless execution is accepted and scientific outcome is exactly
`CONFORMS`. A2/A4/A5/A6 `DIVERGES` or `INCONCLUSIVE` must create an owned
investigation unless prospectively blocking; `NOT_EVALUATED` must be an
execution/inventory failure unless an explicit predeclared non-applicability
rule proves the suite outside scope. Require the closed mapping in every
`outcome_policy` and add all-outcome fixtures for A1 through A6.

### RV-C-002 — High — Per-subject evidence refs cannot enforce the campaign ledger compare-and-swap

The ledger correctly requires exact-predecessor compare-and-swap and rejects
last-writer-wins (`testing-and-gate-strategy.md:797-814`). Certification,
however, gives every source subject a different ref:
`refs/openwepp/evidence/campaigns/<campaign-id>/<subject-commit>` and says to
compare-and-swap “the evidence ref” (`testing-and-gate-strategy.md:892-910`).
Two concurrent heads derived from the same ledger predecessor therefore publish
to two distinct, previously absent refs. Both create operations can succeed;
neither ref update observes or rejects the other. The stored ledger digests may
show a fork after the fact, but the selected Git transaction does not provide
the claimed lost-update prevention or one authoritative current campaign head.

This is the unresolved interaction between C-005/C-006 and D2-003/D2-004.

Required remedy: define one protected mutable campaign-ledger head ref, for
example `refs/openwepp/evidence/campaigns/<campaign-id>/head`, and atomically
compare-and-swap that ref from the exact predecessor evidence commit to the new
evidence commit. Per-subject refs may be immutable aliases created only as part
of, or after, a successful authoritative head transition; they cannot be the
concurrency control. Specify behavior when the alias creation succeeds but head
CAS fails, and add a fixture where two different subject commits share one
predecessor and exactly one advancement succeeds.

### RV-C/D-003 — Medium — Campaign-to-release reuse still bypasses the new closed reuse-class rule in lifecycle prose

The D remediation correctly defaults every gate to `NON_REUSABLE` and permits
cross-boundary content reuse only for accepted `HERMETIC_CONTENT`
(`testing-and-gate-strategy.md:729-762`). But the global CRAP lifecycle still
says a campaign receipt satisfies release whenever bound inputs are unchanged
and `rerun_on_release` is absent (`testing-and-gate-strategy.md:990-1001`).
ADR Decision 4 makes the same unconditional statement, while Decision 8 later
adds the trust/reuse-class condition
(`0039-campaign-scoped-risk-based-testing-and-assurance-gates.md:69-73,87-91`).

Required remedy: make Decision 4 and Section 12.3 explicitly conjunctive with
Section 10.4: unchanged roots and no `rerun_on_release` are necessary but not
sufficient; the receipt must also carry a target-accepted trust class and
`HERMETIC_CONTENT` (or another future explicitly cross-boundary reuse class).
`NON_REUSABLE` and `SAME_EXECUTION` must rerun at release except when campaign
and release are literally the same authenticated execution and the latter class
permits it.

### Verification result

The C-001, C-003, C-004, C-007, and C-008 remedies are substantively complete.
The candidate remains `HOLD` because C-002's authority-outcome algebra is not
exhaustive and the selected evidence-ref topology cannot enforce the ledger
compare-and-swap on which concurrent campaign safety depends. The reuse wording
should also be reconciled before implementation to prevent a default
`NON_REUSABLE` receipt from being treated as release-current.

`HOLD`

## Final residual verification

Evidence class: `Static` plus `Ran` scoped documentation, spelling, identity,
and diff checks

I reverified the second remediated tree after the residual disposition. The
exact authority identities assessed are:

- testing/gate standard SHA-256
  `a94ee94c691dfdc25fe525b2fe8ab4def45e5f684a7751837bb94c94b9f21532`;
- ADR-0039 SHA-256
  `b31e60ba3860fbbad8b34b723e02efc5d48bf96072924d4ff8aea63da3d92aa6`;
- implementation handoff SHA-256
  `f7237ce7456909af587a03214f5b79edc7cd0de6bec0670ba239465d5dd2eb52`;
  and
- round-2 disposition SHA-256
  `f17b8ca5b2e0d4f8ea05e595c28ba14d4cc3f179e881ab7c5e3281ff85542a06`.

The three residuals are closed:

1. Section 5.1 now gives a complete A0–A6 reduction. A1/A3 require exact
   `CONFORMS`; unpromoted A2/A4/A5/A6 divergence or inconclusive evidence opens
   an owned investigation while complete execution remains separately
   represented; `NOT_EVALUATED` cannot satisfy a selected suite; and promoted
   A4/A5 predicates are prospectively plan-bound.
2. Section 11.1 now uses one protected mutable campaign `head` ref as the
   concurrency authority and creates the immutable subject alias in the same
   atomic compare-and-swap transaction. Different subjects sharing one
   predecessor cannot both advance, and a loser replans from the winner.
3. ADR Decision 4 and Section 12.3 now make campaign-to-release reuse
   conditional on release-accepted `PROTECTED_CI` trust and
   `HERMETIC_CONTENT`, in addition to unchanged roots and absence of a
   `rerun_on_release` requirement. `NON_REUSABLE` and `SAME_EXECUTION` rerun.

I also checked the interacting remediation: obligation states now include
`SUPERSEDED` with closed retry, blocker-resolution, invalidation, failure, and
blocking transitions; assurance impact entries have closed states, exact
refresh evidence, and target-bound folding; gate prerequisites consistently
use `node_id`, whose digest binds the complete canonical node; and the Git
change-set procedure consistently treats rename as delete plus add while
separating index, worktree, and untracked state.

No actionable correctness, authority, campaign-lifecycle, assurance-selection,
CRAP-cadence, evidence-reuse, or friction-cutover issue remains from Review C.
The dual-axis A2/A4/A5/A6 table intentionally permits a complete execution to
satisfy the execution obligation while independently opening an investigation;
that is consistent with the preserved correctness-authority model and is not a
false scientific pass.

Final residual disposition: `PASS`
