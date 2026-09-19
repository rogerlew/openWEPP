# Grid40 correctness final-preparation review

Reviewer: `/root/grid40_correctness`, continuing the independent correctness
review recorded in `correctness-implementation-review.md`.

## Evidence class and current identity

**Static:** This is bounded preparatory inspection of the moving impl4 detached
source at
`/home/roger/openwepp-experiments/b01-wb14-grid40-source-20260919`. It is not a
verdict on a final source identity. No original replay, evaluator, solver, model,
Rust build, or Rust test was run by this reviewer. Supplied impl4 test logs are
implementer-run evidence and remain provisional until matched to immutable C/T
patches.

**Ran:** Read-only source hashing/diffing, retained-trace JSON queries, and
artifact/log inventory only. The retained trace queried has SHA-256
`527208c3e5ad07f1744f1bd92c9e4ac17cf7efd9469968a97d7964c5db482dfd`.

## Affected fix verification in progress

Static inspection finds the earlier design defects corrected in the moving cut:

- evaluator class, ordinary, and complete ceilings are preflighted together
  before any applicable start counter is committed;
- every accounting row now names a counter class and operation ID; complete
  Jacobian evaluator children link to their signed-probe parent without sharing
  its ID;
- evaluator errors retain their typed debug classification and started/error/
  in-flight operations can be reconstructed by ID;
- signed-probe identity, complete, error, denied-child, ordinary interruption,
  and unwind paths retain distinct lifecycle evidence;
- counter sessions reject a grid different from the installed policy and report
  the full baseline/treatment numerical identity;
- event-journal failures use a distinct `Grid40EvidenceFailure`, and invalid
  sidecar generation does not relabel them as budget stops;
- the real `covered_trial_is_valid` function-entry count and real leaf-Ci bracket
  loop are observed separately; the latter is not double-charged as an evaluator;
- the positive full-solver no-update witness test accepts the unchanged current
  base and records zero installed updates.

## Remaining corrections before immutable freeze

1. `grid40_count_ci_iteration` currently uses `saturating_add`. Canonical
   authority says no counter may silently saturate or wrap. Use checked addition
   and surface overflow as invalid evidence.
2. A trace-recorder failure discovered after a normal return or `BudgetStop`
   still reaches an `assert!` in the ignored entry. That loses the shared invalid
   sidecar/cost report. Convert the retained trace failure into
   `Grid40EvidenceFailure` and write the same incomplete/invalid sidecar before
   terminating nonzero.
3. Add the ordinary-ceiling interaction control: class and complete availability
   plus exhausted ordinary must leave class/ordinary/complete starts unchanged.
   The current class-cap and complete-cap controls do not cover this permutation.
4. Add the bounded actual-strict-wrapper evaluator-error control, including the
   mutually exclusive replay/fault guards specified in the prior review. Verify
   typed error evidence and continued ordered search; no alternate result or
   production path is permitted.
5. Strengthen the exact-bound full-solver vector to assert the decisive b40
   coordinate bits equal the base, governed step is tiny/zero, complete residual
   remains unacceptable, no update is installed, and the attempted exponent
   sequence ends exactly at40.
6. Reconstruct and freeze the final complete-test C cut, run preservation green
   plus actual-callsite b21 red, then create the one-choice T cut and run the same
   controls green. Bind every supplied result to those immutable file/patch
   hashes and retain the honest earlier-exploratory chronology.

## Original-case operand preparation

`original-case-row-dimensions.json` is correct as a source-derived map for the
29-row original R0 represented-snow system. Independent retained-trace queries
found seven `accepted_base` frames, each with29 raw values and29 applied
normalizers. Both occupancies retain `Inactive` sun gas, `ExactZeroPar` shade
gas, zero sun area, and nonzero shade/wet/stem areas in all seven frames. That
supports, per occupancy, sun hydraulic identity in mm, shade hydraulic balance
in kg m^-2 tile s^-1, two dimensionless beta identities, stem/root hydraulic
balances, the inactive-sun temperature anchor in K, and shade/wet/stem energy
balances in W m^-2 tile. The shared heat/vapor rows and represented-snow ground
plus six soil temperature identities match unchanged source construction.

The artifact properly requires each arm frame's actual applied normalizers and
branch evidence rather than frozen numbers. It is a reconstruction map, not
closure evidence: potential requests are not finalized fluxes, and the
represented-snow ground/soil rows are temperature identities. At result review,
recheck every new frame against the actual gas/area/wet/stage3 branches before
using the labels.

## Provisional disposition

**HOLD.** No immutable final C/T source has been supplied yet, and the six
remaining corrections above affect launch-critical evidence. Even a correctness
GO after fixes cannot substitute for the separately required QA review, which is
currently unavailable.

Inspection-command failures in this continuation: **0**.

## Same-reviewer moving-cut fix verification — 2026-09-19T07:01:10Z

**Static:** This section supersedes the earlier six-item moving-cut list only
for the fixes named below.  It is still not an immutable C/T or launch verdict.
At inspection time the six affected source hashes were:

- `lib.rs` `b0ed624cf562c4e27a939b8b75b0fafa6c890f354e314f119268d657a2908f5e`;
- `lse_first_trial_diagnosis.rs`
  `ec12f6e2837bf0f2846ba2005187a2dcbd23f79e5717ef57672c92879db124c5`;
- `solver_covered_evaluation.rs`
  `c294d82fdfa6e39065eafe4950a540697fbd7f31cf43cba1311870d8b7cc8a7a`;
- `solver_covered_solve.rs`
  `20b94e2da2670201b699cde2e80b8641782564429e7c09d15e80963dc7983223`;
- `solver_tests.rs`
  `d8914d4018067b6e62d2f94ced4da8f52a127337a39b8eac4fbf0b7909f1eea0`;
- `grid40_experimental_tests.rs`
  `4ee6ec1cf6b7a24bdf1163d0740d3e04c383c178c79f83721ec5cc16c12136db`.

**Ran:** Read-only hashing and source/diff inspection only.  The reviewer did
not run Rust, a solver, an evaluator, or the original case.  Reported focused
and crate-test passes are implementer-run evidence and remain provisional.

### Closed on the moving cut

- Ci nested-work counting uses checked addition.  It remains separate from
  complete-evaluator admission and therefore does not double-charge work.
- Class, ordinary, and complete limits are preflighted atomically.  The new
  exhausted-ordinary control verifies no class or complete start is committed.
- Event-journal and ordinary-trace failures have a distinct evidence-failure
  path and share invalid/incomplete sidecar reporting.  The ordinary recorder
  now writes a serialized record to same-directory `.partial.next` and renames
  it over `.partial`; a failed replacement preserves the preceding complete
  prefix.  Final replacement does not truncate that prefix, and recorder begin
  clears stale failure state.  The negative retained-prefix control exercises
  this boundary.
- Strict-fault and replay hooks are mutually exclusive in both directions,
  with fault installation requiring an active counter session.  The fault is
  one-shot, converts only an otherwise successful actual strict evaluator call
  into an existing typed numerical error, and cannot be armed in the original
  replay.  Active strict-exponent state is restored by RAII on ordinary return
  or unwind, so caught failures cannot contaminate later events.
- The real b40 vector separately records every strict candidate before the
  unchanged domain predicate without classifying invalid candidates as
  evaluator starts.  Its controls assert ordered b0..40 attempts, outward
  decisive temperature bits through b39, only b40 reaching evaluation, b40
  bitwise duplication of the current base, no installed update, unchanged
  failed solution, finite governed steps below the live policy thresholds, and
  at least one still-unacceptable normalized residual.  The injected b21 error
  and b22 completion are checked in event order with typed-error evidence.
- The exact trace hash/wrong-arm controls run before deserialization/evaluation;
  the named domain counter is charged at the actual
  `covered_trial_is_valid` entry.  Unique operation IDs, signed-probe parent/
  complete-child links, typed evaluator errors, denied starts, and retained
  in-flight IDs make the raw lifecycle recountable.  The separate accepted
  exponent and attempted-trial observations do not alter solver arithmetic.
- The no-update positive witness accepts the bitwise unchanged current base,
  records zero installed updates, and exercises at least one witness call.
  Default builds retain b0..20; the treatment selector, counters, replay and
  fault controls are test-only.  The shared strict constant, constitutive
  equations, scaling, stencils, limits and public serialization remain
  unchanged in the inspected diff.

### Remaining blocking evidence

1. The actual-wrapper fault control is still located in the treatment-only b21
   fixture in this moving identity.  It must be split into a real-callsite
   control that is green on the final hard-b20 C cut, while b21 remains the
   intended C-red/T-green discrimination.  Direct helper coverage alone is not
   sufficient.
2. Supply immutable complete-test C and one-choice T patches from the frozen
   predecessor, with final six-file hashes and source-bound logs.  C must retain
   preservation greens and the actual b21 exhaustion red; T must change only
   the confined strict range/exhaustion choice and make the same vector green.
   The earlier exploratory sequence cannot substitute for this reconstruction.
3. Reconcile final formatting, default/test-support compilation, focused/full
   owning-crate tests and matched inherited-Clippy fingerprints to the exact T
   identity.  No original-case replay has run.

**HOLD.** The moving-cut correctness fixes above are accepted, but the shared-C
fault control and immutable final C/T lineage remain launch prerequisites.
Independent QA is separately unavailable; this correctness review does not
replace it.  Inspection-command failures in this continuation remain **0**.

## Immutable terminal C/T review — 2026-09-19

### Evidence class and identities

**Static:** I reviewed the complete immutable C and T patches against the exact
frozen predecessor, the two-line C-to-T patch, the final detached source, the
canonical GRID40 section and its binding row, the private replay entry, the
actual covered-solver call sites, the contract-derived controls, the terminal
command receipts/logs, the offline Clippy comparator, and the source-recovery
record.  No original-start replay, canonical evaluator, solver, Rust build, or
Rust test was run by this reviewer.  Supplied Rust results below are root-run
evidence inspected by this reviewer.

The reviewed immutable identities are:

- frozen source tree
  `8cd4866f66b06d6cf96403753cdb3d41ba75f6ca87652341ebe082a86a1b4bc5`;
- C tree `94fa481fb76fead1a8d217c46dcdd27690ae2385cca06b9fd929f0cc1204a25a`
  and patch
  `95f6b0c3bcfb59b7db07aed10f5ace2341f5622f7f049f00ea80ee3e0e1d3f3e`;
- T tree `fae897fe90f6bbadcaf298607c9d23fd986cd778f9dd56195f7a5510cbd5c4be`
  and patch
  `ae2478a9238f3e3f9753c0607bd4129b86a3ca240f59ad44a457a7163d8200b9`;
- C-to-T patch
  `e0a45653ac26fb5ab3f3adc78975623f645845c20b1a91a384cd0c693c08447d`;
- frozen test binary
  `e26cc780e7065211714a37653e55f9038cee3a5a32a6964de7122cd80cf0f272`
  (55,908,800 bytes), bound to the T tree by the binary receipt.

**Ran:** I performed read-only source hashing, patch hashing, `git apply
--check` for both cumulative patches at the frozen base, exact byte comparison
of the repository copies against the external terminal artifacts, Cargo-JSON
manifest/diagnostic inspection, source/diff searches, and log/receipt queries.
The independently recomputed frozen/T tree hashes are the identities above.
`terminal-source-recovery.json` additionally reconstructs C directly from the
frozen base, T from C plus the C-to-T patch, and T directly from the frozen
base, with all three recovered hashes matching.

### Findings

No blocking implementation or science-contract finding remains on the
immutable T source.

The terminal C reconstruction is a valid contract-first behavioral control.
It retains all completed GRID40 controls and the advertised treatment helper
envelope, while the actual covered strict loop and its failure contribution
remain hard b0..20.  Its full preservation run passed 194 tests with seven
ignored and only the named treatment vector filtered.  The named actual-callsite
b21 vector then failed with exit101 because its attempts ended exactly at b20.
This is the required real-path red; it is not relabeled as a passing command or
as a result-bearing measurement.

The C-to-T patch changes only
`solver_covered_solve.rs`: the actual strict loop changes from
`0..=MAX_BACKTRACKING_HALVINGS` to the already tested confined selector, and the
corresponding exhaustion contribution changes from the shared constant to its
matched confined selector.  No equation, derivative, scale, perturbation,
stencil, pivot, domain predicate, acceptance predicate, no-update witness,
update limit, shared constant, or operation order changes in that delta.

The final T implementation preserves the required boundaries:

- non-test compilation returns b0..20 directly; treatment selection, replay,
  fault injection, counters, event evidence, and the original-start entry are
  test-only, with no public configuration/wire or production runner surface;
- treatment uses exactly ordered binary64 factors b0..40 in the existing solver
  body and changes only the matched backtracking-exhaustion contribution;
  baseline and witness remain b0..20 and installed updates remain capped at50;
- the named domain counter is at `covered_trial_is_valid` function entry, so it
  counts each invocation, including repeated stencil/validator/helper callers,
  without counting individual scalar comparisons or changing other guards;
- base, witness, strict, ordinary, complete, assembly, update, signed-probe,
  domain, and Ci-loop observations map to their real call sites.  Evaluator
  class/ordinary/complete admission is atomic, denial precedes execution, and
  checked accounting cannot silently wrap.  The Ci count is honestly limited
  to bracket-loop labels3..64 and is not represented as complete-evaluator
  work;
- signed probes and complete children have unique linked operation IDs, typed
  error/complete/denied/in-flight evidence, and unwind-safe lifecycle handling.
  The synthetic strict fault is one-shot, mutually exclusive with replay in
  both directions, and changes no alternate numerical result;
- recorder replacement is atomic through `.partial.next`, retains the last
  complete partial prefix on replacement failure, and routes recorder/journal
  failures to an invalid measurement sidecar rather than numerical refusal;
- the former post-refusal evaluator probes are removed, so neither arm spends
  hidden evaluator work after the canonical refusal.

The fixed behavioral vectors discriminate the intended choice.  The shared
actual-wrapper fault vector remains inside b20 and directly proves ordered
typed error b19 followed by completed/installed b20.  The separate treatment
vector proves the actual callsite installs b21, and its injected-error variant
records error b21 before completed/installed b22.  The exact-bound vector
observes attempts b0..40, proves b0..39 move outward, b40 duplicates the current
base bits, only b40 reaches evaluation, no update is installed, the failed
solution remains unchanged, governed step fields are finite and within their
unchanged thresholds, and a complete normalized residual remains unacceptable.
The positive no-update witness accepts the unchanged current base and records
zero installed updates.  Existing noncaptured coverage also proves equal
baseline/treatment outcomes for a path that never needs b21.

No substantial numerical logic is duplicated.  Both policies use the same
covered solver, evaluator, domain predicate, acceptance path, and candidate
constructor.  The added repetition is confined to test-only accounting/event
serialization and does not create a second scientific algorithm.

### Supplied terminal validation inspected

Root-run T validation is source-bound and unchanged across each command:
default and `test-support` owning-crate suites each passed195 tests with seven
ignored; default and test-support checks passed; formatting passed; and the
libtest binary was built without execution before being frozen.  The one ignored
original-start entry was not run.

Strict Clippy remains red from inherited diagnostics, so it is not reported as
a pass.  The corrected baseline inputs resolve to the exact frozen manifest,
and the candidate inputs resolve to the exact T manifest.  The reviewed offline
comparator retains diagnostic level, code, message, exact primary highlighted
source and multiplicity while ignoring only source-root prefixes, positions,
indentation and unhighlighted wrapping context.  Default compares95 to95 and
test-support100 to100, with empty candidate-only and baseline-only multisets.
The earlier `final-frozen-clippy`/`final-candidate-clippy` capture was
candidate-versus-candidate and remains rejected evidence; it is not used here.

Manual detached-source inspection covers the applicable A1 bindings:
INV108's deterministic order and first-error precedence are unchanged except
for the expressly authorized treatment range/count; INV112/138 scaling and
stencils are unchanged; and INV139 witness and accepted-candidate admission are
unchanged.  A3 has no changed constitutive equation, unit, or runtime binding.
Owning-main A0 authority admission remains separate authority evidence and is
not misrepresented as a detached-source workflow run.

### Residual risk and disposition

The study acceptance evidence does not exist yet: no original-start baseline or
treatment arm has run, so baseline trajectory correspondence, terminal T
outcome, actual original-case costs, and decisive original-case operand
reconstruction remain unassessed.  Those are result-review obligations, not
defects in this prepared source.  The branch-aware 29-row reconstruction map is
adequate preparation only; every result frame must still be relabeled from its
actual branch evidence and actual applied normalizers.

**Implementation/preparation correctness: GO, with no remaining source
correctness blocker.  Result-bearing launch: HOLD/BLOCKED.**  The separately
mandatory independent QA review is unavailable and this correctness review
does not substitute for it; therefore neither original-start arm may be
dispatched.  A later result verdict also requires this same reviewer to inspect
the matched pair and independently reconstruct the accepted endpoint or
decisive refusal.

Correction to the earlier running tally: this review continuation had **3**
bounded inspection-command failures when an immutable patch was initially
addressed by a relative path from the wrong working directory (one hash lookup
and two text inspections).  Absolute-path recovery succeeded; no source,
evidence, or verdict was affected.  Terminal cumulative inspection-command
failures for this review continuation: **3**.
