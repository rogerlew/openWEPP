# Stage 3 Terminal Meltout And Land-Surface Handoff

Status: `scaffolded / reviewed / Phase-1 authority preflight required`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.

## Objective

Replace the current `m_s <= 1 kg m^-2` suspended Stage 3 shadow boundary with
a contract-authorized terminal snow solve that localizes complete solid-snow
exhaustion within declared tolerances, routes phase-generated and terminally
released liquid in the same substep, changes surface regime, and recomputes
the remainder of the interval under a typed snow-free land-surface boundary
selected from actual cover, liquid, and frost state. A coherent cross-day or
seasonal claim additionally requires a coupled shadow of every land-surface,
soil thermal, soil-water, and frost state affected after divergence from CoE.
CoE remains the sole authoritative melt owner; this package cannot perform
cutover.

Implementation intent: science implementation and diagnostic qualification,
not calibration, empirical fitting, independent validation, default
activation, or publication.

## Rationale

The current shadow advances only within one day, reinitializes from the
post-CoE pack on the next day, and stops thermal/phase evaluation when total
represented snow mass reaches `1 kg m^-2`. The threshold is a libsnobal
resolved-layer boundary, not a physical phase boundary. Libsnobal converts the
remnant to water, but openWEPP intentionally did not import that disposition
while CoE owned melt. The resulting shadow cannot produce a coherent seasonal
trajectory or prove terminal meltout closure.

The proposed strategy, subject to the Phase-1 authority gate, is:

1. define an implicit or error-controlled terminal integrator, including
   admitted state/error tolerances, event bracketing, convergence failure, and
   flux reevaluation for both warming and cooling as snow mass approaches
   zero;
2. retain the admitted `60/15/1 minute` resolved-pack cadence only as the outer
   cadence, not as proof that the shallow terminal solve is stable;
3. collapse shallow snow to one enthalpy-bearing control volume rather than
   suspend physics;
4. localize the earliest combined melt/sublimation solid-exhaustion event
   within contract tolerance without applying snow fluxes after the event;
5. consume cold content and latent fusion energy before declaring meltout;
6. debit ice and route generated liquid through same-substep refreeze,
   retention, terminal retained-store release, surface-liquid supply,
   infiltration, ponding/overflow, evaporation, and residual runoff exactly
   once;
7. select a typed snow-free land-surface regime at the event time from actual
   residue/vegetation cover, frost, ponding, and infiltration state; and
8. recompute radiation, turbulent exchange, vapor transfer, evaporation,
   precipitation heat, and ground/soil energy for the remaining interval.

The old snow-computed `Q_unallocated_after_exhaustion` may not be assigned
wholesale to soil. Snow albedo, surface temperature, saturation state,
roughness, and conduction cease to be valid at meltout. The snow ledger must
end at the localized event with no time or energy silently carried past the
surface transition; a distinct post-event surface ledger owns the recomputed
remainder.

## Included Scope

- Amend canonical snow-energy, snow/frost, soil, evaporation, water-balance,
  and infiltration/runoff authority before code where the ownership inventory
  proves an amendment is required.
- Reconcile libsnobal terminal conversion, Marks/SNOBAL shallow-pack cadence,
  existing WEPP frost/soil state, and actual land-surface energy authority.
- Define the implicit/error-controlled terminal algorithm, tolerances,
  bracketing, nonlinear failure behavior, and flux reevaluation requirements.
- Define typed pre-event snow enthalpy, phase, liquid, vapor, and
  tolerance-localized exhaustion-event operands with units and chronology.
- Define the snow-free land-surface selector and authoritative recipients; if
  complete authority is absent, record a Phase-1 hard hold before production
  edits rather than inventing a flux sink.
- If seasonal persistence is admitted, persist a coupled shadow of all snow,
  surface-cover, surface-liquid, soil thermal/water, and frost state that can
  diverge from CoE; otherwise restrict evidence to event-local diagnostics.
- Implement tolerance-localized residual-snow phase/vapor conversion in
  shadow only after the authority gate passes.
- Implement same-substep shadow refreeze, retention, terminal store release,
  infiltration-first routing, and independently reconstructable ledgers.
- Recompute remaining-interval surface fluxes after meltout when canonical
  authority admits the receiving surface.
- Execute analytical, convergence, cadence, event, terminal,
  noninterference, and real-consumer evidence. Restart and seasonal Snowbird
  evidence are admitted only if the coupled shadow-state requirement passes.
- Extract the shadow solver from the `3000+` line reconciliation module before
  closure.

## Excluded Scope

- Stage 3 authoritative melt, CoE retirement, dual-owner mutation, or default
  activation.
- Directly dumping snow-computed excess energy into soil.
- Treating snow-column drainage as hillslope runoff or bypassing the mandatory
  infiltration-before-runoff handoff.
- Fitted residual-mass thresholds, temperature clamps, minimum heat-capacity
  epsilons, artificial cold-content taxes, or other surrogate physics.
- Reusing CoE-coupled soil, frost, surface-water, or evaporation state after a
  shadow divergence while claiming coherent restart or seasonal behavior.
- Calibration against SNOTEL, Jennings, Snowbird, or the frozen campaign
  outcomes.
- Public schema, release, observation, site-window, or acceptance-threshold
  changes.
- Claiming cutover readiness from shadow-only evidence.

## Dependencies And Authority

- `SC-SNOWENERGY-001`, especially `INV-SNOWENERGY-023/026/029/030/031`.
- `SC-SNOWFREEZE-001` for snow/frost and liquid/frozen-water chronology.
- `SC-WATBAL-001` for unfrozen/frozen soil-water storage identities.
- `SC-SOIL-001` for soil state and constitutive ownership.
- `SC-RUNOFFPART-001` for infiltration-first liquid partition and residual
  runoff.
- `SC-EVAP-001` for cover-, residue-, water-, and soil-limited evaporation.
- libsnobal commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`:
  `_calc_layers.c`, `_adj_layers.c`, `_divide_tstep.c`, `_below_thold.c`,
  `_mass_bal.c`, `_runoff.c`, and `snobal.h`.
- Pinned WEPP baseline commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
  for existing soil/frost state and receiving-boundary provenance, including
  `hr_tmp.for`, `tmpadj.for`, `frostn.for`, `frwatc.for`, `winter.for`,
  `watbal_hourly.for`, `grna.for`, `evap.for`, and `evappm.for`.
- Predecessor package
  `20260805-snow-stage3-complete-carrier-shadow-melt-001` and its Snowbird
  seasonal evaluation.

## Intended Write Set

- `docs/work-packages/20260805-snow-stage3-terminal-meltout-soil-handoff-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` only if
  the reviewed soil recipient changes that contract's owned state/ledger.
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- a new bounded Stage 3 shadow/terminal-solve module under the same hydrology
  subsystem;
- the direct runner state/handoff modules required to persist diagnostic
  shadow state and emit internal trace evidence;
- focused snow-energy, frost/water handoff, runner, and contract tests;
- ignored evidence under
  `target/snow_stage3_terminal_meltout_soil_handoff/`.

The exact source/test paths must be narrowed in `artifacts/owned-file-manifest.md`
before production edits. Scope expansion requires prospective package
amendment and review.

## Phase Plan

### Phase 1 — Authority And Operand Freeze

Inventory snow, liquid, vapor, frost, soil, runoff, evaporation, runner, and
surface-regime ownership. Author an operand-lineage table. Define the
receiving-regime selector and decide whether canonical authority supports the
complete post-meltout land-surface solve and all coupled persistent states;
distinguish direct authority, justified inference, and missing authority.
This is a go/no-go pre-implementation gate. If authority remains incomplete,
close this package in a truthful `HOLD` or prospectively split an
authority-admission successor before any production edit.

### Phase 2 — Contract Tests And Pre-Implementation Gate

Add analytical contract vectors for cold snow, isothermal snow, joint
sublimation/melt, tolerance-localized combined exhaustion, simultaneous
precipitation/deposition, within-step surface transition, negative
post-transition energy, terminal retained-water release,
infiltration/ponding/overflow/runoff, integrator convergence, restart, and
snow reappearance. Run binding, unit, profile, and authority gates before
production code.

### Phase 3 — Coupled Persistent Shadow State

If admitted by Phase 1, extract the shadow solver and introduce typed
persistent cross-day state for snow plus every affected surface-cover,
surface-liquid, soil thermal/water, and frost operand. Prove uninterrupted
versus restart equivalence and preserve bitwise authoritative CoE state and
outputs. If only event-local authority is admitted, do not claim persistence
or seasonal coherence.

### Phase 4 — Terminal Solve And Receiving Surface

Implement the contract-defined implicit or error-controlled shallow-pack
integrator. Localize the earliest combined phase/vapor solid-exhaustion event
within tolerance, flush retained snow liquid exactly once, close the snow
ledger, select the reviewed land-surface type, and recompute the remaining
interval. Do not reuse snow fluxes or reserve full-substep sublimation. Route
surface liquid through infiltration before ponding/overflow and residual
runoff.

### Phase 5 — Real Consumer And Seasonal Evaluation

Execute synthetic events and reconstruct all ledgers independently. Execute
restart equivalence and the frozen Snowbird primary windows only when Phase 3
proves the coupled shadow state needed for a coherent trajectory; otherwise
limit the disposition to event-local evidence. Evaluate previously frozen
chronology directions without tuning. Shadow evidence cannot authorize
cutover.

### Phase 6 — Review, Verification, And Disposition

Complete dual independent reviews, finding disposition, dual terminal
verification, selected direct gates, exact-diff reconciliation, line-count
governance, security impact, and truthful final disposition.

## Conservation And Publication Acceptance

Before production edits, `artifacts/operand-lineage.md` must identify every
energy, ice, vapor, liquid, frozen-soil, surface-water, infiltration,
ponding/overflow, evaporation, snow drainage, hillslope runoff, and soil
recipient operand; units; temporal and area basis; source authority; and
authoritative-versus-shadow status. Tests must separate plausible aliases,
including snow-computed terminal excess versus recomputed land-surface energy,
generated melt versus incoming rain, terminal retained-store release versus
store level/change, snow drainage versus hillslope runoff, and frost depth
versus frozen-water energy/storage.

Required identities include independently reconstructed snow energy, solid
mass, and liquid disposition through the event plus a distinct post-event
surface/soil energy identity. Self-consistency and one-sided bounds are
supporting evidence only.

## Exit Criteria

1. Canonical contracts define the residual-snow enthalpy state,
   implicit/error-controlled integrator, state/error tolerances, event
   bracketing, convergence failure, tolerance-localized combined phase/vapor
   event, receiving-regime selector, post-event recipients, sign conventions,
   guards, and linked ledgers without surrogate physics.
2. Every current-scope contract/gate obligation has direct evidence; no unmet
   required gate is relabeled as future work after execution begins.
3. Any seasonal claim is backed by a coupled persistent shadow of every
   affected surface, soil thermal/water, and frost state; its restarts are
   bitwise/deterministically equivalent to uninterrupted shadow execution.
4. Ice, liquid, vapor, refreeze, terminal store release, infiltration,
   soil/surface storage, ponding/overflow, evaporation, residual runoff,
   cold-content, fusion, and post-event energy ledgers reconstruct
   independently within contract tolerances.
5. No snow-computed energy or vapor demand is applied after the localized
   exhaustion event; remaining time is recomputed under the selected surface.
6. No authoritative CoE snow state, runoff, defaults, public outputs, or
   downstream behavior changes.
7. The real direct runner consumes the admitted event-local or coupled
   persistent shadow path; no skeleton, daily reinitialization, or reuse of
   CoE-coupled receiving state carries a seasonal claim.
8. Focused, quick, frost, critical full-workspace, and real Snowbird gates
   selected under `testing-and-gate-strategy.md` pass or the package closes in
   an explicit legitimate HOLD.
9. Dual reviews, finding dispositions, and dual verifications are complete
   with no undispositioned findings.
10. No nonexempt `3000+` line Rust file remains; the shadow solver is extracted
    before closure.

## Security Impact

No network, credential, parser-trust, unsafe-code, or external-execution change
is intended. Record the final diff assessment in `artifacts/security-impact.md`.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation to one
read-only domain-science reviewer, one read-only Rust correctness reviewer, one
read-only Rust QA reviewer, two read-only terminal verifiers, and one
`comparator_suite_runner` for required heavy/full-workspace gates. Expected
outputs are compact findings or metrics with artifact/log paths. Only the
primary executor may edit tracked files.

## Progress

- [x] User directed scaffold and independent strategy review.
- [x] Proposed enthalpy/event-driven meltout and recomputed land-surface
  handoff frozen before implementation.
- [x] Independent scaffold review and six blocking findings dispositioned.
- [ ] Package execution separately authorized and started.
- [ ] Contract authority, implementation, gates, reviews, and disposition.

## Decision Log

- Decision: treat `1 kg m^-2` as a solver-regime boundary, not a physical phase
  boundary. Rationale: areal heat capacity becomes numerically stiff, while
  libsnobal's threshold terminates the resolved layer rather than establishing
  new thermodynamics. Date/Author: 2026-08-05 / Codex.
- Decision: do not assign the snow-computed excess directly to soil. Rationale:
  snow-surface fluxes become invalid at meltout; remaining-interval fluxes must
  be recomputed after a typed surface transition. Date/Author: 2026-08-05 /
  Codex.
- Decision: accept all six independent-review findings. Rationale: terminal
  warming and cooling require a contract-defined convergent integrator; the
  receiving surface and liquid route depend on actual cover, frost, and water
  state; and seasonal coherence requires coupled receiving-state persistence.
  Date/Author: 2026-08-05 / Codex.

## Outcomes

The scaffold review is dispositioned. Execution must begin with the Phase-1
authority go/no-go gate and may truthfully close `HOLD` before production edits
if complete receiving-surface and coupled-state authority cannot be admitted.
