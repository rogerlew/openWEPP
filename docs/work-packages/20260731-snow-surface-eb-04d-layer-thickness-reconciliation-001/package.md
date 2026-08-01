# SNOW-SURFACE-EB-04D Layer-Thickness Reconciliation Closure

Status: `complete / pass`

Date: `2026-07-31`

Campaign: `SNOW-SURFACE-EB`

Plan class: `Defect-Closure ExecPlan`

This living ExecPlan follows `docs/codex_exec_plans.md` and
`docs/defect_closure_execplans.md`.

## Purpose / Big Picture

Close the two EB-04A/04B layer-geometry failures without enlarging a tolerance
or deleting represented snow. Both failures occur because the multilayer
density handoff filters a layer using a `1e-9 m` SWE threshold while comparing
physical layer depth against an independently equal-valued `1e-9 m` depth
tolerance.

## Objective

Amend canonical authority and the real density/runtime consumer so represented
layers use a dimensionally coherent mass lifecycle. Preserve every layer whose
ice mass exceeds the existing `1e-9 kg m^-2` density-model zero-mass boundary,
which is `1e-12 m` SWE under `rho_w = 1000 kg m^-3`; retain the independent
`1e-9 m` aggregate closure tolerance only for residual checking.

## Correction Authority Envelope

Defect:

- `DEFECT-EB04B-GEOMETRY-001`: `harvard_open/S` and `marcell_open/LS`
  reject valid persistent columns because one represented fragment is filtered
  by SWE while its physical thickness remains in the expected aggregate.

Authority:

- `SC-SNOWENERGY-001`, especially `INV-SNOWENERGY-021`;
- physical conservation and dimensional consistency;
- EB-04A typed snapshots and EB-04B independent reconstruction;
- the existing density-model zero-mass constant in `kg m^-2` and named
  `rho_w` conversion.

Allowed edits:

- amend `SC-SNOWENERGY-001` and its registry note;
- add contract-derived exact-side, conservation, and real-consumer tests;
- correct the multilayer density handoff and typed replay reconstruction;
- route both downstream layer-retention consumers through the same lifecycle
  predicate and named SWE-to-area-mass conversion;
- serialize complete layer vectors in the opt-in snow research trace so the
  replay can reconstruct conservation independently;
- register the typed layer-vector unit seams and their scalar exceptions;
- add package-local exact two-case replay/evidence;
- update campaign roadmaps and package catalog.

Protected boundaries:

- do not increase or suppress the `1e-9 m` mass/depth closure tolerance;
- do not filter physical depth by an SWE threshold or vice versa;
- do not delete positive represented ice, liquid, refrozen mass, cold content,
  or settle state;
- do not weaken the typed aggregate-mismatch guard outside the authorized
  lifecycle boundary;
- do not change EB-04C thermal-domain branches, selectors, coefficients,
  observation scoring, or user schemas.

## Implementation Intent

Intent: `contract-first defect closure and production implementation`.

Calibration intent: `NOT_APPLICABLE`; the package corrects unit-governed state
reconciliation and introduces no fitted or user coefficient.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`;
- `docs/specifications/science-contracts/index.md`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`;
- `crates/openwepp-unit-boundary/src/lib.rs`;
- `crates/openwepp-sim-contract/src/units_mod/boundary_catalog.rs`;
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`;
- `tests/integration/snow_surface_eb03_contract.rs`;
- `tests/integration/snow_surface_eb03_runtime.rs`;
- this package tree;
- `docs/ROADMAP.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/work-packages/README.md`.

Any additional production surface requires a plan amendment before editing.

## Phase Plan

1. Freeze the exact two failures, units, operands, authority, and protected
   boundaries.
2. Amend `SC-SNOWENERGY-001` before production edits.
3. Add contract-derived tests for exact boundary sides, dimensional conversion,
   state preservation, typed rejection outside closure, and real consumption.
4. Record the pre-implementation contract gate.
5. Implement the dimensionally coherent layer lifecycle in the production
   density handoff and typed replay.
6. Replay both exact failure trajectories and prove they pass their formerly
   rejected processing day without a new conservation or thermal error.
7. Run focused, quick, frost, and Critical full correctness gates; complete
   dual review, finding disposition, dual verification, and closure.

## Acceptance Criteria

1. Layer retention is selected in mass units: positive represented layer mass
   greater than `1e-9 kg m^-2`, equivalent to `1e-12 m` SWE.
2. `1e-9 m` SWE is no longer used as a layer-deletion boundary.
3. The independent `1e-9 m` SWE and physical-depth aggregate residual checks
   remain unchanged and typed.
4. Exact below/equality/above vectors distinguish the mass-lifecycle boundary
   from the aggregate residual tolerance.
5. Retained mass, physical depth, liquid, refrozen mass, cold content, density,
   and settle state survive handoff and close independently.
6. A materially inconsistent layer aggregate still returns the existing typed
   `SnowLayerAggregateMismatch` error.
7. Both exact EB-04 geometry targets pass their formerly rejected processing
   day; EB-04C's 22 thermal targets and branch behavior remain protected.
8. The real production density consumer reads the corrected lifecycle; no
   wrapper, shadow, or test-only path carries the closure claim.
9. Every selected gate, dual review, finding disposition, and dual terminal
   verification passes. Any unmet criterion forces `HOLD`.

## Validation Requirements

- contract schema/profile and canonical binding checks;
- focused contract-derived and production-path tests;
- exact two-target replay with source/input/binary hashes;
- independent mass/depth/state reconstruction and wrong-unit/tolerance/deletion
  anti-alias assertions;
- EB-04C protected regression coverage;
- authority anti-evasion gates if required-case bindings change;
- `cargo fmt --all -- --check` and strict applicable Clippy;
- `cargo nextest run --workspace --profile quick`;
- `cargo nextest run --workspace --profile frost`;
- Critical exact-head full-workspace correctness because a production kernel
  state-reconciliation boundary changes;
- scoped Markdown, security, line-count, and exact-diff checks.

## Conservation / Publication Acceptance

Before production edits, record layer mass (`kg m^-2` and SWE `m`), physical
depth (`m`), density (`kg m^-3`), liquid/refrozen SWE (`m`), cold content
(`J m^-2`), and settle state with owners and conversions. Fixtures must reject
the wrong alternatives of applying the depth tolerance as a mass-deletion
threshold, increasing the residual tolerance, or dropping a positive fragment.
Independent reconstruction must close both captured states from serialized
layers rather than restating the producer's filtered sum.

## Security Impact Gate

No network, secret, authentication, unsafe Rust, dependency, external write, or
public schema change is intended. Subprocesses use explicit argument arrays and
write transient outputs only under `target/`; accepted reports are package-local.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent science/code reviewers, two terminal
verification agents, and a `comparator_suite_runner` for required Critical
full-workspace validation. Review and verification are read-only; the suite
runner writes only ordinary outputs under `target/`. Expected outputs are
compact verdicts/metrics plus package-recorded artifacts.

Subagent requirement: REQUIRED for the Critical full-workspace correctness
profile. The parent must not run that heavy profile while the authorized suite
runner is available.

## Progress

- [x] (2026-07-31) User authorized scaffold and end-to-end execution.
- [x] (2026-07-31) EB-04B geometry mechanism and EB-04C handoff read.
- [x] (2026-07-31) Correction envelope and protected boundaries frozen.
- [x] (2026-07-31) Contract amendment and contract-derived tests complete.
- [x] (2026-07-31) Production correction and exact two-case replay complete.
- [x] (2026-07-31) Selected validation, dual review, dual verification, and closure complete.

## Surprises & Discoveries

- The failing fragments are not zero mass: approximately
  `5.26e-10 m` SWE equals `5.26e-7 kg m^-2`, more than 500 times the existing
  density-model zero-mass boundary.
- The equal numeric values `1e-9` conceal different dimensions and roles:
  represented-mass lifecycle versus aggregate residual acceptance.
- Primary review found that downstream Stage 3 retention duplicated the
  equivalent threshold and that aggregate-only trace fields could not support
  independent reconstruction. The downstream predicate edit exposed the
  additional-surface requirement; this amendment records that review-driven
  expansion before terminal disposition and the subsequent trace/unit work.

## Decision Log

- Decision: keep the independent closure tolerance unchanged and make layer
  lifecycle selection in mass units using the existing named density-model
  zero-mass boundary.
  Rationale: this is dimensionally coherent, conserves represented state, and
  closes the proven mechanism without tolerance inflation.
  Date/Author: 2026-07-31 / Codex.
- Decision: bind density and Stage 3 retention to one executable predicate,
  use the named unit-boundary conversion, and expose complete layer vectors
  only in the opt-in research trace.
  Rationale: this prevents threshold drift and lets package tooling recompute
  mass/depth conservation without trusting producer aggregate fields.
  Date/Author: 2026-07-31 / Codex after primary-review findings.

## Outcomes & Retrospective

EB-04D closes both geometry failures without tolerance inflation, state
deletion, or a new coefficient. The canonical lifecycle retains density layers
whose represented mass is strictly greater than `1e-9 kg m^-2` after named
SWE conversion, while the independent `1e-9 m` aggregate residual guards
remain unchanged. Both 16,437-day trajectories complete with independently
reconstructed mass/depth closure. Focused, quick, frost, and Critical full
gates pass, as do dual review and dual terminal verification. EB-04E may now
qualify the corrected population without empirical scoring.
