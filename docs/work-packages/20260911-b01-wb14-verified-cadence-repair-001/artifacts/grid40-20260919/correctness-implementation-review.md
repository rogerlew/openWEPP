# Grid40 correctness implementation review

Reviewer: `/root/grid40_correctness`, independent of the implementation and
science-advice authors.

## Evidence and identity

**Static:** Reviewed the canonical
`SC-LANDSURFACEENERGY-001/solve-boundary.md#b01-grid40-original-start-experiment`,
the package authority, role-review guidance, frozen predecessor
`/home/roger/openwepp-experiments/b01-wb14-lse-first-trial-source-20260918`,
and these immutable base-relative patches:

- control C: `impl3-control-c-base-relative-20260919T055747Z.patch`, SHA-256
  `703c4a5d38cd0daaf8f6b53997201265421d71e93c054ed841db2c7cb67156ad`;
- treatment T: `impl3-treatment-t-final-base-relative-20260919T060017Z.patch`,
  SHA-256
  `3961666c9e4cde4d9343ff3498c72d75e80248126a8867e2e983c009401770f6`.

I inspected supplied execution logs as execution evidence the implementer ran;
I did not run Rust, an evaluator, a solver, or the original case. The supplied
T source-file manifest matches all six files reconstructed from the exact T
patch. Supplied exact-T evidence records the b21 treatment vector PASS, focused
8 PASS/1 ignored, and the owning crate 176 PASS/7 ignored. Supplied strict
Clippy evidence for exact T is FAIL with new diagnostics; a later post-lint
patch has a different SHA and is outside this frozen-cut verdict.

**Ran:** Independently recomputed both patch SHA-256 values, applied both
patches to temporary copies of the frozen predecessor, compared C to T, and
matched the six reconstructed T file hashes to
`impl3-t-prelint-source-files-20260919T0602Z.sha256`. These were source and
artifact inspection commands only.

## Findings

### High — overlapping ceilings can count an operation that is denied before evaluator entry

In treatment `solver_covered_solve.rs:225-241`, `grid40_evaluate` commits the
ordinary counter before checking/committing the complete counter. Base,
witness, and strict call sites also commit their class counter before entering
`grid40_evaluate` (`:564-569`, `:1244-1251`, `:1288-1297`, and `:1340-1345`).
If a later applicable ceiling is exhausted, the earlier counters and start
events remain incremented although the canonical evaluator closure never
executes. That contradicts the canonical rule that a denied next operation is
not executed or counted as an evaluator start and prevents exact reconciliation
at a common-ceiling boundary.

Admit all counters applicable to one evaluator entry atomically: preflight the
class, ordinary, and complete ceilings without mutation; on success commit all
applicable counters/start events and enter the evaluator; on failure preserve
all prior consumed values and record exactly one denied operation without an
evaluator start. Retain the distinct signed-probe-start semantics: a signed
helper can legitimately start before a later complete-evaluator denial, but
the denied complete evaluator itself must not be counted. Add interaction
tests for complete exhausted after ordinary/class availability, ordinary
exhausted with complete available, and class available with a common ceiling
exhausted. The current single-counter synthetic denials do not exercise this
defect.

### High — event-write failures are converted into an apparently valid BudgetStop

`solver_covered_solve.rs:195-209` raises `Grid40BudgetStop` for journal
serialization/open/write failure. The ignored entry catches every
`Grid40BudgetStop` at `grid40_experimental_tests.rs:190-213` and emits a normal
`terminal.kind = "BudgetStop"` sidecar. For an admitted-event write failure,
the counter may already have changed while `counts.budget_stop` remains absent;
the missing event is therefore both unreconciled and mislabeled. Canonical
authority requires failed serialization or missing evidence to remain
incomplete/invalid, never an ordinary private budget disposition.

Use a distinct evidence-integrity stop/status and preserve it as an invalid
terminal classification with the partial counts and the failing evidence
channel. Test event-journal open/write failure and trace serialization/write
failure through the actual ignored-entry recorder boundary. Successful sidecar
writing after such a failure must not turn it into a valid numerical result;
sidecar failure must remain an externally visible invalid process result.

### High — category reconciliation and actual evaluator-error classification are not retained

`Grid40OperationEvent` at `solver_covered_solve.rs:129-134` contains only
`operation`, `admitted`, and `outcome`. An ordinary strict evaluation therefore
emits indistinguishable `strict_evaluation/started` records for its strict,
ordinary, and complete charges. The summary retains aggregate `errors` and
`inflight`, not per-category terminal classifications. Raw events cannot
independently prove the required per-category equation
`attempted = completed + error + in-flight-at-interruption`.

Further, strict-search and halved-witness `Err` values are discarded at
`solver_covered_solve.rs:1293-1298` and `:1340-1360`; the trace says only
incomplete/error and does not retain the actual typed error classification
required by the contract. Give every accounting event an explicit counter
class (base, witness, strict, ordinary, complete, signed probe, domain,
assembly, or update) and retain typed evaluator-error classification in the
event/ordinary trace. Add a raw-event-to-summary reconciliation test covering
success, evaluator error, and interruption. A bounded one-shot `cfg(test)`
strict evaluator fault is acceptable only through the actual strict wrapper,
with mutually exclusive RAII replay/fault guards so it is impossible in either
original replay arm; it must not change production code or supply alternate
physics/results.

### High — the frozen prerequisite vector set and C control are incomplete

The exact T patch has direct tests for range/count selection, predicate-entry
counting, isolated single-counter denial, aggregate evaluator lifecycle, a
real b21 installation, and exact-bound refusal. It does not directly bind all
mandatory prerequisite behaviors. The independent frozen-source map identifies
no real covered strict-search duplicate-rounding test, no strict evaluator-Err
test, no full-solver no-update/no-install test, and no combined tiny-step plus
unacceptable-residual test. The frozen T exact-bound vector also asserts only
terminal refusal/count/unchanged solution; it does not assert the decisive b40
bitwise duplicate, tiny/zero governed step, residual still unacceptable, or
the attempted exponent/event sequence. Wrong-case/nonproduction confinement
also lacks a direct negative entry test.

The named immutable C patch is not a green final control: its treatment helper
still returns `0..=20` while its newly installed test expects `0..=40`. Its one
recorded actual-callsite red is discriminating, but it does not establish a
green preservation cut. The later reconstructed C patch
`1ea21a418677f6d8642340e6fa63c1172f91631fc6ce3a5eebb0b9408069bb91`
records 8 PASS/1 ignored and the real-callsite b21 red retrospectively, after
the earlier exploratory T existed. That reconstruction is useful and its
chronology can be reported honestly, but it does not bind tests added after
that reconstruction.

Before final T, install the complete final test set in a fresh C reconstruction
whose helper exposes the intended private range while the actual strict call
site remains hard `0..=20`; run all unchanged/preservation tests green and the
real b21 call-site vector red; then make the single treatment call-site/range
change and run the same tests green. Preserve the earlier scaffold as
exploratory and do not claim the full tests preceded it. This corrected
reconstruction sequence is sufficient for the final candidate's contract-first
control, but the currently supplied frozen C/T pair does not yet supply it.

### Medium — counter session identity is not bound to the installed grid

`grid40_begin_call_counts(_grid)` at `solver_covered_solve.rs:153-159` ignores
its argument, while ceilings and the actual range read a separate thread-local
grid. The current ignored entry happens to install the grid first, but the API
permits a treatment-labelled counter session with baseline limits/range (or the
reverse). Bind the session to the installed grid and fail before solving on a
mismatch. Include the bound grid/policy identity in the sidecar and exercise
the mismatch refusal.

### Medium — nested component-limit reporting is not demonstrated

The sidecar emits Grid40 counters only. The frozen implementation does not show
where the contract's unchanged nested component limits/work are reported
separately, and none of the supplied focused assertions reconcile that evidence.
Name the existing authoritative nested counters/limits and serialize or bind
their report for each arm, or add the smallest test-only observation needed to
prove their unchanged enforcement. Do not duplicate their algorithms or
replace existing component guards.

## Confirmed unchanged behavior

Static C-to-T comparison changes the actual experimental strict selection from
fixed b0..20 to fixed b0..40, adjusts one event-count expectation, and adds the
exact-bound test. Outside `cfg(test)`, the canonical strict range and exhaustion
increment remain `MAX_BACKTRACKING_HALVINGS` (20). The shared constant, b1..20
halved no-update witness, scaling, finite-difference stencil, LU/pivot rules,
acceptance-before-iteration-limit order, update installation order, domain
predicate body, constitutive equations, normalizers, and accepted-candidate
constructor are unchanged. The domain counter is placed at function entry to
`covered_trial_is_valid`, so it covers every invocation rather than individual
scalar comparisons. The four prior post-refusal evaluator probes are removed
from both arms.

The actual b21 fixture is discriminating: supplied C call-site evidence refuses
before installation, while exact T records b21 installation. The exact-bound
fixture's terminal assertions are consistent with fixed exhaustion at b40 and
unchanged failed solution, but the missing decisive assertions above still
matter.

## Authority and validation mapping

Manual detached-source mapping confirms that the only numerical choice intended
to vary is INV108's private strict range/exhaustion count. INV112/INV138/INV139
remain source-identical in scaling, stencil, witness, acceptance, and no-update
logic. No constitutive relationship or conservation/publication equation is
changed, so no new A3 constitutive suite or A0 closure computation is triggered
by the Rust diff itself. The main-checkout A0 authority-admission PASS establishes
canonical wording only; it is not detached Rust workflow evidence.

For the final frozen candidate, required source-specific evidence is: the
complete contract-derived focused vectors above; owning-crate tests; default and
`test-support` compilation/tests for changed trace paths; rustfmt; identical
baseline/candidate warnings-denied Clippy with no introduced/changed diagnostics
under the prospectively selected inherited-lint policy; static proof that the
treatment selector remains `cfg(test)` and no public/runtime consumer can reach
it; exact raw-event/category reconciliation; and the concrete arm-recorder
write-boundary review. The already accepted A0 authority admission may be reused
for unchanged canonical text. Constitutive A3 is `NOT_APPLICABLE` to the final
diff only if that diff still changes no constitutive formula, guard, scale, or
component limit.

## Residual risk and later independent reconstruction

No original case or result-bearing pair has run. If the pair later reaches an
actual Accepted treatment, independently reconstruct at least: the decisive
last base residual/norm; Jacobian and RHS to the installed direction; the b21
coordinate update from `x + 2^-21 delta`; domain distances and branch identity;
strict norm decrease; governed steps; final finite normalized residuals and
constructor admission. If it refuses, reconstruct the first decisive
obstruction and the fixed b0..40 domain/evaluation sequence instead. In either
case reconcile every event class, counts, accepted exponents, and zero extra
post-refusal probes against the ordinary trace. This reconstruction must use
recorded primitive operands, not call the solver or reuse its acceptance helper.

Inspection failures: **4**. One attempted a stale/nonexistent reconstruction
script name, one used the wrong patch strip depth before the successful
reconstruction, one local JavaScript orchestration expression was malformed,
and one temporary-cleanup shell command was rejected by the tool policy. All
affected inspections were subsequently completed with corrected read-only
commands; no Rust/evaluator/model run occurred.

## Verdict

**HOLD — no implementation approval and no result-bearing launch.** The frozen
T preserves the intended ordinary numerical method outside the named private
range, and the supplied b21/exact-bound evidence is useful, but the counter
admission, evidence-integrity, reconciliation, prerequisite-vector, final-C,
and current-source-quality gaps above are blocking. Re-freeze the corrected C
and T cuts with exact hashes; this reviewer should verify only the affected
fixes and final drift before any launch recommendation.
