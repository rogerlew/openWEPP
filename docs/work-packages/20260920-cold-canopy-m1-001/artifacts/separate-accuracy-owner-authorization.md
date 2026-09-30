# COLD-CANOPY-M1 — separate reduction, tolerance and cadence investigations

## Owner instruction and objective

Execute two separately attributable investigations: (A) exact algebraic reduction
at fixed 60-second supports, with matched numerical-tolerance sweeps; (B) coarser
supports with one solver/formulation and matched tolerance held fixed. Relaxed
accuracy is explicitly permitted. Do not ask the owner again whether it is allowed.
Find the practical accuracy/runtime frontier, not another correctness-only result.

This prompt is a proposed execution envelope until the owner invokes it. Adoption
authorizes the isolated implementation, diagnostic cases and finite sweeps below.
It does not adopt their eventual error levels for production. The earlier combined
falsification plan is superseded: its proposed 0.1% flux, 0.01 K, 60-second event
timing and half-budget screens are not mandatory application acceptance gates.
An accuracy miss is data, not permission to retune cases or abort the whole sweep.

Practical anchors already adopted: complete 10-OFE, 36,525-day century <=182.625 s
CPU /210 s wall; representative mean 500 us CPU /550 us wall per OFE-day; M1 regime
CPU ceilings stable cold750 us, mixed1500 us, transition2500 us per completed
OFE-day. Preserve existing scaling, memory, physical-map and warm-regression
requirements. These are distinct denominators; a column-day uses some of the
OFE-day budget and does not include every native owner/receiver. Do not invent a
five-OFE target or treat 100x proposal speedup as the deployment requirement.

## Fresh-agent starting context

Repository: `/workdir/openWEPP`. Main evidence commit:
`20f02922d` (resolve and record its full identity). The current package's leading
uncommitted section, **Owner direction — separate reduction and cadence; permit
accuracy tradeoffs**, is authorized user-directed work. Preserve and incorporate
it; do not reset it. Preserve all other dirty/staged/untracked work.

The sole maintained narrative is
`docs/work-packages/20260920-cold-canopy-m1-001/package.md`.
Read its leading owner direction, architecture decision and two performance
summaries, then only affected source/authority. Do not reread its entire history.
Relevant artifact prefixes are `architecture-decision-`, `cost-attribution-`
and `projection-cache-`. Source/recovery metadata and independent reviews are
linked there. Verify actual identities, not summary assertions alone.

Cached frozen14 treatment source:
`91fae0d951262b38dbd06d616f27e2426e474f2206125daab8c774798b2f94a7`.
Recover it using the retained `projection-cache-recover.py` if needed; leave
retained trees unchanged and create isolated experiment directories without
branch switching. Do not substitute main Rust or the failed native candidate.
Main Rust has never adopted the private G4 controller.

Prior measured reference CPU: initialization100.305 us; proposal896.679 us;
combined1000.618 us (independently computed medians). Component attribution is
wall-only: Stage1 occupies92.52% of proposal and83.42% of combined time. G4
stops at `transition_model`; it is not an accepted step, full solve or day.
The fixed diagnostic provider requires1440 joint two-occupancy solves/day;
it permits only0.521/1.042/1.736 us per solve under the regime budgets with
everything else free. Two occupancies are not two independent solves. Exact
reduction alone at this cadence may remain impractical; that does not prevent
measuring its benefit independently from a cadence change.

Apply root/nearest/package/science guidance and affected LSE, vegetation,
coupled-time, numerical architecture and testing standards. Read affected
equations, acceptance/materialization and support custody before implementation.

## Explicit scope, allowance and ownership

Authorize a detached test-only column-day harness and the minimal full/reduced
solver variants needed for these comparisons. Retain actual M1 physics, both
occupancies, hydraulic/final materialization work and state continuity through
each diagnostic day. No native production/provider integration or original
target-prefix/baseline/treatment execution. This is a newly authorized diagnostic
corpus, not a hidden rerun of old physical slots. No production admission,
historical fallback chain, new dependency, branch switch or main Rust adoption.

Prospective diagnostic authority amendments are allowed only for exact reduction,
explicit tolerance profiles, and support coarsening within the specified forcing/
parent boundaries. Review them before numerical body changes. Existing production
defaults and historical acceptance/results remain unchanged. Do not manufacture
surrogate physics, change forcing or clamp invalid states to obtain convergence.

Proposed new allowance: **14400 charged seconds**, including **1800 seconds**
for final review, preservation and disposition. Carry at least441303.393795 s;
new fixed cumulative ceiling455703.393795 s. Verify any greater actual closing
charge once; it reduces available balance. Add neither the old1697-second
remainder nor refunded reserves. Anchor one UTC deadline at first work, including
reading, and charge elapsed waits/concurrency once. No re-anchoring or renewal.
Commands must fit before the reserved closing window; maximum180 s per command.
All physical/reference/timing execution combined is additionally capped at
1800 elapsed seconds; report partial coverage if that cap is reached.

Astra orchestrates one implementer and distinct independent correctness and
QA/evidence reviewers using repository routes, at most two concurrent children,
no nested spawning. Reviewers cannot review their own design. Reuse them for
fix verification. Freeze result-bearing source and inputs; no writer mutation
during validation or measurement. Uncontained custody failure stops execution.
In-scope corrections do not require repeated owner permission.

Spend at most the first60 active minutes on specification/readiness. Freeze one
nonlinear algorithm, exact reductions, tolerance matrix, seed policy, run caps,
reference strategy and test inventory before physical results. Do not let design
expand into a new solver-development campaign. If no implementable common method
fits this envelope, return the precise missing prerequisite and useful completed
static findings rather than silently launching another method.

## A. Exact reduction and numerical accuracy, with cadence fixed

Use the architecture brief's algebraically justified substitutions: seven fixed
ground/soil identity coordinates, the explicit zero-PAR humidity balance with
positive denominator, and zero-structural-area anchors only where applicable.
Propagate all dependencies into residuals/derivatives; reconstruct the complete
21-coordinate state and independently check all original physical rows.
Generally21->13 unknowns, or11 when both documented sun anchors qualify.
Do not equate zero PAR with zero area or remove fully-wet null temperatures as
permanent physical variables. Retain reviewed current-base holds and reactivation.
Drainage/capacity and rank transitions need the stated full-domain treatment.

For the reduction contrast, full and reduced arms must share equations, nonlinear
algorithm/globalization, derivative policy, tolerance profile, initial physical
state, support sequence and work caps. Do not replace BVLS/SVD only in one arm
and attribute the difference entirely to reduction. If the same algorithm cannot
operate on both representations, classify the changed algorithm as a separate
confounder and do not claim a clean reduction speedup. No runtime fallback between
the arms. Retained original code may be an external diagnostic comparator only.

Use four prospective profiles P0-P3. P0 preserves existing thresholds as the
strict comparison. P1-P3 target temperature-step thresholds of1e-6,1e-4,1e-2 K
respectively; these are experimental settings, not measured temperature errors
or adopted application limits. Before any physical run, derive and freeze the
corresponding dimensional humidity, hydraulic, beta, energy, mass/capacity,
inner-solve and final-acceptance settings. Use magnitude-aware absolute/relative
criteria and explain near-zero behavior; do not simply multiply every epsilon.
Inspect both `m1_tolerances` and final materialization: their current min-of-
absolute-and-relative rules can make requirements stricter than an absolute cap.
Keep nonlinear stopping and downstream admission consistent under each profile.

Inner factor/search/KKT precision may be relaxed as part of a profile only through
the same declared algorithm in both arms, with a prospective error rationale and
rank/domain protection. Do not reinterpret rank deficiency as convergence or
silently loosen acceptance after a result. Hold finite/domain/owner validity
separate from numerical error tolerances. Conserved quantities may have finite
residual budgets; missing/duplicated transfers and inconsistent debit/credit
identity are not numerical-accuracy settings.

Produce matched full/reduced results at60 s for every profile. Compare reduction
gain within a profile; compare tolerance gain within a formulation. Report failed
full/reduced cells, not invented ratios. No bit-identical iterative trajectory is
required for algebraic reduction; root equivalence and physical reconstruction
are the meaningful controls.

## B. Coarser time steps, with method and tolerance fixed

Use one common functioning formulation/solver for the cadence sweep, preferably
the reduced arm. Prespecify that choice before measurements; if it cannot complete
the reference cadence, classify the cadence comparison as unavailable rather than
switch methods opportunistically. Use profiles P2 and P3 as two separately labeled
strata. Within each stratum, retain the same numerical settings at60,300,900,1800 s.
Reuse the corresponding60-second A result only when every source/input setting
matches. These contrasts isolate cadence at fixed numerical tolerance.

Consume/authenticate every original forcing record. Coarsen only identical forcing
inside1800-second parents; retain actual forcing changes and parent boundaries.
Do not average nonlinear forcing, cross owner/output/restart boundaries or claim
unchanged fixed-provider receipts. No adaptive hidden subcycling in this first
sweep. A cell that cannot complete its prescribed supports records failure; no
fallback to a finer grid. Report actual interval counts, including split endpoints.

The numerical error of P2/P3 at60 s must be estimated against the strict reference
separately. Report cadence differences against matched-profile60 s, and total
differences against the validated stricter reference. Do not label the former as
pure temporal error if numerical error is unresolved. This60-second reference is
a discrete comparison, not continuous-time or observational truth.

## Corpus, independent references and finite execution

Adopt the three new derived one-day inputs in
`artifacts/architecture-decision-successor-cases.json`, SHA256
`b3ad4fa07c7748c56df0ad9c2fb681be019e86e9bd9befb97d705eb4d10e2ce8`:
A cold ice, B mixed-to-liquid rain, C initial join/cold-to-warm transition. Verify
the manifest/base input/forcing bytes. Preserve initial bits, geometry, both
occupancies, soil/snow boundaries, forcing and duration. These are not slices of
the historical72-hour trajectories. The manifest's proposed cost/accuracy screens
are superseded as gates by this exploratory sweep; retain them only as labeled
comparison lines. Physical regime intents remain coverage obligations: a missing
intended regime means missing evidence, not permission to replace or relabel a case.

Freeze six short controls from the prior brief: empty admissible; empty
supersaturated refusal; directional phase joins; capacity/positive drainage;
fully-wet->dry reactivation; near-bound/rank refusal. Test exact substitutions,
chain-rule derivatives, output reconstruction and failure handling with independent
operands. A candidate routine cannot supply its own expected values.

Establish reference credibility with independently reconstructed original residuals
and mass/energy/transfer integrals, plus a stricter-precision or tighter-solve
stability check chosen before execution. High precision is a reference tool, not
a production acceptance requirement. If no converged credible reference exists,
preserve timing/completion data but label accuracy unresolved; do not turn the
candidate into an unquestioned oracle. A failing strict arm need not prevent
other prespecified profiles from running safely; no accuracy claim can be made
against a failed reference. Limit reference generation/checking to900 s within
the overall physical cap, with commands<=180 s.

Finite maximum: A has3 cases x2 formulations x4 profiles =24 cells. B adds
3 cases x2 profiles x3 new cadences =18 cells. Thus **42 unique candidate cells**,
plus one strict reference trajectory per case and its one predeclared stability
check, and the six controls. No additional cases or adaptive profile search.

For each candidate cell, one complete untimed diagnostic day establishes outputs
and coverage. Only completing cells enter six fixed timing batches of four fresh
complete days each. Freeze a balanced order before running; initialization is
fresh per day, never reset within a day. Per-cell wall allowance is60 s including
diagnostic and timing; a timeout is not a completed-day timing. Numerical/domain
failure ends that cell and is recorded; independently safe scheduled cells may
continue. Integrity failures stop the experiment. Exceeding an old proposed
accuracy limit or the deployment budget does not stop the remaining fixed sweep.

No retry for a valid negative, timeout or noisy timing. At most one corrected
replay of an affected cell/control/reference is allowed for an independently
verified implementation/recording defect, retaining the failed evidence. If a
shared correction invalidates more cells than this permits, stop and return the
affected evidence as invalid rather than silently restarting the campaign.

Use fixed release/toolchain/host settings with no concurrent heavy builds/tests.
Record CPU and wall separately using supported high-resolution clocks. Include
all day initialization, accepted/rejected numerical work, derivatives, factors,
hydraulics, materialization and diagnostic state updates. Exclude build/startup,
fixture parsing and output formatting; document unavoidable observation overhead.
Optional tracing stays outside timing. Consume and check outputs, preserve every
sample and compare timed/diagnostic numerical results under identical settings.

## Outputs and decision

Produce separate accuracy/runtime plots or compact tables for A and B, including
failed/missing cells. Report temperature/humidity/store differences, per-occupancy
dry/wet vapor, ET, upper->lower intercepted drainage mass/enthalpy, lower release,
signed and gross flux integrals, cumulative mass/energy residuals and phase/drainage
timing. Near-zero relative errors need explicit absolute scales. Do not let opposing
flows or occupancy sums hide error. Coarse transition times may be intervals;
report resolution honestly instead of imposing a hidden60-second chronology gate.

Show cost relative to the existing complete-day ceilings without declaring a
column-only result a whole-OFE-day pass. Report unmeasured receiver/owner costs
and headroom needed, not an invented50% allocation. Show the nondominated settings
and where substantially more accuracy buys no meaningful output improvement.
Do not adopt a production error budget from these three days or extrapolate them
into empirical validation or a measured century runtime.

Select applicable validation prospectively under the testing strategy; reconcile
the terminal diff and preserve inherited lint failures. Reviewers verify equations,
profile consistency, independent error reconstruction, fair attribution, custody
and runtime conclusions. Prior reviews cover only unchanged material. At budget
expiry preserve incomplete controls/gates plainly; do not turn partial success
into deployment acceptance or invent a follow-on allowance.

Update the existing package and commit scoped reviewed diagnostic authority,
documentation/evidence and recoverable detached source locally. No push or main
Rust adoption. Return the two separate conclusions, measured error/cost frontier,
whether any setting offers a credible deployment path, missing full-system work,
review/validation status and charged time. Stop after these investigations;
further method changes or production adoption require a new explicit decision.
