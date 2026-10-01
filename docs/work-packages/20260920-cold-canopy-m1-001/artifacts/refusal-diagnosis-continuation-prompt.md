# COLD-CANOPY-M1: diagnose day-solve refusals and disposition repair feasibility

## Adoption and objective

This is a proposal until the owner invokes it. Invocation authorizes only the
bounded diagnostic work below, including the three named witness captures if
existing evidence is insufficient. Authoring or reading it does not launch work.
Use `docs/work-packages/20260920-cold-canopy-m1-001/package.md` as the sole maintained
record. Incorporate this adopted delta there; retain the original A/B protocol
and its failed results as historical evidence, not a schedule to restart.

The control gate now passes. The hold is zero complete reference/candidate days:
all six reference/stability attempts and all24 A cells fail; all18 B cells are
unavailable because their matched baselines fail. Two accepted support0 records
reconstruct independently, but both associated days fail at support1. Diagnose
whether the three observed refusal classes reflect a demonstrated implementation
error, a correct guard rejecting a bad proposal/state, a conditioning/rounding
limit, or missing evidence. Do not assume another test-oracle defect or loosen
acceptance to make the experiment pass. Deliver a concrete, independently reviewed
repair decision with a credible runtime argument; do not start a solver campaign.

## Source, evidence and custody

Start from `f393685c7dcf6508d6ecab27908da5bfabd2bc52` plus the package's later
hold-disposition/prompt record. Verify actual repository state and preserve all
unrelated changes. No branch change, push or main Rust adoption.

Read the leading package results and these artifacts first:
- `c6-oracle-results.json`, `c6-oracle-terminal-state.json`;
- `c6-oracle-terminal-correctness-review.json`, `c6-oracle-terminal-qa.json`;
- `c6-oracle-run-evidence.tar.gz` with its member-hash manifest;
- provenance-bound `c6-oracle-partial-reconstruction-input.json` and report.
All paths above are relative to the package's `artifacts/` directory.

Authenticate and recover `c6-oracle-source-cut02.tar.gz` with its JSON receipt
into a new isolated diagnostic copy. Source identity is
`e900961e8e7b902f1cc13e01344275769e4241d3f4f1cf4e6b0a399ab03760c3`;
fixture hash is `d3616640db3e7c689162fcf5fa0e71c76ecd9b068beb070cacf3157fc97d8362`.
Verify every receipt-recorded link, including `.venv`; never restore links through
an existing directory symlink. Preserve frozen source/binary/run directories.
The old release test binary hash is
`c94275f4fe236ef35c5d6c80a42a0f73e59cdad1296afb419a5c817ff95725fb`.
An instrumented source requires its own build and custody evidence.

## Performance-first diagnostic boundary

Retain targets:500us CPU/550us wall per complete OFE-day; regime CPU750/1500/2500us;
10-OFE,36,525-day century <=182.625s CPU/210s wall. The observed4.920672–10.316618ms
CPU values are failed partial attempts, not completed-day costs or matched timing
samples. They do not establish a speedup, a lower bound for a redesigned method,
or century runtime.

Before proposing further numerical work, inventory the measured work counts and
existing component-cost evidence. At60s cadence,1440 joint two-occupancy solves
share the OFE-day budget:500/1440 =0.347222us CPU per solve even with every other
cost free; regime ceilings imply0.520833/1.041667/1.736111us. Do not count the two
occupancies as separate solves. Distinguish budget arithmetic from measured cost.
A correction that removes a refusal but retains an unsupported cost path is not
a deployable recommendation. Any repair proposal must state its expected work
and cost, evidentiary limits, missing native costs and remaining target gap. Do
not solve the cost problem by silently changing cadence, forcing or acceptance.

## Static diagnosis, then only necessary captures

Use existing raw operands and static source first. Inspect the relevant retained
refinement feasibility checks, physical mapping/admission and canonical science
contracts, following the repository instruction routes. Keep ADR-0044's canonical
solver rule, typed guards and numerical architecture binding.

Predeclare these representative witnesses; no adaptive case/profile selection:

| Witness | Frozen cell | Failure to explain |
| --- | --- | --- |
| W1 | A_cold_ice_day, FULL/P0,60s, support0 | refinement_scratch_radius |
| W2 | C_freeze_then_melt_day, FULL/P2,60s, support0 | refinement_final_box |
| W3 | C_freeze_then_melt_day, REDUCED/P0,60s, support0 | VEG-E-142/MhConsistency |

Counts in the A table are19 scratch,4 final-box and1 M/H refusal; six additional
reference/stability entries are scratch failures. Three FULL/P0 A entries reuse
references, so do not count these as extra physical executions. W1 captures the
one original FULL/P0 reference invocation that the A row reuses; do not launch
separate reference and A copies.

For W1/W2, reconstruct the exact compared norms/bounds and signed violation,
units, scaling, ULP distance, arithmetic order and guard threshold. Bind current
base, candidate and scratch/final corrections, full/reduced coordinate maps,
active/free/held masks, fixed-coordinate contribution, original/face trust radii,
lambda, J/r/scales, factor/refinement stage and work counters needed to reproduce
the failed inequality. Serialize binary64 bits and ordered coordinate/row IDs,
including scaled bounds, successive corrections and squared/unsquared norms as
reached. Preserve the existing strict inequalities; do not invent a tolerance.
Distinguish full radius from reduced-face radius and
ordinary from compensated corrections. Independently test the actual witness
with higher-precision or exact operand arithmetic where useful; do not treat a
synthetic nearby example or candidate-supplied expected value as proof.

For W3, trace the rejected trial's M/H/T operands and phase, their beginning versus
current roles, proposal/mapping provenance and exact canonical consistency rule.
Record occupancy/index, coordinate indices, raw M/H bits, the exact rejecting
`from_mass_enthalpy` predicate and call site. If construction fails, canonical
phase is undefined: do not infer or relabel it. Identify whether the state is
inconsistent, the map/derivative/control response
is wrong, or the existing evidence is insufficient. Do not clamp a phase/state,
alter guard order, or relabel a domain failure as convergence.

If required operands are absent, add only detached test-only observation and a
single-purpose failure-dump entry point. Capture the naturally reached first
refusal of each named support using the same original initialization/solver/
profile/input. Do not construct a more convenient state or suppress earlier
refusals. Observation must not change decisions, evaluation count, work limits,
precision or ordering. Freeze fields and independently review noninterference
before result-bearing invocation; cap output and emit only necessary operands.
These are newly authorized diagnostic reproductions, not replacements for the
failed A results. Captures are not performance benchmarks.

## Finite allowances and validation

Propose no additional charged seconds: retain ceiling458534.927980s. Reconcile
latest package carry including this prompt's authoring and all return tails;
exclude only documented inactive owner-decision intervals. At first execution
work, including reading, anchor R = ceiling minus carry once. If R <=600s,
preserve and return without launch. Otherwise hard deadline=start+R; work cutoff
is600s earlier for closing. Invocation explicitly renews earlier expired UTC
windows by this formula; it does not reset charges. Readiness for any capture is
capped at600 charged seconds within R. Every command <=180s and must fit before
closing; all active work/waits/concurrency count once.

Allow at most one result-bearing invocation per W1/W2/W3, fixed order, only when
needed for missing operands:30 elapsed seconds each,90 seconds aggregate. This
is a separate named diagnostic allowance; preserve prior physical2.265900994s
and prior historical control0.053829565s as separate totals. No generic replay,
corrected replay, extra seed, parameter search, full-day run, reference/stability
rerun, A/B sweep, timing batch or fresh control campaign is authorized. Any core/
Jacobian/solver preflight consumes its witness slot. A failed or incomplete capture
is preserved without retry; remaining independent captures may proceed unless
custody/integrity fails. An unexpected first outcome is data, not license to tune.
Offline arithmetic on retained operands is not a physical invocation.

Astra orchestrates one implementer and distinct correctness and QA reviewers,
at most two concurrent children, no nested delegation. Reuse accepted unchanged
review portions. For instrumentation, select focused format/no-run build,
source-bearing lint comparison and fresh source/build/tool authentication; no
physical preflight disguised as compilation. Require an approved sealed receipt
before captures. If source inspection alone suffices, omit instrumented build and
captures and state that explicitly. Never require redundant controls merely to
populate an evidence checklist. Apply applicable authority/anti-evasion checks
only if their triggering surfaces actually change; this prompt does not authorize
suite, fixture or guard changes.

## Deliverable and stop

For each witness preserve evidence class, raw identity, exact refusal inequality,
independent reconstruction, known/unknown cause and whether it represents the
broader failure class. Do not generalize one witness to every profile without
source/operand support. Distinguish verified implementation defects from valid
negative outcomes and hypotheses. Preserve missing evidence honestly.

Return one bounded next-step recommendation: (a) a demonstrated local correction
with precise affected function/contract, test acceptance and cost implications;
(b) a method/conditioning issue needing a separately authorized replacement; or
(c) insufficient evidence/runtime feasibility, with the precise missing fact.
Multiple refusal classes may have different dispositions. Rank the blocking
issues and explain which must be resolved before a meaningful day/cost comparison.
A suggested correction is not implemented under this diagnosis-only authority.

No numerical-body, solver, tolerance, rank/feasibility threshold, guard, physics,
fixture, forcing, cadence or production change is permitted. No fallback chain,
new dependency, authority weakening or silent surrogate. Do not automatically
restart measurements even if a likely repair is obvious.

Update package.md with disposition, quantitative runtime constraints, source and
selected checks, independent reviews, exact terminal diff and final charged time.
Preserve recoverable instrumented source and raw captures when created; commit
only scoped documentation/evidence/source recovery locally, no push. Stop at the
reviewed diagnosis. Controls remain accepted historical evidence; scientific and
runtime HOLD remains until a separately authorized repair establishes the missing
complete-day evidence.
