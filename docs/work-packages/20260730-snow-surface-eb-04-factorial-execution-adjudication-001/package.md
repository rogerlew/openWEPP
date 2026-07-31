# SNOW-SURFACE-EB-04 Factorial Execution And Adjudication

Status: `executed / hold / nonpromotion`

Date: `2026-07-30`

Campaign: `SNOW-SURFACE-EB`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Purpose / Big Picture

Execute the one preregistered `B/L/S/LS` round admitted by EB-03A/03B and
decide whether additive sub-canopy longwave, energy-consistent sublimation, or
their combination earns an EB-05 promotion package. Preserve the frozen
physics, forcing, fixtures, selectors, observation roles, and stop-loss.

## Implementation Intent

Intent: `independent-validation` plus deterministic factorial adjudication.
This package does not calibrate coefficients, change production physics, or
activate a default.

## Objective

- Execute all four cells through the real direct-production hillslope
  consumer across the five SNOTEL open controls, five bound Harvard/Marcell
  canopy/open lanes, and two diagnostic Sleepers frost lanes.
- Prove trace identity, independently reconstruct the applicable mass ledger,
  and audit the producer-carried energy/latent residuals before interpreting
  performance.
- Score the frozen `INV-SNOWFREEZE-050` observation rubric and the
  prospectively frozen EB-04 timing operators.
- Publish marginal, combined, and interaction effects, protected signatures,
  contrary evidence, uncertainty, and claim limits.
- Produce readable plots with a Markdown sidecar for every figure.
- Apply the one-round stop-loss and issue a promotion/nonpromotion decision.

## Frozen Prospective Decision Protocol

The complete pre-result protocol is
[`artifacts/prospective-decision-protocol.md`](artifacts/prospective-decision-protocol.md).
No result-bearing run may begin until that artifact is complete.

## Included Scope

- Package-local experiment runner, deterministic analysis, tabular artifacts,
  SVG figures, and Markdown sidecars.
- Additive publication of already-computed Stage 3 component totals and
  latent/mass closure operands to the opt-in research snow trace. This
  diagnostic-only surface supports mass reconstruction, residual audits, and
  human-readable component time series; it does not alter process math.
- Existing direct-production diagnostic selectors only:
  `layered_thermal_liquid_v1`,
  `dilley_unsworth_subcanopy_v1`, and `neutral_bulk_stage3_v1`.
- Existing observation files and immutable fixture inputs.
- Existing canonical `SC-SNOWFREEZE-001` rubric and EB-03/03A closure
  tolerances.
- Roadmap, work-package catalog, and assurance-impact handoff updates.

## Excluded Scope

- No process-physics Rust or contract amendment.
- No fixture, observation, forcing, coefficient, threshold, selector, default,
  parser, runfile schema, user surface, or public output-schema change.
- No empirical fitting, rescaling, observation leakage, or second experiment
  round.
- No decisive warm-maritime conifer transfer claim because paired observations
  are unavailable.
- No EB-05 default activation or promotion implementation.

## Dependencies

- EB-01 preregistration and observation-role freeze.
- EB-02 authoritative longwave contract.
- EB-03 shared thermal/energy composition.
- EB-03A active/lower thermal correction.
- EB-03B terminal validation hold lift.
- `SC-SNOWFREEZE-001` invariants 050, 085, and 086.
- `SC-SNOWENERGY-001` v3.

## Intended Write Set

- This package tree.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`
  only for additive opt-in snow-trace diagnostic fields that already exist in
  `DirectSnowStage3Diagnostics`.
- directly affected runner construction tests.
- `tests/integration/snow_surface_eb04_package.rs` and `Cargo.toml` only if a
  package-contract integration target is necessary.
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- assurance source-adoption status only when impact analysis proves it is
  required.

Process-physics Rust, canonical contracts, fixtures, and observations are
read-only. The additive diagnostic trace edit above is authorized before
implementation. Any other Rust need forces prospective package amendment
before the edit.

## Conservation / Publication Acceptance

The existing EB-03/03A diagnostic producer is read by a package-local
independent consumer. Acceptance requires:

- exact shared non-target environment and fixture identity for `B/L/S/LS`;
- mass closure at `<= 1e-9 m` water equivalent per daily coupling step;
- Stage 3 energy closure at `<= 1e-6 J m^-2`;
- latent/mass identity at `<= 1e-6 J m^-2`;
- full independent reconstruction of the published surface total from its
  published component operands;
- exact rejection of double sublimation and liquid aliasing;
- real downstream WAT and trace consumption; and
- no interpretation of a cell failing a physical gate.

## Phase Plan

1. Scaffold, intake authority, freeze every result operator and decision
   threshold, and add contract-derived tests for the diagnostic trace fields.
2. Publish the already-computed diagnostic operands, then implement the
   deterministic package-local factorial runner and figure builder.
3. Build the exact runner, execute the fixed matrix once, and adjudicate.
4. Run focused/domain/campaign validation selected from the terminal diff.
5. Complete dual independent reviews, finding disposition, dual terminal
   verification, roadmap/catalog updates, and final disposition.

## Validation Requirements

- Package tool self-check and deterministic regeneration/diff check.
- Real direct-production execution inventory for every lane/cell.
- Existing focused EB-03/03A contract/runtime suites.
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile quick`
- `cargo nextest run --workspace --profile frost`
- `cargo nextest run --workspace --profile full`
- Scoped Markdown lint/reference checks.
- Figure/sidecar one-to-one inventory and SVG parse check.
- Exact-diff, line-count, placeholder/stub, assurance-impact, and security
  reconciliation.

Exact current evidence from EB-03B may be reused only when source identity and
affected surface are demonstrably unchanged. Terminal impact may add but may
not silently remove a requirement.

## Exit Criteria

1. All operators and thresholds were frozen before the first result-bearing
   run.
2. Every prescribed lane has exactly four comparable cells or a named
   fail-closed execution result.
3. Physical ledgers pass before performance interpretation.
4. Every response, effect, interaction, protected lane, failure, uncertainty,
   and contrary result is retained.
5. Every figure is plot-only and has a complete Markdown sidecar.
6. The promotion/nonpromotion decision follows the frozen rule and stop-loss.
7. No observation, forcing, coefficient, fixture, production physics, or
   default was changed.
8. All current-scope validation requirements pass.
9. Dual review, finding disposition, dual verification, exact-diff, line-count,
   and final handoff are complete.

Any unmet current-scope criterion forces `HOLD`; it cannot be deferred to
EB-05.

## Security Impact Gate

No secrets, network access, authentication changes, unsafe Rust, fixture
mutation, or shell interpolation are authorized. All experiment subprocess
arguments are explicit.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent science/code reviewers and two terminal
verification agents. Expected outputs are
`artifacts/review_agent_a.md`, `review_agent_b.md`,
`verification_agent_a.md`, and `verification_agent_b.md`. Review access is
read-only. Verification write access is limited to the assigned package
artifact.

## Progress

- [x] (2026-07-30) User authorized scaffolding and full execution.
- [x] (2026-07-30) Package scaffolded and prospective protocol frozen.
- [x] (2026-07-30) Factorial runner implemented and checked before result
  execution.
- [x] (2026-07-30) Fixed 48-cell matrix executed once without retries.
- [x] (2026-07-30) Seven failure-aware figures and sidecars generated.
- [x] (2026-07-30) Validation, review, verification, and HOLD disposition
  complete.

## Surprises & Discoveries

- The short HJ Andrews consumer used by EB-03A did not expose the
  population-wide runtime problem. Across the fixed lanes, every LS cell, ten
  S cells, and two L cells failed. Twenty-two failures route through the
  effective-conductivity error wrapper; `harvard_open/S` and
  `marcell_open/LS` instead fail prior-layer thickness reconciliation.
- That wrapper maps every underlying meteorology error to one generic
  hydrology-domain error and prints `layer.density_kg_m3` as the rejected
  conductivity-path value. Values such as `522` are therefore densities, not
  computed conductivities; the exact lower-level cause is not recoverable from
  the retained error.

## Decision Log

- Decision: use the existing `INV-SNOWFREEZE-050` ordinal rubric as the primary
  observation score rather than inventing a new fitted objective.
  Rationale: it was accepted before this campaign and already separates
  forcing-robust and forcing-limited signatures.
  Date/Author: 2026-07-30 / Codex.
- Decision: require a strict protected-lane non-regression rule for LS.
  Rationale: a headline aggregate gain cannot compensate for a new
  forcing-robust failure or failed physical ledger.
  Date/Author: 2026-07-30 / Codex.

## Outcomes & Retrospective

The single fixed round closes as nonpromotion before empirical adjudication.
B completed all 12 lanes, L completed 10, S completed 2, and LS completed none.
The 24 typed failures occurred from day 13 through day 12,517, demonstrating
that short consumer tests were insufficient for population-wide runtime
admissibility. Retained partial rows satisfied the mass, producer-carried
surface/cold-content closure, producer-carried latent/mass residual, and
thermal checks before the rejected step, and the candidate operands were
nonzero. Because the wrapper loses the
underlying meteorology error, EB-04 does not label the exact constitutive cause
or apply a post-result numeric change. The retained trace also lacks shortwave,
so the preregistered full independent surface-component reconstruction cannot
be completed without an unauthorized rerun. It also publishes only the
producer-carried latent/mass residual, not the independent per-step operands
needed to reconstruct that identity. No LS observation score, protected
comparison, interaction, or promotion claim is admissible. The physical-gate
and one-round stop-losses apply; EB-04 holds and EB-05 is bounded to
negative-result assurance closeout.
