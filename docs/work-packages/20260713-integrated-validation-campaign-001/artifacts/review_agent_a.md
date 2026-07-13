# Independent Review A

Status: `HOLD-VALID-WITH-FINDINGS`

Evidence class: **Ran artifact inspection + Static source review**

Reviewed frozen production source:
`f80a115148e75a08269eb14a8c1b0e7791ca891a`.

Verdict: `HOLD-VALID` with accepted findings that must be corrected or
explicitly carried as blockers before terminal verification. The required
release command really failed, the remaining release and final gates are
truthfully blocked, and no partial integrated-validation PASS is legitimate.

No heavy test, release, coverage, comparator, or workspace gate was rerun for
this review.

## Command And Source Truth

All command IDs 00 through 18 have matching log and `/usr/bin/time -v`
records. The commands, exits, elapsed times, and maximum RSS values in
`gate-results.md` reproduce from those files. Commands 00 through 15 exit zero
with the stated selection counts. Command 16 is the literal no-skip
`bash tools/release/run_release_candidate_gates.sh`, exits 101 after 8:50.55,
and stops in `cargo test --workspace` with exactly three failed H2637 selector
tests. Commands 17 and 18 exit zero but are correctly labeled HOLD-evidence
Markdown/diff checks, not Phase 6 closure substitutes.

Source inspection confirms the mechanism boundary. The header of
`tests/integration/laned_shadow_h2637.rs` states that its environment-mutating
tests are supported only under nextest process-per-test isolation. The three
failed tests clear and mutate the same Lane D selector environment variables.
The release script nevertheless invokes threaded `cargo test --workspace` at
its workspace gate, before required authority, release build/lint, and
stability. The failure is therefore a real release-harness violation; it is not
a production-physics, comparator, fixture, or threshold verdict.

The frozen source is a direct descendant of the committed CQR authorization,
and the working changes inspected by this review are confined to package,
roadmap, catalog, log, and successor-package evidence. No production, test,
contract, schema, release-script, or fixture edit was mixed into the frozen
campaign.

## Consumer, Publication, And Conservation Review

The strongest passed lanes are real and non-tautological:

- p61 and p102 invoke `execute_hillslope_run`, independently read pass Parquet
  and strict HBP output, reconstruct hourly water/sediment sums, bind nonzero
  multi-class sediment, and use an OFE-2 texture perturbation to reject a
  hillslope-global composition alias;
- W7R invokes the production watershed CLI for jobs 1 and 4, compares decoded
  required Parquet schemas/rows/null posture, parses the generated HBP, and
  proves public routed `sed_del` differs from raw `tdet - tdep`;
- MT3/W11B invokes the production watershed CLI, proves equal-total/different-
  timing sensitivity, consumes the upstream same-grid channel egress, and
  reconstructs terminal water plus storage and terminal sediment mass; and
- the runner and watershed package suites provide broad typed fail-closed
  coverage, while commands 03 through 05 directly bind the three matrix-listed
  missing/mixed/malformed routing-authority cases.

Those tests substantiate real-consumer and independent-reconstruction test
bindings. They do not, however, make every adjacent artifact a completed Ran
record. `operand-lineage.md`, `consumer-path-evidence.md`,
`conservation-reconstruction.md`, `publication-identity.md`,
`fail-closed-results.md`, and `comparator-delta-review.md` remain explicitly
`ACTIVE`/Static/pending. In particular, the positive H2637 test reads the
active manifest plus HBP/Parquet bytes and checks producer-published residuals;
it does not independently reconstruct the package's listed precipitation,
groundwater-reservoir, baseflow-once, deep-seepage, or WAT operands. The frost
profile summary likewise does not itself archive a selected snow/frost numeric
reconstruction. No per-lane output hashes or numeric result table closes those
claims here.

This incompleteness does not invalidate the HOLD: the package explicitly
requires a full post-fix restart and forbids carrying pre-fix results into a
future PASS. It does prevent the current assessment from treating every static
map as completed conservation closure or reusable terminal evidence.

## Release Attribution And HOLD Legitimacy

The HOLD boundary is legitimate. Changing
`tools/release/run_release_candidate_gates.sh` is outside this validation-only
package's declared write set, while serializing, skipping, retrying, or
overriding environment state would weaken the required unmodified release
lane. The package records the nonzero command, affected scenario, named defect
`INTVAL-REL-001`, bounded release-harness authority, successor package, and
full-campaign restart condition. The required authority, stability, release
build/lint, and Phase 6 gates are `BLOCKED`, not passed, deferred into a false
completion, or silently omitted.

The attribution record does overstate one detail. Command 03 proves
`h2637_active_fails_closed_without_routing_coefficients` passes alone under
nextest. Commands 04 and 05 are the different mixed/malformed default-authority
tests; they are not isolated executions of
`h2637_active_and_disable_are_mutually_exclusive` and
`h2637_active_and_shadow_are_mutually_exclusive`. The latter two failures are
consistent with the explicit shared-environment mechanism, but the artifact
set does not contain three focused reproductions of the three release failures.
That correction affects diagnostic precision, not the fact that the mandatory
release gate failed and forces HOLD.

## Successor Envelope Audit

The queued DC package correctly owns the release script, README, a narrow
source guard, and direct conversion from workspace libtest to
`cargo nextest run --workspace --profile full`. It protects production,
fixtures, selectors, assertions, thresholds, required authority, and skip
posture, and it requires the complete integrated campaign to restart at the
correction commit.

Its current acceptance command is not yet executable as written through all
lanes. The release script defaults `COHORT_SEEDS_CSV` and `WATCHLIST_CSV` to
empty strings and deterministically exits 2 at the stability boundary unless
both are supplied. The DC package simultaneously requires the unmodified
no-argument default release command to exit zero through stability and limits
the allowed correction to replacing the workspace test command while
protecting stability behavior. A nextest-only edit would therefore expose a
second guaranteed failure. Before DC execution, the successor must either:

1. name the canonical cohort/watchlist and expected-suite arguments in the
   required no-skip release invocation, and update INTVAL's restart matrix to
   use that exact command; or
2. explicitly expand and review the DC envelope to install authoritative
   default stability inputs without weakening the stability lane.

The successor's first actionable item remains correctly phrased as `close
defect INTVAL-REL-001`, but the invocation/envelope contradiction must be fixed
before it can claim end-to-end closure.

Roadmap and package-catalog entries correctly mark INTVAL executed HOLD, name
`INTVAL-REL-001`, link the bounded successor, and require a full restart rather
than resumption from partial evidence.

## Findings And Disposition

| ID | Finding | Disposition |
| --- | --- | --- |
| INTV-A-01 | `hold-legitimacy-audit.md` and `disposition.md` imply all three release failures passed focused nextest executions, but only the missing-coefficients failure did; commands 04/05 cover different mixed/malformed cases. | `accepted`: correct the wording to distinguish one direct isolated reproduction from two source-supported shared-environment attributions, or add exact focused evidence in the successor. This does not change HOLD. |
| INTV-A-02 | Several consumer/conservation artifacts remain Static/ACTIVE and do not contain output hashes or complete independent H2637 groundwater/snow operand reconstruction, while the assessment says the passed lanes bind the adjacent reconstruction maps. | `accepted`: retain these as partial pre-fix test-binding evidence, qualify the assessment, and do not reuse them for post-fix PASS. The mandatory full restart is the closure route. |
| INTV-A-03 | The DC successor requires a no-argument default release command to pass stability, but the protected current script requires explicit cohort and watchlist arguments and otherwise exits 2. | `accepted`: amend the successor command/envelope before execution as described above. Until fixed, the successor is not an adequate end-to-end hold-lift plan. |
| INTV-A-04 | Phase 6 formatting, Clippy, full nextest, deny, Markdown, and diff gates were not run after release failure. | `rejected` as a false-PASS concern: they are explicitly `BLOCKED`, the package is HOLD, and the later HOLD-only doc/diff checks are not substituted for them. |
| INTV-A-05 | The release failure might be waived as infrastructure because domain lanes passed. | `rejected`: release is a mandatory current-scope gate; infrastructure ownership legitimizes defect transition and HOLD only, never PASS. |

No finding is `deferred`. INTV-A-01 and INTV-A-02 are evidence corrections
for truthful HOLD closure. INTV-A-03 is an accepted successor-package finding
that must be fixed before the hold-lift executes. Any attempt to proceed to
verification or integrated-validation PASS without those dispositions is a
`follow-up` blocker and remains `HOLD-INTEGRATED-VALIDATION`.

Final Review A verdict: `HOLD-VALID-WITH-FINDINGS`. There is no false PASS,
and the current frozen campaign cannot close as
`PASS-INTEGRATED-VALIDATION`.
