# Snow Terminal Enthalpy Event Numerics

Status: `complete`

Date: `2026-08-07`

Package ID: `20260807-snow-terminal-enthalpy-event-numerics-001`

Plan class: `Critical kernel evaluation implementation`

## Objective

Admit and implement an error-controlled, evaluation-only shallow-snow enthalpy
solve that localizes the earliest complete solid-exhaustion event. Close the
snow-side mass, phase, vapor, liquid, time, and energy ledgers at that event
without assigning remaining time, liquid, or energy to a receiving land
surface and without changing authoritative CoE behavior.

## Implementation Intent

Intent is `science implementation for internal event-local numerics`, not
calibration, independent validation, production activation, publication, or
cutover. The libsnobal/Marks shallow-pack structure and physical conservation
authorize the numerical event solve. Missing post-snow land-surface authority
limits the endpoint claim but does not block isolated snow-domain mechanics.

## Included Scope

- Amend `SC-SNOWENERGY-001` and `SC-SNOWFREEZE-001` before production edits.
- Define a one-volume shallow-pack enthalpy state, validity domain, adaptive
  error control, event bracketing/localization, convergence failure, and exact
  combined melt/sublimation solid-exhaustion chronology.
- Add typed evaluator inputs/results/errors and an absent-by-default request.
- Integrate only with `persistent_accumulation_shadow_v1`, preserving its
  existing resolved-pack behavior away from the terminal domain.
- Close snow energy, ice, vapor, liquid, and evaluated-time ledgers at the
  localized event; emit all terminal operands through an internal schema.
- Prove the real rejecting internal consumer independently reconstructs the
  event and rejects producer residuals, aliases, post-event snow flux, and
  unsupported receiving-surface claims.
- Preserve exact disabled production state, routing, defaults, public outputs,
  WAT/HBP/PASS bytes, and CoE ownership.

## Excluded Scope And Claim Limits

- No land-surface, soil, frost, infiltration, runoff, evaporation, ponding, or
  receiving-energy recipient and no remainder-of-interval recomputation.
- No assignment of terminal unallocated energy or released liquid after the
  snow-domain event; both remain explicit censored handoff quantities.
- No physical seasonal efficacy, calibration, promotion, default, publication,
  ownership, or cutover claim.
- No fitted threshold, minimum heat-capacity epsilon, temperature clamp,
  artificial cold-content tax, or other surrogate physics.

## Dependencies And Authority

- `SC-SNOWENERGY-001`, `SC-SNOWFREEZE-001`, ADR-0042, and the persistent
  accumulation shadow package.
- CC0 libsnobal at `/home/workdir/pysnobal`, commit
  `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`, especially `_calc_layers.c`,
  `_adj_layers.c`, `_divide_tstep.c`, `_below_thold.c`, `_e_bal.c`,
  `_mass_bal.c`, and `snobal.h`.
- Marks et al. shallow-snow stability precedent already anchored by the
  canonical snow-energy contract.
- Prior terminal-meltout authority inventory, used only to preserve the
  receiving-surface boundary rather than to reintroduce its broad hold.

## Intended Write Set

- This package tree, `docs/work-packages/README.md`, `docs/ROADMAP.md`, and the
  snow campaign roadmap.
- `SC-SNOWENERGY-001`, `SC-SNOWFREEZE-001`, their index, assurance DRAFT source
  lock/receipt if required, and focused contract tests.
- Stage 3 evaluator/state modules in `openwepp-hillslope-orchestrator`.
- Direct runner builder/internal trace/consumer modules needed for the typed
  request and event evidence.
- Focused Rust and integration tests.
- Ignored execution logs under `target/snow_terminal_enthalpy_event_numerics/`.

Any expansion requires prospective amendment and review.

## Contract-First Phase Plan

1. Freeze required reading, operand lineage, numeric protocol, write set, and
   validation selection; commit the scaffold.
2. Amend contracts, add contract-derived tests, and record a passing
   pre-implementation contract gate.
3. Implement typed enthalpy state, adaptive solve, event localization, and
   terminal snow-domain ledgers.
4. Connect the default-off persistent evaluator and real internal consumer
   without adding a receiving surface or altering production.
5. Run analytical, convergence-order, boundary, negative, focused, quick,
   frost, and critical full-workspace gates.
6. Reconcile the exact diff, line counts, assurance, reviews, verification,
   prompt archive, catalog/roadmap, disposition, and local commit.

## Conservation And Output Acceptance

Before production edits, record every enthalpy, cold-content, flux-energy,
ice, vapor, melt/liquid, event-time, and censored handoff operand with units,
time/area basis, source authority, and diagnostic status. Tests must use
unequal operands and reject full-step melt, full-step sublimation, snow-flux
continuation after exhaustion, endpoint-only aliases, producer residuals, and
terminal energy/liquid interpreted as a receiving-surface disposition.
Acceptance requires independent reconstruction from the actual internal row,
two-sided convergence/error checks, and mass/energy/time closure.

## Validation And Exit Criteria

- Contracts define the enthalpy state, algorithm, tolerances, event ordering,
  typed failures, and claim boundary before code edits.
- Exact below/at/above terminal-domain and exhaustion-event vectors pass,
  including cooling/no-event and joint melt/sublimation cases.
- Refinement demonstrates the declared local error/event-time order against an
  analytical or independently high-accuracy oracle; iteration/nonconvergence
  failures are typed and fail atomic state commit.
- Independent consumer reconstruction closes event-local energy, ice, vapor,
  liquid, and time and rejects every named alias.
- Disabled/absent selection preserves production state, arithmetic, schema,
  outputs, defaults, and ownership exactly.
- Focused, quick, frost, critical full-workspace, Clippy, formatting, docs,
  assurance, security/data, line-count, dual-review, and dual-verification
  requirements pass with direct current evidence.

## Calibration Readiness

This is a fixed-physics numerical implementation with no parameter fitting or
observation-bearing result. `calibration-readiness-matrix.md` must use the
ADR-0042 enums and disposition all ten readiness obligations.

## Security And Data Impact

Local source and retained fixtures only. No secrets, external messages,
deployment, observation edits, or public release. Internal traces remain
explicit opt-in.

## Line-Count Governance

Record touched Rust line counts before and after. Files at 2,000+ lines are
`WARN`; nonexempt files at 3,000+ lines block closure.

## Review And Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one read-only numerical-science reviewer, one read-only
primary Rust reviewer, one read-only QA/consumer reviewer, one comparator suite
runner for heavy frost/full-workspace commands with writes limited to ignored
target logs, and two independent read-only terminal verifiers. Expected outputs
are compact findings, exact commands/counts, convergence/closure checks, and
verification verdicts.

## Progress

- [x] (2026-08-07) User authorized scaffold and end-to-end execution.
- [x] (2026-08-07) Froze and committed scaffold `c979f2fbd`.
- [x] (2026-08-07) Completed contract-first authority and 36-test
  pre-implementation gate after independent sign/endpoint corrections.
- [x] (2026-08-08) Implemented and validated event-local terminal numerics,
  typed failures, schema-v8 evidence, and the rejecting internal consumer.
- [x] (2026-08-08) Completed numerical, Rust, and QA reviews; exact-head full
  workspace, focused, Clippy, formatting, assurance, and governance gates; and
  terminal verification/disposition.
