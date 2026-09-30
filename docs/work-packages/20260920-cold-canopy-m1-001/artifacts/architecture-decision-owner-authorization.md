# COLD-CANOPY-M1 — bounded architecture and deployment-cost decision

Draft for owner adoption. Executing this prompt adopts only the decision study
below. Authoring it does not start execution. Record the adopted scope and final
decision at the front of the existing
`docs/work-packages/20260920-cold-canopy-m1-001/package.md`, the only maintained
narrative. No numerical implementation or new physical execution is authorized.

## Decision to make

Determine whether a substantially cheaper numerical architecture can preserve
the required M1 physical equations at the adopted deployment cost, or whether
meeting that cost requires an explicit model/cadence tradeoff. Recommend one
path and one small, falsifiable successor experiment. A justified conclusion
that no evaluated path is presently credible is acceptable; another indefinite
sequence of solver corrections is not.

**Runtime is a feasibility constraint, not an eventual optimization goal.**
Start from the existing CPU ceilings per completed OFE-day: stable cold 750 us,
mixed phase 1.5 ms, transition 2.5 ms. Retain existing map-count, scaling, memory
and warm CPU/wall-regression screens. The owner’s scale concern is a 100-year,
five-OFE run and roughly 100x improvement over the discussed proposal cost.
Recover any recorded whole-run wall target from its actual source; do not ask
the owner to repeat existing targets or manufacture a new one from extrapolation.
Keep proposal, support, completed OFE-day and whole-run costs distinct.

## Pinned evidence and scope

Start from local evidence commit
`b6ac817300ba9ce449298f30afb7736d9e35509a`. Inspect the package’s two completed
performance experiments and their decisive raw summaries, source identities and
review conclusions. Verify source rather than substituting main Rust for the
detached experiment. Cached frozen14 treatment source is
`91fae0d951262b38dbd06d616f27e2426e474f2206125daab8c774798b2f94a7`;
recovery and profiling-source metadata are linked in package.md.

Measured reference medians: initialization 100.305 us CPU / 104.387 us wall;
proposal 896.679 / 901.309 us; combined 1000.618 / 1010.427 us. Enabled-observer
wall attribution gives SVD 37.66%, other Stage1 24.71%, damping search 19.17%,
refinement 9.08%, and all core evaluations 4.17% of proposal cost. These are
wall fractions, not component CPU measurements. The whole Stage1 union is
92.52% of proposal / 83.42% of combined wall cost. Making that union free would
yield only 13.368x proposal / 6.034x combined speedup with other work unchanged.
Those bounds constrain local changes; they do not prove a different architecture
cannot change initialization, evaluation frequency or other work as well.

The G4 seam stops at `transition_model`; it has no accepted nonlinear step,
full solve, hydraulic/materialization completion or completed-day denominator.
Do not infer iteration count or full-run feasibility from its successful return.
All previous integrity stops, failed results, original physical target slots,
native integration and production/full-M1 holds remain unchanged.

Read applicable root/package/science instructions, numerical-solver architecture,
kernel-preparation and direction-review guidance, then affected canonical LSE,
vegetation and support-cadence authority. Use targeted dependencies and actual
call sites, not a full reread of the historical record. RHESSys comparison must
use the pinned RHESSysEastCoast source `375c75b1cd2202217651dff43aa113d80b9c1118`
and identify that fork; do not generalize it to every RHESSys version. Inspect
primary sources for scientific claims and cite exact source locations.

## Bound and ownership

Proposed allowance: **5400 charged seconds**, including **1800 seconds** for
independent review, preservation and disposition. Carry at least
`437600.393795` charged seconds; new fixed cumulative ceiling `443000.393795`.
Verify the latest closing receipt once; any greater carry reduces the available
balance. Do not add the old 1086-second remainder or refund historical charges.
Anchor one UTC deadline at first execution, including reading, and charge all
elapsed waits/concurrency once without re-anchoring. Commands must fit the work
window plus reserve; no single command over 180 seconds. Return a bounded
incomplete decision if the evidence cannot be resolved within the allowance.

Astra owns scientific/technical direction. Bounded source extraction may be
delegated under repository role routes. Obtain distinct independent correctness
and QA/evidence assessments of the proposed decision; neither may review its
own design. At most two concurrent children, no nested delegation. Reuse the
same reviewers for corrections and keep review proportional to this static
scope. Review does not authorize or validate an unexecuted successor method.

Only analysis, read-only source inspection, primary-source retrieval and small
offline arithmetic/symbolic scripts are permitted. Scripts may analyze retained
operands and compute budgets but must not run a new solver, physical case,
benchmark cohort or candidate prototype under another name. No kernel/native
edits, production adoption, canonical contract amendment, new dependencies,
branch switch or push. Preserve unrelated work and accepted experiment trees.

## Required analysis, in decision order

### 1. Establish the actual denominator before proposing algorithms

Trace forcing records, accepted supports, parent intervals, occupancies/columns,
nonlinear updates, rejected proposals and adaptive subdivisions from the real
intended caller to the solver. Identify what is implemented, merely proposed,
and still unconnected. Distinguish a forcing-record interval from a mandatory
solver interval; neither assume equivalence nor assume records may be skipped.

Derive supported lower bounds and conditional scenarios for solves/OFE-day and
proposals/solve in stable, mixed and transition regimes. If iteration counts are
unknown, show them as variables with clearly labeled illustrative sensitivity
cases, never as observed averages. Account for initialization frequency, warm
state reuse if actually permitted, hydraulics, ownership/materialization and
other induced costs. No complete-path run is authorized to fill missing data.

For each regime express the necessary budget as
`N_solves * (C_setup + N_proposals * C_proposal) + C_other <= B_day`,
refining it if rejected supports or repeated assemblies need separate terms.
State which costs are measured, statically bounded, estimated or unknown. Use
`C_other=0` only as an explicitly optimistic bound. Derive the resulting allowable
cost per solve/proposal. If cadence alone makes current costs untenable, put that
finding first; do not bury it behind a preferred algorithm.

### 2. Map mathematical coupling and constraints

Map all 21 coordinates and residual rows to physical owners, algebraic/state
roles, conservation equations, phase/capacity constraints and dependencies.
Identify which unknowns require simultaneous coupling, which admit exact local
elimination or a Schur complement, and which would require approximation to
split or lag. Support claims with equations and actual derivative structure,
not visual sparsity of one captured matrix. Include validity at wet/dry,
freeze/melt, bound and rank-changing transitions; avoid a G4-only shortcut.

Explain why SVD, active-face searches, fixed damping bisections and compensated
refinement were introduced, distinguishing physical requirements from choices
of numerical representation. Determine whether simpler methods can preserve
the necessary admissibility, convergence and conservation conditions. Do not
assume that replacing SVD with a cheaper factorization removes the surrounding
conditioning or constrained-step problem. Do not treat existing solver details
as immutable physics, nor discard their failure protections without an account
of the conditions they protect.

### 3. Compare at most three serious paths

Evaluate a compact set, including preservation of the equations and an explicit
model simplification if warranted. Candidate classes are:

- Structured/reduced coupled solve with justified analytical elimination and
  treatment of active physical regimes, rather than the present general BVLS
  machinery on every proposal.
- Partitioned or sequential updates retaining the process equations, with a
  clear account of splitting/lagging error, feedback, stability and conservation.
- A simpler RHESSys-informed process formulation or different authorized cadence,
  identifying exactly which science and outputs would change.

These are comparison classes, not an instruction to design three complete
solvers. Merge or reject unsupported alternatives promptly. RHESSys speed is not
evidence of equal physics or acceptable error for this application. Physical
regime dispatch must be defined from state, not chosen after solver failure;
no historical fallback chain is an acceptable architecture.

For each serious path, give a source-supported work model: expensive evaluations,
matrix dimensions/factorizations, iterations, initialization and cadence. Provide
optimistic and plausible conditional cost ranges tied to measured components
where transferable. Explain transfer limitations; a FLOP count is not a timing.
State whether the path can plausibly satisfy the per-solve allowance from step 1.
Do not multiply overlapping speedups or project one G4 case across an entire run.

Separate algebraically equivalent reformulation, changed numerical accuracy,
changed temporal approximation and changed physical model. For any approximation,
identify affected ET/runoff/storage/phase timing and cumulative water/energy
balance, required reference cases and proposed error-budget rationale. Do not
invent tolerances and present them as owner-approved scientific acceptance.
Preserve independently testable admissibility and accounting obligations.

### 4. Make one decision and specify its falsification

Select one path, reject the others with concrete reasons, or state that none
has a defensible deployment-cost path with current evidence. The decision must
follow the derived runtime allowance, not familiarity with a solver or sunk work.
If one missing fact prevents selection, identify exactly that fact and the
smallest bounded way to obtain it; do not replace the decision with a generic
request for further investigation or a broad literature survey.

Specify one successor experiment only: scientific hypothesis, affected authority,
minimal implementation surface, fixed representative cases including difficult
transitions, independent numerical/conservation reference, proposed accuracy and
runtime acceptance, run/time limits and stop conditions. It must distinguish an
algorithm that is cheaper per iteration from one that completes the required
work cheaply. State what result would reject the chosen direction before more
engineering is invested. Proposed scope/tolerances remain subject to adoption;
do not execute the experiment or amend contracts in this decision step.

## Delivery

Update the existing package with a compact decision brief: runtime requirement
and cadence-derived allowance first, coupling findings, at-most-three comparison
rows, selected path, scientific tradeoffs, feasibility uncertainty and the single
falsification experiment. Link raw arithmetic/source extraction only as needed.
Use `Static:` for this analysis and distinguish inspected prior `Ran:` evidence
from newly executed offline calculations. No new performance claim follows from
the static design. Reviewers must check denominator/cadence reasoning, equations,
cost transfer assumptions, independent error criteria and authority implications.

Commit scoped package/evidence locally after review; no push or main Rust
adoption. Preserve recoverable calculations, exact evidence provenance, reviewer
findings and charged time. Return a clear architecture decision or precise
incomplete finding. Do not claim deployment readiness or authorize another
correctness-only implementation increment.
