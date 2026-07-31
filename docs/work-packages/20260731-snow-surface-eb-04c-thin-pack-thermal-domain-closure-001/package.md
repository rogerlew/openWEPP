# SNOW-SURFACE-EB-04C Thin-Pack Thermal-Domain Closure

Status: `complete / pass`

Date: `2026-07-31`

Campaign: `SNOW-SURFACE-EB`

Plan class: `Defect-Closure ExecPlan`

This living ExecPlan follows `docs/codex_exec_plans.md` and
`docs/defect_closure_execplans.md`.

## Purpose / Big Picture

Close the 22 EB-04A thermal failures without tuning sublimation or weakening a
physical guard. Seventeen failures reach impossible temperatures and five
reach a valid-Kelvin numerical underflow only after the represented snow mass
has fallen far below the Marks/SNOBAL minimum resolved layer mass.

## Objective

Amend canonical authority and the real Stage 3 production consumer to reproduce
libsnobal's two distinct `1 kg m^-2` branches while CoE continues to own snow
existence and mass. Total pack mass at or below the boundary suspends exchange
before thermal partition. In a resolved pack, a lower volume strictly below
the boundary collapses to a one-volume solve and continues exchange; equality
remains two-volume.

## Correction Authority Envelope

Defects:

- `DEFECT-EB04B-THERMAL-001`: 17 positive-cold-content, vanishing-mass states
  imply `T <= 0 K` before carrier evaluation.
- `DEFECT-EB04B-THERMAL-002`: five valid-Kelvin states underflow the SNOBAL
  ice-saturation dependency at the same sub-resolution mass scale.

Authority:

- `SC-SNOWENERGY-001` and its physical-domain/conservation invariants;
- Marks/SNOBAL active-layer and timestep authority;
- libsnobal `_calc_layers.c`, `_below_thold.c`, `_divide_tstep.c`, and
  `_adj_layers.c` at commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`;
- EB-04A typed replay and EB-04B complete chronology.

Allowed edits:

- amend `SC-SNOWENERGY-001` and its registry lifecycle note;
- add contract-derived boundary and real-consumer tests;
- add the exact thermal-resolution branch and diagnostics in the Stage 3
  production path;
- update the runner trace consumer for explicit branch evidence;
- add package-local replay/analysis and lifecycle evidence;
- update the snow-energy roadmap and work-package catalogs.

Protected boundaries:

- no absolute-zero or snow-temperature clamp;
- no air-temperature substitution, cold-content tax, epsilon vapor pressure,
  fitted limiter, or new user coefficient;
- no change to the CoE snow-existence, melt, density, phase, or liquid-routing
  selections;
- no conversion of unresolved snow to liquid and no deletion of mass or cold
  content;
- no sublimation coefficient calibration or EB-04 observation scoring;
- no weakening of typed invalid-temperature, pressure, or closure guards;
- no EB-04D layer-geometry correction.

## Implementation Intent

Intent: `contract-first defect closure and production implementation`.

Calibration intent: `NOT_APPLICABLE`; this is a fixed model-domain boundary,
not a user parameter or empirical calibration.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`;
- `docs/specifications/science-contracts/index.md`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`;
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`;
- `tests/integration/snow_surface_eb03_contract.rs`;
- `tests/integration/snow_surface_eb03_runtime.rs`;
- this package tree;
- `docs/ROADMAP.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/work-packages/README.md`.

Any additional production surface requires a plan amendment before editing.

## Phase Plan

1. Freeze authority, exact failures, correction semantics, operands, and gates.
2. Amend `SC-SNOWENERGY-001` before production edits.
3. Add contract-derived tests for exact threshold sides, state preservation,
   forbidden aliases, and resume behavior.
4. Implement the real Stage 3 consumer branch and explicit trace diagnostics.
5. Replay all 22 exact thermal failures with the corrected binary and prove
   each passes its formerly rejected processing day without a new
   thermal-domain error.
6. Run selected focused, quick, frost, and Critical full correctness gates;
   complete dual review, finding disposition, dual verification, and closure.

## Acceptance Criteria

1. Exactly total `m_s <= 1 kg m^-2` suspends Stage 3 thermal/exchange before
   partition; total `m_s > 1 kg m^-2` remains resolved.
2. In a resolved pack, `0 < m_l < 1 kg m^-2` collapses the thermal partition
   to one whole-pack volume and continues exchange; `m_l = 1 kg m^-2` remains
   a two-volume solve.
3. The suspended branch applies zero shortwave, longwave, latent, vapor,
   conduction, sublimation, or surface-energy mutation and does not fabricate a
   temperature or conductivity.
4. CoE SWE, layer mass, liquid, refrozen mass, and cold content remain present;
   no unresolved snow is converted, deleted, or routed as melt.
5. A later increase above the threshold resumes from the conservatively
   retained layer state and existing projection rules.
6. All 22 exact EB-04A thermal targets pass their formerly rejected processing
   day; none
   returns below-absolute-zero, saturation-underflow, closure, or missing-layer
   errors before that boundary.
7. B/L protected behavior, exact selector defaults, and every existing typed
   invalid-state guard remain intact.
8. The real runner trace publishes both branch identities and their masses;
   no wrapper, shadow, or test-only path carries the closure claim.
9. Every applicable selected gate, dual review, finding disposition, and dual
   terminal verification passes.

Any unmet criterion forces `HOLD`.

## Validation Requirements

- contract schema/profile and canonical binding checks;
- focused contract-derived and production-path tests;
- exact 22-target diagnostic replay with source/input/binary hashes;
- independent state-before/state-after mass, cold-content, and zero-flux
  reconstruction plus wrong-branch/epsilon/clamp anti-alias assertions;
- `cargo fmt --all -- --check` and applicable Clippy;
- `cargo nextest run --workspace --profile quick`;
- `cargo nextest run --workspace --profile frost`;
- Critical exact-head full-workspace correctness because production kernel
  authority and a shared snow runtime branch change;
- scoped Markdown, source-guard, security, line-count, and diff checks.

## Conservation / Publication Acceptance

The unresolved branch is conservation-sensitive. Before production edits,
record mass, liquid, refrozen mass, cold content, and all energy/vapor operands
with units and owners. Tests must distinguish state preservation from the
rejected alternatives of forced liquid conversion, layer deletion, temperature
clamping, epsilon pressure, or applying one more flux. Independent real-run
evidence must reconstruct unchanged state and zero exchange at the boundary.

## Security Impact Gate

No network, secret, authentication, unsafe Rust, external write, or public
schema change is intended. All subprocesses use explicit argument arrays and
package-local or `target/` outputs.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent science/code reviewers, two terminal
verification agents, and a `comparator_suite_runner` for required Critical
full-workspace/batch validation. Review and verification access is read-only;
the suite runner writes only ordinary test outputs under `target/`. Expected
outputs are compact verdicts/metrics plus package-recorded artifacts.

Subagent requirement: REQUIRED for the Critical full-workspace correctness
profile. The parent must not run that heavy profile while the authorized suite
runner is available.

## Progress

- [x] (2026-07-31) User authorized scaffold and end-to-end execution.
- [x] (2026-07-31) EB-04B failure ownership and local libsnobal authority read.
- [x] (2026-07-31) Correction envelope and protected boundaries frozen.
- [x] (2026-07-31) Contract amendment and contract-derived tests complete.
- [x] (2026-07-31) Production correction and real-consumer diagnostics complete.
- [x] (2026-07-31) Exact 22-target replay and selected validation complete.
- [x] (2026-07-31) Dual review, finding disposition, dual verification, and closure complete.

## Surprises & Discoveries

- The five vapor-pressure underflows are not an independent parameter problem:
  all 22 thermal failures occur after mass falls far below libsnobal's minimum
  resolved snow-layer mass.
- Primary authority review caught that libsnobal does not suspend a resolved
  pack merely because its lower volume is sub-resolution. `_calc_layers.c`
  instead collapses `m_l < 1 kg m^-2` into one thermal volume and continues;
  only total `m_s <= 1 kg m^-2` takes the no-layer branch.
- Libsnobal converts sub-threshold snow to water, but that phase/mass decision
  belongs to CoE in openWEPP. EB-04C adopts only the authoritative thermal-domain
  boundary, retaining CoE mass and cold content rather than copying libsnobal's
  phase conversion.

## Decision Log

- Decision: use the exact fixed `1 kg m^-2` libsnobal small-timestep threshold
  as a Stage 3 thermal-resolution boundary, not a user coefficient.
  Rationale: this is the nearest authoritative model-domain rule and covers all
  22 failures without fitting or altering the physical guard.
  Date/Author: 2026-07-31 / Codex.
- Decision: preserve the source's distinct comparison sides and ordering.
  Total `m_s <= 1 kg m^-2` branches before partition; lower `m_l < 1` collapses
  to one volume after partition; lower equality remains two-volume.
  Rationale: this is the exact `_calc_layers.c` control flow and closes the
  thin lower-volume failures without suppressing valid whole-pack exchange.
  Date/Author: 2026-07-31 / Codex after independent review.
- Decision: preserve CoE mass and proportional cold content below the boundary.
  Rationale: importing libsnobal's residual-snow-to-water conversion would
  violate the campaign's explicit CoE ownership and create an unclosed energy
  transition.
  Date/Author: 2026-07-31 / Codex.

## Outcomes & Retrospective

EB-04C closes both EB-04B thermal defect families without calibration or guard
relaxation. The production Stage 3 consumer now reproduces the two exact
libsnobal `1 kg m^-2` branches while CoE retains mass and phase ownership.

All 22 captured trajectories pass their formerly rejected processing day: six
exercise total-pack suspension and 16 exercise lower-volume collapse, with no
forbidden thermal error. Twenty trajectories complete; two later reach new
occurrences of the separately admitted EB-04D layer-geometry mechanism.

Focused tests pass `23/23`; quick passes `2119/2119`; frost passes `325/325`;
and the canonical Critical full rerun passes `2168/2168`. An initial full run's
single 720-second assurance-test timeout is retained in the gate record; that
test then passed alone and in the clean canonical rerun without changing any
timeout or test setting. Dual review and dual terminal verification return
`PASS / PASS`.
