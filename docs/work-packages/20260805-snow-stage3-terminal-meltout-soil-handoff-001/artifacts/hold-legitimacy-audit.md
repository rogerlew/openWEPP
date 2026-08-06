# HOLD Legitimacy Audit

Status: PASS / legitimate missing-authority boundary

Evidence class: Static contract and pinned-source audit on 2026-08-06.

## Boundary

The declared Phase-1 gate requires complete canonical authority before any
production edit. That authority is missing for the terminal numerical method,
post-meltout surface-energy solve, its energy recipients, and a persistent
parallel coupled state. This is a scientific/ownership authority boundary, not
an implementation-effort or validation-cost stop.

## Evidence

- `SC-SNOWENERGY-001#INV-SNOWENERGY-029/030` admits resolved-snow phase
  chronology but explicitly blocks positive terminal energy and residual-snow
  proxies.
- `SC-SNOWFREEZE-001#GAP-SNOWFREEZE-006` keeps residual snow and terminal
  energy non-promotable; `GAP-SNOWFREEZE-002` leaves snow-free wet-heat/Qwet
  completion open.
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-091` prohibits closure on a second mass
  state, so a persistent independently mutable shadow needs an explicit
  authority amendment.
- Libsnobal `_calc_layers.c`/`_adj_layers.c` perform threshold conversion to
  water, not error-controlled event localization; its no-snow branch does not
  provide the WEPP land-surface recipient.
- Pinned WEPP `tmpadj.for` produces the `surtmp` driver consumed by the frost
  top heat path. It does not advance an unfrozen-soil enthalpy column, pair
  event-local vapor mass with latent energy, account for precipitation heat,
  or carry surface-water/runoff enthalpy.
- The pinned baseline's `qwet` migration-water heat branch is inactive under
  the active `frzftp=0` posture and cannot be promoted as a live recipient.
- The current soil, evaporation, water-balance, and runoff contracts provide
  soil constitutive/water mass state, daily ET, infiltration, ponding, and
  residual runoff, but not the missing energy states.

## In-Envelope Routes Considered

1. **Apply libsnobal residual conversion.** Rejected: it clears snow state at a
   threshold without the required energy-conserving event or receiving-surface
   continuation.
2. **Assign terminal excess directly to soil.** Rejected by contract and
   physics: the flux was evaluated with snow properties that expire at
   meltout.
3. **Use pinned `tmpadj` as the complete receiving solver.** Rejected: its
   admitted scope is a legacy frost surface-temperature driver and it lacks
   the required energy recipients/ledgers.
4. **Continue a snow-only persistent shadow against CoE soil state.** Rejected:
   it cannot support restart or seasonal physical coherence and conflicts with
   the current single-state ownership posture.
5. **Implement event-local mass routing only.** Rejected as partial target
   implementation because the declared recipient and energy gates would
   remain open.

## Canonical Authority-Admission Routes

The repository's two bounded A0 admission routes were considered explicitly:

- **ADR-0024 source-intent authority has not succeeded for this target.** The
  pinned libsnobal source expresses threshold conversion and snow-state
  clearing, while pinned WEPP supplies partial frost-boundary and water-mass
  routines. No cited routine expresses the composite event-local algorithm,
  post-snow component equations, energy recipients, conservation identity, or
  coupled-shadow ownership. Promoting those partial, inactive, or ambiguous
  branches would manufacture source intent instead of extracting it. ADR-0024
  therefore supports the recorded provenance but cannot yet close this A0 gap.
- **ADR-0028 observed-data admission has not succeeded and is not currently
  applicable to this decision.** No prospectively defined, independently
  reviewed, conserving terminal/receiving-surface candidate exists to score.
  Nor is there a prospectively held-out, forcing-robust observation operator
  that can adjudicate event-local receiving-energy closure or parallel-state
  ownership. Existing Snowbird/Jennings/SNOTEL evidence can diagnose seasonal
  behavior, but cannot waive conservation, define missing energy operands, or
  grant software-state ownership. An ADR-0028 route would first require a
  conserving candidate, frozen observation roles and operators, cross-regime
  generalization criteria, and contract-first admission review.

These are scoped findings, not permanent exclusions. The roadmap assigns the
land-surface-energy authority increment as the next owner and the coupled-state
authority increment as the additional owner for persistence and seasonal
claims. Their next gates are the exact equation/ledger and ownership decisions
listed in `worker-handoff.md`.

## Why The Current Package Cannot Close The Boundary

Closing the gap requires selecting and independently reviewing new scientific
authority for a general snow-free land-surface energy balance, then separately
defining parallel coupled-state ownership across contracts. Neither equation
set nor ownership decision exists in the current canonical authority or pinned
baseline. Inventing either inside production code would be prohibited
surrogate physics. The package therefore correctly stops before contract-test
or Rust edits and hands off the two named authority-admission increments.
