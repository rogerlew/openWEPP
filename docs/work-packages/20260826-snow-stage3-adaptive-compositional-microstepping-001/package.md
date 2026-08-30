# Stage-3 adaptive compositional microstepping, phase equilibrium, and production integration

Status: `HOLD — OWNER-AMENDED ONE-DAY OBJECTIVE QUALIFIED; EXTERNAL SCIENCE
AUTHORITY REQUIRED FOR FULL CUTOVER`

Date: `2026-08-26`

Package ID: `20260826-snow-stage3-adaptive-compositional-microstepping-001`

Plan class: critical defect-closure implementation, qualification, and atomic cutover.

This ExecPlan is maintained under `docs/codex_exec_plans.md`; Progress,
Surprises & Discoveries, Decision Log, and Outcomes & Retrospective remain live.

## Purpose / Big Picture

Replace the terminal Stage-3 support/root model with the owner's one selected
temporal model: deterministic adaptive compositional microstepping on exact
60-second (`60_000_000_000 ns`) integer-nanosecond quanta. The accepted
two-child path advances the
complete unpublished owner transaction, bounded vapor enters same-step phase
equilibrium, terminal liquid is consumed exactly once, and the parent continues
through snow-free or reappeared-snow chronology before atomic publication.

## Owner decision and correction authority envelope

The owner directive in `/tmp/adaptive-microstepping.md` supersedes the prior
terminal chronology and phase-competition HOLDs without rewriting their
evidence. It authorizes contracts, production/private APIs, DTO/restart schema
successors, receiver, Batch, runner, tests, authority guards, and compiler-
discovered owners needed for one Stage-3 implementation.

Defect: `CHILD1-TERMINAL-PHASE-COMPETITION-001`. The old transition melts only
beginning ice and applies deposition afterward, allowing positive ending ice
with positive unallocated melt energy and no event. The correction is the
owner-selected enthalpy projection and adaptive compositional chronology.

Immutable red lines are those in the owner directive: no CoE or fixed-step
fallback, no snow-model selector, no sub-60-second carrier, no trial publication,
no state interpolation or evaluated-carrier proration, no independent lane
advance/merge, no duplicate vapor latent energy, and no silent mass/energy/
owner/receipt closure failure. Historical HOLD/rejected evidence is protected.

This package performs science implementation and independent qualification;
calibration is not applicable. Fixture observations are `DIAGNOSTIC_ONLY`.

### 2026-08-27 owner temporal-floor amendment

The owner replaces the package's provisional 600-ms adaptive floor with an
exact 60-second (`60_000_000_000 ns`) floor. This changes temporal admission
and candidate tiling only. It does not change any conservation equation,
bounded-vapor or phase projection, mass/energy/water custody, lane or owner
topology, receipt identity/order, exact rollback, or fail-closed obligation.
Stable ordinary supports must accept steps substantially larger than the
60-second floor; floor stepping is bounded nonlinear/event evidence, not the
ordinary execution target.

Every result whose support grid, floor decision, attempt count, event tick, or
performance depended on 600-ms quanta is `SUPERSEDED` and cannot support the
amended package. Implementation and contract-vector changes plus fresh reruns
are required; this documentation amendment records no replacement execution.

### 2026-08-28 owner performance-objective amendment

The owner pauses seasonal qualification and archive/memory/per-step
optimization after the bounded archive path proved exact 2-day and 60-day
residency. The active objective returns to the canonical exact-60 one-day
microstep workload: identify and reduce limiting phase/fixed-point failures,
especially repeated 64-iteration cap exhaustion and the approximately 1,435
accepted / 1,500 rejected support attempts. Strict ledgers, event chronology,
owner/receipt custody, exact rollback, and fail-closed cap exhaustion remain
unchanged; no persisted diagnostic surface is authorized.

The performance amendment also admits the canonical
`TOL-SNOWENERGY-005` causal receipt termination rule. The retained
snow--soil receipt keeps the exact equal/opposite energy actually consumed;
installed-endpoint reconstruction must be finite and within `1e-9 J m^-2`
and `1e-8 K` before identity-only resealing to the exact installed candidates.
That reseal must replay both owners byte-for-byte. The rule does not relax
physical-ledger closure, topology, support, phase, or receipt custody; larger
residuals retry within the existing cap, with the exact-60 floor remaining
fail-closed.

The current-source P102 replay invalidated the remaining undamped exact-floor
evidence. At support `178380000000000..178440000000000 ns`, the outer solve
reached iteration 96 with LSE, soil, and boundary maps converged while both
Stage 3 lanes alternated density by exactly one binary64 ULP. Continuous mass,
temperature, cold-content, refreeze, and cumulative-energy operands differed
only by a few ULP and were already within their unchanged physical-class
tolerances. `SC-SNOWENERGY-001@25` therefore permits the existing `w=0.5`
policy at the exact floor only after authentic Stage 3 candidates exhibit an
`A/B/A` cycle under the unchanged exact-discrete/native-unit convergence
authority. Intermediate iterates remain unpublished; authentic candidate
density and every discrete/event/topology field remain exact; cumulative
mass/energy closure is revalidated; and final acceptance remains exact
authentic-candidate replay. The floor, tolerances, 96-iteration cap, ledgers,
receipts, rollback, and fail-closed behavior are unchanged. All exact-floor
fixed-point and P102 evidence predating this amendment is `SUPERSEDED` pending
the focused and real-fixture reruns recorded by this package.

Replacement evidence on 2026-08-28: the covered-convergence policy module
passes, including raw exact-floor default, conditional `w=0.5`, below-floor
refusal, authentic one-ULP density-cycle detection, reconstructed
thickness/fingerprint and persistent cumulative closure, and discrete
terminal-event/topology poison refusal.
The result-blind five-completed-parent real-fixture diagnostic also passes its
typed bounded stop in `233.13 s` wall time (`219.12 s` test time). Its
fixed-point audit contains 28 converged 60-second evaluations at 36--52
iterations and 25 converged 120-second evaluations at 18--29 iterations; the
`7440--7500 s` raw exact-floor evaluation converges in 45 iterations and the
`7440--7560 s` stable 120-second evaluation converges in 29. One trial at each
of 120, 240, 480, and 900 seconds reaches the unchanged typed 96-iteration
cap. The maximum converged receipt reconstruction residual is
`9.78616299107670784e-10 J m^-2`, below the unchanged `1e-9 J m^-2` bound.
These are fixed-point evaluation counts, not controller-accepted support
counts; the owner-run canonical one-day v6 remains the authority for the final
accepted/rejected distribution and performance disposition.
The current SnowEnergy contract guard passes and the hillslope-orchestrator
crate checks cleanly. The exact current-source P102 command advanced for
`4381.58 s` without recurring the former exact-60 fixed-point cap at
`1529.60 s`; it then failed closed on a distinct later atmospheric-authority
guard, `weiss_norman_potential=-0.41482265712199873`. Because the fixture helper
performs its own inner `expect`, the later failure unwound before the opt-in
target-support audit could be recovered; no target iteration or whole-run
closure number is claimed from that bounded replay. The solver disposition is
therefore supported by the focused exact-density/closure/event tests and by
advancement beyond the former cap, while the separate atmospheric-authority
failure remains outside this amendment.

A subsequent unconditional exact-floor implementation trial is `REJECTED`
evidence. Although the physical day and ledgers completed, it regressed to
approximately 1,336 accepted and 1,426 rejected trials, with nearly every
parent selecting twenty-eight 60-second supports and one 120-second support;
1,291 rejections were exact-discrete comparison failures and 100 were bounded
fixed-point nonconvergence. This lost the prior 497-accept / 206-reject stable-
support result and violated the owner requirement that ordinary stable
supports accept substantially larger steps. The current conditional `A/B/A`
switch supersedes that unconditional trial: raw-convergent floor maps remain
raw, while only an observed authentic period-two cycle enables damping for the
remainder of its bounded solve.

The SnowEnergy assurance identity for the conditional amendment was
invalidated and adopted through scientific-full transaction
`0af4bb65cce769ba59ac30a67aa5ec05c86bf006b3c206d06faecca772ac78d4`.
`openwepp-assurance validate --all` then passes all three selected reports at
generation `931f2f31c529378f63377c3fda7ea1906654f5d2fd7ef114eb4b0b47e18fe809`.

Final one-day diagnosis on 2026-08-29 corrected a separate comparison defect.
The current conditional solver initially regressed to 1,336 accepts and 1,426
rejects because WB14's per-OFE child receipt map used its transaction-local
64-hex digest keys as a cross-factorization `ReceiptOrdering` surface. A direct
`H` child and composed `H/2 + H/2` children necessarily have different child
receipt identities/counts even when their physical state closes. The audit
also incorrectly reported `ReceiptLineage` as a mismatch although production
already excluded it. `SC-SURFACELIQUID-001@13` now classifies exactly the WB14
child ordinal and `per_ofe_authorities.*.receipts` key/history as exact
per-trial factorization lineage. Each path still retains, seals, and validates
its complete exact receipt chain; all other receipt ordering, event/topology,
custody, rollback, and fail-closed surfaces remain exact.

The corrected five-parent fixture reduced comparison evaluations from 94 to
30, eliminated all 87 exact-discrete mismatches, and admitted candidates
through 1,800 seconds. Later exact chronology, positive-solid forcing,
accepted-carrier identity, and qualification-partition corrections invalidate
the earlier 497/206 result as terminal exact-head evidence while retaining it
as historical solver evidence. The final post-correction canonical one-day
fixture passes with 588 accepts, 320 rejects, 1,078 retained publication
supports, and 59 events. Accepted widths are `139x60`, `111x120`, `320x180`,
`12x240`, `1x300`, `3x420`, `1x900`, and `1x1800` seconds; 76.36% exceed the
fallback floor. The superseding post-terminal-custody optimized body is 420.11
seconds versus the retained
485.858-second baseline. It records 124 bounded fixed-point nonconvergences,
140 scaled comparison rejections, zero exact-discrete/event comparison
rejections, mass residual `3.55271367880050093e-15 kg m^-2`, energy residual
`1.39698386192321777e-9 J m^-2`, and receipt reseal maxima
`9.98625182546675205e-10 J m^-2` / `1.06297193269710988e-11 K`.

The Season A process active when this amendment arrived was terminated with
SIGTERM and is `PAUSED/INCOMPLETE`, not qualification evidence. Season A/B and
further archive optimization remain held until the owner releases those
workloads again.

The rebuilt PL14 cold-canopy fixture exposed a distinct covered-V10 numerical
backtracking defect at the exact 60-second fallback: two exactly zero-area
canopy occupancies had no physical component-energy residuals, but their
inactive sun/shade/wet temperature rows targeted cold canopy air below the
liquid-vapor constitutive domain. Every positive Newton line-search factor was
therefore structurally inadmissible at iteration zero. `SC-LANDSURFACEENERGY-001@11`
now binds `INV-LANDSURFACEENERGY-131`: only those numerically inactive
liquid-vapor coordinates target `max(T_canopy, 273.15 K)`. Zero-area physical
operands remain zero, active physics/tolerances/ledgers are unchanged, and the
exact-60 fallback, events, receipts, rollback, and fail-closed rules remain
binding. The affected PL14 and contract evidence is invalidated pending the
focused reruns recorded by this package.

Focused replacement evidence on 2026-08-28: all 75 land-surface-energy crate
tests pass; the three affected contract suites pass 16/16; the adaptive
warm-start authority-binding guard passes; and affected crate `cargo check`
passes. The real PL14 fixture no longer rejects at the initial 60-second
covered-V10 backtracking seam and advances to the 900-second composed support.
Its terminal fixture disposition remains pending because it then fails closed
on the separate BGC finalized-use inventory join, before the PL14 test can
publish its iteration and independent ledger-closure audit.

### 2026-08-30 fixed-point transition optimization amendment

The owner authorizes execution of the profiling-selected optimization target.
The exact-head one-day cap audit attributes 94 of 124 bounded failures to
Picard/finalization attempts where LSE, soil, and complete-boundary maps have
already converged while Stage 3 remains separated first at per-layer cold
content. The same workload attributes 95 of 140 scaled comparison rejections
to per-layer refrozen liquid. Static inspection finds that the numerical
under-relaxation guard treats liquid-water, cold-content, and refrozen-liquid
zero crossings as discrete posture changes and therefore falls back to raw
Picard at the refreeze onset/removal seam.

The first contract-first candidate permitted only these continuous phase-axis
zero crossings inside an otherwise-authorized convex unpublished iterate. Its
focused five-parent run reduced cap failures 5 to 4, but the canonical day is
`REJECTED`: accepted/rejected counts and the accepted-width distribution were
unchanged, cap failures increased 124 to 188, and test-body runtime regressed
416.94 to 432.62 seconds. Closure remained exact. The candidate production,
test, and first v27 contract edits were removed; authority returned to v26
before selecting the distinct v27 classification below. The optimization
checkpoint then continued against the measured direct/composed refrozen-liquid
limiter without tolerance relaxation.

The selected `SC-SNOWENERGY-001@27` correction classifies only per-layer
`refrozen_liquid_m` as within-trial factorization history for adaptive
direct/composed error estimation. Exact accepted composed owner/restart bytes
still retain the tracer, and committed publication still independently
reconstructs physical refreeze and closes mass, liquid, and energy. The
canonical one-day fixture passes with 504 accepted / 227 rejected trials,
49 exact-floor supports, and `374.23 s` body time versus the retained
588/320, 139-floor, `416.94 s` baseline. Exact discrete/event rejections remain
zero and ledger/receipt limits are unchanged. The remaining 128 fixed-point
caps remain the next solver limiter rather than being masked by tolerance.

## Included scope and deliverables

Included: bounded vapor/latent custody; enthalpy phase projection; exact grid
and adaptive direct/composed controller; typed complete-owner comparison and
receipt chain; joint lanes; terminal event/liquid receiver/snow-free remainder;
solid-precipitation reappearance; restart/replay; runner just-in-time supports;
successor SnowEnergy, SnowFreeze, CoupledTime, and compiler-discovered contract
surfaces; production cutover and CoE retirement after qualification.

Deliverables are the implementation and test matrix, canonical contract
successors, current package evidence, representative adaptive seasonal runs,
real-consumer negative proof for the old path, and exact clean critical gates.

## Intended write set

- this package tree and `docs/work-packages/README.md`;
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`,
  `SC-SNOWFREEZE-001.md`, `SC-COUPLEDTIME-001.md`, and the affected
  integration prose in `SC-LANDSURFACEENERGY-001.md`,
  `SC-SURFACELIQUID-001.md`, and `SC-VEGETATION-001.md`, plus the registry and
  guards;
- affected `crates/openwepp-*` production, restart, runner, and test sources;
- affected integration tests and fixtures.

The terminal owned-file manifest records the compiler-discovered actual set.

## Contract-first sequence

1. Amend canonical successor contracts.
2. Add contract-derived phase/adaptive tests.
3. Record the pre-implementation contract gate.
4. Edit production code and direct consumers.
5. Execute focused, affected, authority, seasonal, and exact-workspace gates.
6. Complete reviews, finding disposition, verification, and atomic cutover.

## Phase plan and progress

- [x] (2026-08-26) Intake owner decision, prior HOLDs, repository governance,
  current contracts, and implementation seams.
- [x] (2026-08-26) Scaffold forward-only package and pre-edit evidence.
- [x] (2026-08-27) Checkpoint A: contract authority, bounded vapor, phase
  projection, and exact-60 focused matrix.
- [x] (2026-08-27) Checkpoint B: adaptive one-lane complete-owner
  direct/composed stepping, stable 60/1800 support, memo, and forced-oracle
  equivalence.
- [x] (2026-08-27) Checkpoint C: event, exact-once receiver, remainder,
  reappearance, rollback, and seven-posture/cross-midnight restart evidence.
- [x] (2026-08-28) Checkpoint D: joint multi-lane execution, exact terminal
  groups, lane/topology membership, and event receiver chronology.
- [x] (2026-08-29) Checkpoint E: restart and runner/day continuation. Restart
  evidence is green; the final exact-60 real runner passes one complete day
  with 48 parent supports, 588 accepted adaptive supports, 320 rejected trials,
  1,078 retained publication supports, 59 events, committed qualification
  snapshot, and real downstream consumption.
- [ ] Checkpoint F: exact-head gates and reconciliation are current; atomic
  cutover remains HOLD on contract-authority gaps for twilight forcing,
  frozen litter liquid, and terminal/resolved snow-phase complementarity.
  Seasonal/archive/per-step qualification is owner-paused and is not a closure
  prerequisite for the amended one-day objective.

### 2026-08-28 inverse-basis surface authorization finding

Season A exposed a valid same-store authorization whose OFE-ground debit was
exactly `f_t*W_0`, while the resource phase's required binary64 inverse
`F/f_t` was one ULP greater than `W_0` (`f_t=.62`, residual
`-1.734723475976807e-18 kg H2O m^-2 tile-ground`). This is neither missing
custody nor admissible candidate clamping. `SC-SURFACELIQUID-001` version 12
therefore requires the existing symmetric common authorization scale to prove
both OFE-basis and exact tile-basis debit sums before sealing authorization.
The resource phase remains exact and fail-closed. Focused `.62` inverse-basis,
unscaled-debit, inconsistent-supply, and exact-zero dry-store vectors are
current-scope gates before the seasonal rerun.

## Validation intent

Risk is `Critical`: production kernel chronology, state/restart layout, shared
owners, conservation, runner consumption, and default cutover may change.
Focused edit loops use the phase/adaptive tests, orchestrator check/test build,
rustfmt, and diff hygiene. Increment closure adds warnings-denied affected
Clippy, affected integration/contract/restart/runner suites, independent mass,
water, energy and receipt reconstruction, anti-evasion, source scans, doctests,
and exact real-consumer evidence. Terminal closure runs the canonical exact
clean full-workspace correctness and authority requirements selected by
`docs/standards/testing-and-gate-strategy.md`. No full-season 60-second oracle is
admitted.

Conservation/output acceptance requires the pre-edit operand-lineage table,
anti-alias fixtures, rejected formulas, independent reconstruction, real
closure/magnitude audit, and metadata/schema alignment. Self-consistency and
one-sided bounds are supporting evidence only.

## Reviews and delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to independent snow-thermodynamics/adaptive-numerics, ownership/hydrology/
restart, Rust correctness/performance, QA/anti-evasion reviewers, two terminal
verifiers, and `comparator_suite_runner` for heavy batch gates. Expected output
is compact findings or exact command metrics/log paths and package artifacts;
review source access is read-only and artifact write access is bounded to this
package. Session-level authorization remains governed by the active user/tool
policy and cannot be created by this package.

Every finding is accepted and fixed, rejected with authority, or assigned a
legitimate follow-up boundary. Closure requires four requested GO reviews and
dual PASS verification with gate-legitimacy and line-count checks.

## Exit criteria and HOLD legitimacy

Success is exactly the owner directive's success criteria: one adaptive Stage-3
operator; known fixture closure; no ice plus material unallocated energy;
practical stable-step behavior; joint lanes; exact-once terminal liquid;
reappearance; byte-equivalent restart; runner complete-day/season execution;
clean workspace/authority gates; independent GO; atomic production cutover.

An unmet current-scope criterion is not deferred. HOLD is legitimate only for
one of the scientific stop conditions enumerated in the owner directive and
requires a HOLD legitimacy audit proving the boundary and why every in-envelope
correction route cannot close. Effort, compiler failures, DTO design, test
failures, and performance tuning are implementation work, not HOLD boundaries.

Security impact: no secrets or external services. Authority-suite and retained
historical guards may be strengthened but never weakened.

## Surprises & Discoveries

- The existing tree already contains complete-owner candidate state, shared
  carrier reevaluation, Batch event grouping, terminal parcel/WB14 receiver,
  restart, and runner attachment. The controlling production defect remains
  concentrated in the old terminal transition and root/localization operator.
- Exact-60 focused execution and restart pass through seven interruption
  postures and cross-midnight continuation. The final production one-day run
  completes all 48 parents, 588 accepted supports, 320 rejected trials, and 59
  events, then passes committed publication, the downstream consumer, archive
  fold, and output transaction.
- Default-off real-fixture audit established that the repeated 64-iteration
  ceiling was a stable period-2 covered Stage-3/soil outer iteration. A
  support-scaled under-relaxed current-iterate solve now converges representative
  120- and 480-second supports in 27 and 75 iterations. A bounded `0.25`
  long-support contraction floor prevents vanishing progress at 1800 seconds;
  authentic candidate density remains unblended and acceptance remains
  bitwise-exact. Structural, phase, receipt, closure, rollback, and publication
  rules remain unchanged.
  The final amended one-day replacement run passes.
- The late 1,336/1,426 regression was caused by receipt digest keys being
  classified as physical order. Separating exact per-trial WB14 factorization
  lineage from cross-factorization physical state restored the earlier coarse
  controller distribution without relaxing any accepted-path receipt check.

## Decision Log

- Decision: reuse and replace the current complete-owner terminal seam rather
  than create a parallel snow runner.
  Rationale: this preserves the already-tested owner/receiver/restart chain and
  satisfies the prohibition on alternate temporal models.
  Date/Author: 2026-08-26 / Codex.
- Decision: accept checkpoints A-C from their focused amended-floor and restart
  evidence, retain checkpoint D pending its complete production matrix, and
  retain checkpoints E-F open until the unresolved-liquid publication defect
  and all terminal gates close. This historical decision was superseded after
  the one-day consumer passed; the remaining cutover HOLD is now the narrower
  external-authority boundary recorded in `artifacts/disposition.md`.
  Rationale: focused PASS evidence could not be promoted through the then-
  failing real one-day downstream consumer.
  Date/Author: 2026-08-27 / Codex.
- Decision: classify only the WB14 per-OFE digest-keyed child receipt map and
  child ordinal as per-trial factorization lineage during adaptive physical
  comparison.
  Rationale: direct `H` and composed `H/2 + H/2` have different exact child
  receipt identities/counts by construction; both paths retain and validate
  those exact receipts, while physical equivalence must compare their ending
  state, event/topology posture, and conservative ledgers.
  Date/Author: 2026-08-29 / Codex.

## Outcomes & Retrospective

The owner-amended exact-60 canonical one-day objective is complete: 588
accepted supports, 320 rejected trials, 76.36% of accepted supports above the
floor, 13.53% lower optimized model/test body time, and strict mass/energy and
receipt closure. Production persists no microstep diagnostics. Exact terminal
child custody and the amended contract/assurance identities are validated.

Atomic production cutover remains HOLD on external twilight, frozen-litter,
and terminal/resolved phase authority, plus the recorded broad external
failures/timeouts. Seasonal, archive, memory, and generic per-step work remains
owner-paused and is not used to dilute or defer the completed one-day result.
