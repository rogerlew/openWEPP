# COLD-CANOPY-M1 — bounded performance recovery: damping-search projection cache

Draft for owner adoption. Executing this prompt adopts only the experiment,
allowance and comparison cohort below. Authoring it does not start execution.
Incorporate adopted scope at the front of the existing
`docs/work-packages/20260920-cold-canopy-m1-001/package.md`; that remains the
only maintained narrative. Do not create another governance framework.

## Objective and practical acceptance

Deliver one measured, behavior-preserving optimization of the private G4
proposal: compute the invariant projection used by the damping search once
per factor/right-hand-side pair instead of reconstructing it for every lambda.
Establish a comparable release baseline and treatment with optional diagnostic
capture disabled, while preserving algorithmic checks and work limits.

The owner requires deployable speed. Correctness alone is not success.
Existing M1 CPU ceilings per completed OFE-day remain stable cold 750 us,
mixed phase 1.5 ms, transition 2.5 ms, with the existing map-count, scaling,
memory and warm-regression screens. The scale concern is a 100-year, five-OFE
simulation and approximately 100x improvement over the discussed proposal
cost. Do not replace these with a newly invented target or ask the owner to
repeat recorded targets. These slice CPU ceilings are not whole-run wall-time
ceilings; recover any additional whole-run target from its actual record.

This experiment can establish a proposal-cost improvement, not completed-run
feasibility. Lead the final result with baseline/treatment CPU and wall cost,
speedup, remaining gap and whether the evidence supports further investment.
Passing numerical checks without demonstrated cost improvement is a negative
performance result. Do not expand the work to manufacture a positive result.

## Exact start, custody and time bound

Read root/nearest instructions, the current package summary, numerical-solver
architecture, testing strategy, bounded execution and affected LSE numerical
authority. Use targeted source/evidence reading; do not reread the historical
package end to end. Root guidance at commit
`1d0782751bb355158bc587563bc9a045094ecee5` records the owner's runtime priority.

Start from accepted detached cut09, source tree
`04465bc08aa8d991b856a6caf130bc8543c5a2bb6608939bdf6ed9821bb38e3b`,
using `artifacts/bvls05-terminal09-recovery.json` and
`artifacts/bvls05-terminal09-source.tar.gz` in the current package.
Archive SHA-256: `8c8bb484098c98abb19147fe158bfad3522a5efd8115bb6f31249fcc40a292ad`.
Verify these identities from actual evidence before use. Preserve the accepted
tree at `/home/roger/openwepp-experiments/cold-canopy-m1-20260920`; use separate
baseline/treatment experiment directories, without creating/switching branches.
Do not start from the unaccepted native candidate or main Rust.

This adoption lifts the prior stop only for this isolated experiment. Native
integration, G3 completion, original target prefix/baseline/treatment/repeat,
parent/receiver/cycle progression and production adoption remain stopped.
All previous failures, spent runs and integrity findings remain historical facts.

Proposed new allowance: **7200 charged seconds**, including **1800 seconds**
reserved for review, preservation and disposition. Carry at least
`427743.393795` charged seconds; new cumulative ceiling `434943.393795`.
Verify the latest actual closing receipt once; a greater carry reduces available
time under that fixed ceiling. Do not add the old 1122-second balance or refund
old charges. At first execution, including reading, anchor one UTC deadline;
no re-anchoring or wait deductions. Commands must fit the remaining work window
and reserve. No command exceeds 180 seconds without an already applicable,
explicitly authorized exception. Expiry returns an honest incomplete result.

Astra orchestrates one implementer and distinct independent correctness and
QA/evidence reviewers under repository role routes, at most two concurrent
children, with no nested delegation. Reuse reviewers for fixes. Use immutable
source snapshots for result-bearing commands. No writer edits a measured or
validated source tree during a command; source mutation invalidates the result
and stops this experiment. Routine in-scope corrections need no owner prompt.

## One optimization, with explicit exclusions

Inspect `m1_trust_region_stage1.rs`, especially `lambda_step()` and
`svd_lambda_ball()`. The present inner loops reconstruct `A V`, divide by the
singular values, and accumulate `U^T y` for each lambda, although the factor and
right-hand side are invariant inside that search. Recorded G4 uses free sizes
19, 18, 19, 18 with positive multipliers. Static accounting attributes 1,380,960
multiply-and-accumulate terms to projection reconstruction during the four
48-step bisections alone. This is an operation estimate, not measured time.

Extract and cache only that invariant projection at the narrowest owning
search scope. Compute it using the original traversal, grouped arithmetic,
guard order and binary64 operations. Retain the original lambda-dependent
coefficient calculation, explicit `V` step reconstruction and norm evaluation.
Do not substitute an orthogonality-based norm, use a different factor field to
approximate `A V`, reorder reductions, introduce FMA, change precision, or
change the 48-step search. Cache ownership must prevent reuse across changed
factors, right-hand sides, faces, Jacobians, phase selections or supports.

The helper's refinement callers may have different right-hand sides. Preserve
their existing calculations and guard behavior; do not reuse a search cache
there by implication. Leave all physics, residuals, derivative stencils,
rank/ball/KKT tolerances, BVLS-05 structural eligibility, seed/state, phase
selection, active-set decisions and acceptance predicates unchanged.

This is not a general allocation, scratch-storage, derivative, tolerance,
solver-replacement or physical-model optimization package. Necessary local
cache storage and benchmark plumbing are in scope; unrelated cleanup is not.
No dependency additions, production fallback, main Rust adoption or automatic
continuation to another optimization.

Reconcile the proposed extraction with the actual canonical arithmetic and
work-accounting clauses before editing code. Adoption permits a narrowly
reviewed canonical clarification for once-per-search invariant evaluation
and truthful operation accounting only, if required. Preserve numerical
thresholds and cap values. Distinguish actual work from retained conservative
budget charges; never report eliminated work as executed. If this requires a
different numerical acceptance policy, stop with that concrete finding.

## Controls and finite comparison cohort

Before result-bearing execution, record implementation intent, source/input
identities, applicable validation commands and a fixed comparison protocol in
package.md. Reviewers assess correctness and measurement attribution, not just
whether tests pass. Select triggered validation directly under the testing
strategy; diagnostic isolation does not itself waive Critical requirements.
Do not dispatch broad filters that silently replay historically spent physical
cases. Unmet mandatory validation remains unmet, not retrospectively deferred.

Use independent analytic expectations plus comparison to the immutable original
implementation. Cover zero and positive lambda, radius-search outcomes, changed
right-hand sides/factors, nonfinite/overflow inputs, remaining-rank refusal,
signed zeros, active-face changes, and unaffected refinement callers. Verify
the cache evaluates once per owning search rather than once per lambda.
Expected values must not come from calling the candidate itself.

For this deliberately order-preserving extraction, require bit-identical
numerical outputs on the frozen comparison cases, with identical decisions,
typed failures and input-state preservation. This is regression evidence for
this optimization, not a new bit-exact physical convergence requirement.
Execution counts and optional diagnostic records may differ only in the
explicitly reviewed removal of repeated invariant work.

This adoption expressly authorizes a **new comparison cohort**, separate from
all old run slots, using the unchanged G4 endpoint
`m1_coupled_tests::m1_trust_region_retained_phase_capacity_tie_uses_actual_model`:

1. One diagnostic correctness invocation per baseline/treatment arm, retaining
   every G4 physical assertion and comparing complete numerical outcomes.
2. One warmup batch of 16 fresh proposals per arm, followed by six paired
   measurement batches of 32 fresh proposals per arm. Alternate pair order
   B/T, T/B, B/T, T/B, B/T, T/B. Each proposal starts from the same authenticated
   original input and seed; no evolving state or cross-proposal cache reuse.

Freeze this workload and sampling schedule before measurements. No additional
physical cases, target solves or opportunistic repetition are implied. At most
one affected-arm corrected replay is permitted for an independently verified
implementation/measurement defect, retaining the failed source and evidence;
noise, disappointing speedup or a valid numerical negative is not such a defect.

Both arms must use the same release toolchain/settings, benchmark seam,
fixtures and optional-observer configuration. Run serially on the same host
with no concurrent builds or heavy tests. Disable only optional capture,
serialization and diagnostic logging inside timed regions. Retain all guards,
budgets and algorithmic state; an authoritative counter is not removable
instrumentation. Keep observability changes identical in both arms.

Use a high-resolution monotonic wall clock and supported process/thread CPU
clock, reporting clock resolution and batch duration. Do not reuse the 10-ms
`/proc` tick estimate as precise per-proposal CPU time. Exclude build, process
startup, fixture parsing and printing; report initialization, proposal and their
combined cost separately. Prevent dead-code elimination and compare consumed
outputs outside timed regions. Document any unavoidable observation overhead.
Source/call evidence and diagnostic equivalence must show both modes execute
the same numerical work. If valid minimally observed timing cannot be achieved
inside the bound, return that limitation rather than invent a speedup.

## Decision and delivery

Report every paired batch, median paired speedup and spread for CPU and wall,
with absolute costs. Distinguish the new comparable baseline from historical
5.463608-ms instrumented G4; do not credit instrumentation removal to caching.
An inconclusive improvement remains inconclusive. Preserve all raw outcomes.

Disposition is one of: verified proposal improvement; no demonstrated
improvement; behavior mismatch; or incomplete with the exact blocker.
None constitutes full M1 acceptance or a completed OFE-day/100-year benchmark.
Do not divide proposal wall time by slice CPU ceilings and call it an overrun
ratio. If presenting the one-proposal-per-OFE-day extrapolation, label the
assumption and exclude it from deployment acceptance.

Return cost and practical gap first, then numerical equivalence, review scope,
validation and reproducible source/build/input identities. Preserve a recoverable
candidate and the unchanged reference, with source custody and charged time in
the existing package. Commit only scoped, reviewed documentation/evidence and
necessary recovery artifacts under applicable existing permissions; do not
publish main Rust or imply new push authorization. Leave unrelated work intact.

End this step with a supported recommendation about the next largest cost,
not another implementation. A faster proposal still requires later complete
solve and OFE-day feasibility evidence. The owner should receive a measured
performance decision, not another correctness-only milestone.
