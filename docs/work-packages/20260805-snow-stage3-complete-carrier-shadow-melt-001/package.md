# Stage 3 Complete Carrier And Shadow Melt

Status: `scaffolded / authorized / prediction frozen`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.

## Purpose

Resume the CoE-to-Stage-3 cutover in the scientifically forced order: complete
and validate the Stage 3 surface-energy carrier first, then compute the
`INV-SNOWENERGY-029` cold-content-first melt chronology in shadow state while
CoE remains the sole authoritative melt owner. Do not cut over while the
carrier is incomplete, seasonal net energy is implausible, available ice is
exhausted with positive terminal energy, or any linked-ledger gate remains
open.

## User Authority And Prospective Freeze

Direct user instruction on 2026-08-05 authorizes resuming CoE cutover work with
carrier-first shadow sequencing. The following predictions are registered
before implementation or result inspection:

- mid-winter melt decreases;
- peak SWE increases;
- peak SWE date moves later;
- spring melt rate increases; and
- seasonal Stage 3 net energy approaches physical closure near zero rather
  than retaining approximately `+216.87 MJ m^-2` at Snowbird.

These are prospective directional tests, not tuning targets. Thresholds,
acceptance rules, or site windows may not be changed after result inspection
without a new package and explicit result-aware disclosure.

## Frozen Gate Observation

For Snowbird, median per primary accumulation window in retained Stage 3
evidence:

| Quantity | Median |
| --- | ---: |
| Absorbed shortwave | `223.25 MJ m^-2` |
| Surface energy applied | `6.39 MJ m^-2` (`2.9%`) |
| Unused positive energy | `216.87 MJ m^-2` (`97.1%`) |
| Latent-fusion equivalent of unused energy | `0.649 m` SWE |
| Median snowfall | `0.769 m` SWE |
| Median pack loss | `0.530 m` SWE |

The unused positive energy is equivalent to about `84%` of median seasonal
snowfall. It is not a post-cutover melt forecast. It demonstrates that the
current shortwave-dominated carrier is not physically complete. Conversion of
today's carrier would exhaust `m_ice_available` and leave positive
`Q_unallocated_after_exhaustion`, which is already a hard cutover hold under
`INV-SNOWENERGY-029`.

## Scientific Freeze

- `GAP-SNOWENERGY-011` remains open: complete sensible and
  precipitation-advected heat are absent from the carrier.
- Explicit longwave is required for the shadow carrier and cutover evidence;
  its current default-off compatibility selector does not authorize a
  shortwave-only melt carrier.
- Complete net radiation and the admitted sensible, latent, conduction, and
  precipitation-advection operands must be summed exactly once with the
  positive-toward-snow convention.
- Cold content is satisfied before latent-fusion conversion.
- `m_melt=min(Q_excess/L_f,m_ice_available)` is shadow-only in this package.
- `Q_unallocated_after_exhaustion` must be zero for every cutover-eligible
  substep. A positive value is not discarded or proxied and blocks cutover.
- CoE remains the sole authoritative mass-mutating melt owner throughout this
  package. Shadow Stage 3 melt cannot affect pack state, routing, public
  outputs, defaults, or downstream consumers.
- The principal mechanistic hypothesis is thermodynamic gating, not flux
  magnitude: cold content should suppress accumulation-season melt and defer
  melt toward an isothermal spring pack.
- Retained 21M evidence that `C_open` is net negative while the unvalidated
  2008 `C_canopy` branch is the sole net-positive CoE contributor is treated as
  evidence of formulation misspecification risk, not a calibration target.

## Included Scope

1. Freeze exact retained 21L/21M/21N energy and timing evidence and reconstruct
   every Snowbird operand independently.
2. Complete the typed Stage 3 shadow carrier with explicit net longwave,
   sensible heat, latent heat, conduction, and precipitation-advected heat.
3. Require complete-carrier shadow evaluation regardless of compatibility
   longwave selector while leaving authoritative compatibility behavior and
   defaults unchanged.
4. Add typed shadow cold-content, available-ice, melt, terminal-energy, and
   linked mass/energy ledgers derived from `INV-SNOWENERGY-029/030`.
5. Persist internal diagnostic evidence sufficient to compare Stage 3 shadow
   chronology with authoritative CoE on the same substeps and accumulation
   windows.
6. Prove exact noninterference: CoE remains the only mass-mutating melt owner
   and all established runtime/public outputs remain byte-identical.
7. Evaluate the frozen predictions and seasonal carrier plausibility without
   tuning to outcomes.

## Excluded Scope

- Stage 3 authoritative melt conversion or CoE retirement.
- Any dual-owner state mutation.
- Default, selector, runfile, schema, public-output, calibration, coefficient,
  site-window, observation, or release change.
- Treating shadow results as cutover authority when terminal energy, thin-pack,
  same-substep liquid, or real-consumer gates remain open.

## Intended Write Set

To be narrowed after source-seam inventory and before production edits:

- `docs/work-packages/20260805-snow-stage3-complete-carrier-shadow-melt-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- applicable snow-energy implementation modules under `crates/openwepp-runner/src/hillslope/`
- applicable typed meteorology/energy modules under `crates/openwepp-meteorology/src/`
- focused integration tests under `tests/integration/`
- ignored evidence under `target/snow_stage3_complete_carrier_shadow_melt/`

No production edit may begin until the exact file list replaces the two
module-level placeholders above.

## Phase Plan

### Phase 1 — Freeze And Seam Inventory

Bind exact inputs, independently reconstruct the Snowbird gate number, map all
carrier producers/consumers and unit/sign/temporal boundaries, narrow the
write set, and establish byte-identical authoritative baselines.

### Phase 2 — Complete Shadow Carrier

Implement typed complete flux operands and exact-one integration. Longwave is
mandatory in shadow evidence. Preserve compatibility selectors and CoE state.

### Phase 3 — Shadow Melt Chronology

Compute cold-content-first bounded Stage 3 melt and linked ledgers without
mutating pack or routing state. Fail closed on incomplete operands, nonfinite
values, closure violations, or positive terminal unallocated energy.

### Phase 4 — Prospective Evaluation

Run real accumulation-window evidence, evaluate the frozen directions and
seasonal net-energy plausibility, and disposition rather than tune every
failure. A later atomic cutover package is authorized only if all holds close.

## Validation And Gates

- Contract invariants and exact typed operand reconstruction.
- Analytical sign/unit/time-integration vectors for every carrier component.
- Independent energy and mass closure reconstruction.
- Shadow/noninterference tests proving authoritative CoE state and outputs are
  unchanged.
- Real Snowbird and campaign-window consumer evidence.
- Focused, quick, frost, and immediate full-workspace correctness regression.
- Dual domain-science/Rust review, dual terminal verification, and heavy runner.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation to one
read-only domain-science reviewer, one read-only Rust correctness reviewer, one
read-only Rust QA reviewer, two read-only terminal verifiers, and one
`comparator_suite_runner` for heavy campaign/full-workspace gates. Subagents
may not edit tracked files or reinterpret frozen acceptance rules.

## Progress

- [x] (2026-08-05) User authorized carrier-first shadow resumption.
- [x] (2026-08-05) Froze directional predictions and the Snowbird retained
  energy gate before implementation.
- [x] (2026-08-05) Scaffolded this package before source edits.
- [ ] Reconstruct the gate and narrow the exact write set.
- [ ] Implement and validate the complete carrier.
- [ ] Implement non-mutating shadow melt and linked ledgers.
- [ ] Evaluate prospective directions and disposition all gates.

## Decision Log

- Decision: longwave is mandatory for shadow cutover evidence even though it
  remains a compatibility default-off selector. Rationale: a shortwave-only
  carrier cannot satisfy seasonal physical plausibility or the terminal-energy
  cutover gate. Date/Author: 2026-08-05 / Codex.
- Decision: separate shadow computation from authoritative deployment.
  Rationale: this preserves exact-one melt ownership while allowing the
  complete carrier and cold-content mechanism to be tested prospectively.
  Date/Author: 2026-08-05 / Codex.

## Outcomes

Pending execution.
