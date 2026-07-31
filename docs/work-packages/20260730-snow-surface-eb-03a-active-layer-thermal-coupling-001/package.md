# SNOW-SURFACE-EB-03A Active-Layer Thermal Coupling

Status: `executed / complete / pass`

Date: `2026-07-30`

Campaign: `SNOW-SURFACE-EB`

Defect: `GAP-SNOWENERGY-007`

This defect-closure ExecPlan is a living document maintained under
`docs/codex_exec_plans.md` and `docs/defect_closure_execplans.md`.

## Purpose / Big Picture

Close the EB-03 cold-content failure by replacing the depositional-top-layer
surface control volume with the authoritative Marks/SNOBAL active thermal
layer, coupling that layer to the lower snowpack inside the hourly energy
balance, and using authority-defined stability substeps. The candidate remains
behind the existing default-off Stage 3 and surface-energy selectors.

## Objective

Establish one physically admissible thermal provider for the `B/L/S/LS`
consumer cells without a clamp, fitted limiter, new user coefficient, remote
observation, or change to the protected CoE melt path. The provider must:

- form an active thermal control volume over
  `min(total_snow_depth, 0.25 m)`;
- conserve mass, cold content, and thermal resistance when that control volume
  crosses depositional-layer boundaries;
- include snowpack conduction in the same substep energy balance as radiation
  and turbulent vapor exchange;
- use Marks/SNOBAL mass-dependent timestep subdivision; and
- complete the real S and LS consumers with physical temperatures and
  independently reconstructed mass/energy closure.

## Implementation Intent

Intent: science implementation and defect closure. This package performs no
empirical calibration or independent validation.

```text
science_implementation_status = IMPLEMENTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

The status is updated only after terminal evidence. Constants are fixed from
admitted authority and are not user-facing calibration coefficients.

## Correction Authority Envelope

- Defect: `GAP-SNOWENERGY-007`, observed as the EB-03 S and LS consumers
  reaching the Stage 3 `0 K` bound with material SWE remaining.
- Canonical contracts:
  `SC-SNOWENERGY-001` and `SC-SNOWFREEZE-001`.
- Primary process authority: Marks et al. (1998, 1999), Anderson (1976), and
  the CC0 libsnobal implementation pinned locally at
  `/home/workdir/pysnobal` commit
  `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`.
- Supporting stability authority: Lute et al. (2022), section 2.2.7, which
  documents the Marks progressively smaller-timestep response to shallow-pack
  instability. Its alternative fitted/clamped SnowClim correction is excluded.
- Allowed edit classes: contract amendment, contract-derived tests, typed
  meteorology/snow thermal primitives, Stage 3 runtime composition, internal
  diagnostics, deterministic package evidence, and directly affected
  construction tests.
- Protected boundaries: production defaults, public schemas, CoE melt/rain
  mass, density and phase algorithms, frost behavior, fixtures, observed data,
  and existing EB-03 longwave/view-factor authority.
- Conversion rule: when the seven-gate authority bar is met, implement the
  complete admitted mechanism in this package. Diagnostic uncertainty,
  implementation effort, or a partially passing consumer does not justify
  `HOLD`.

## Included Scope

- Reconcile depositional layers with an independent active/lower thermal
  partition.
- Bind and implement the `0.25 m` maximum active-layer depth and the
  Marks/SNOBAL `60/10/1 kg m^-2` timestep thresholds with
  `60/15/1 minute` execution levels.
- Bind and implement harmonic interlayer conductive transfer using layer
  thermal resistance and the same substep temperatures as the surface fluxes.
- Evaluate radiation, vapor/latent exchange, and conduction within each
  stability substep.
- Preserve exact-one vapor mass and latent-energy derivation.
- Conservatively distribute active/lower thermal state back to persistent
  layers.
- Produce diagnostic attribution for current event-layer, active-layer,
  coupled-conduction, and substepped states.
- Execute analytical, focused, protected default/B/L, and real S/LS consumer
  evidence.

## Excluded Scope

- No default activation, user-facing selector, or new user coefficient.
- No empirical tuning, site calibration, forcing rescaling, or observation
  leakage.
- No temperature clamp, cold-content tax, arbitrary limiter, or
  air-temperature state replacement.
- No replacement of CoE melt with energy-balance melt.
- No EB-04 factorial adjudication or EB-05 promotion.
- No new prognostic canopy-temperature model.
- No broad snow-density, albedo, phase, frost, or output-schema change.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-meteorology/src/surface_energy.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`
  for additive, environment-gated snow-trace thermal diagnostics
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs`
  for the existing elevation-derived atmospheric-pressure projection required
  by the admitted libsnobal effective-conductivity formulation
- directly affected `crates/openwepp-hillslope-orchestrator` construction tests
- `tests/integration/snow_surface_eb03_contract.rs`
- `tests/integration/snow_surface_eb03_runtime.rs`
- a focused EB-03A integration test if separation is clearer
- `Cargo.toml` only if a new integration-test target is added
- this package tree
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- assurance source-adoption/rendered-review artifacts only if canonical
  assurance drift tooling requires them

The terminal diff must distinguish inherited EB-03 changes from EB-03A edits.
Any production edit outside this set requires prospective amendment.

## Contract-First Sequence

1. Amend canonical contracts.
2. Author contract-derived tests.
3. Record the failing-before-production contract gate.
4. Edit production code.

Production code edits for this package are prohibited before steps 1–3 are
complete.

## Conservation / Output Acceptance

Before production edits, complete `artifacts/operand-lineage.csv`. Acceptance
requires:

- independent reconstruction of active-layer mass, depth, heat capacity, cold
  content, and interface thermal resistance;
- distinct fixtures rejecting event-layer-only heat capacity, direct cover as
  sky view, omitted conduction, wrong conduction sign, hourly-only thin-pack
  integration, vaporization heat, latent sign reversal, and double mass debit;
- per-substep active/lower energy closure;
- whole-pack energy closure in which internal conduction cancels exactly;
- vapor mass equals snow-ice export and latent heat uses the identical signed
  exchange; and
- a real S/LS magnitude audit with physical temperature chronology.

Producer self-consistency and “did not reach 0 K” are supporting evidence only.

## Phase Plan

### Phase 0 — Scaffold and authority intake

Freeze authority, write set, operand lineage, protected boundaries, and
validation selection.

### Phase 1 — Contract and tests

Amend both snow contracts, add contract-derived analytical and guard tests, and
record the pre-implementation contract gate.

### Phase 2 — Authoritative implementation

Implement active/lower thermal partitioning, coupled conductive energy,
mass-dependent substeps, conservative state projection, and diagnostics.

### Phase 3 — Real-consumer closure

Execute the B/L/S/LS cells, protected defaults, reconstruction, formatting,
lint, domain profiles, documentation checks, reviews, and final disposition.

## Validation Requirements

- `cargo fmt --all -- --check`
- focused EB-03/EB-03A integration tests
- `cargo nextest run -p openwepp-meteorology`
- `cargo nextest run --workspace --profile quick`
- `cargo nextest run --workspace --profile frost`
- `cargo nextest run --workspace --profile full`
- `cargo clippy --workspace --all-targets -- -D warnings`
- strict contract/profile and Binding Exposure checks for both touched
  contracts
- raw unit-conversion checks for touched production files
- scoped Markdown lint/reference checks
- independent analytical recurrence for the coupled active/lower solve
- real direct-production B/L/S/LS consumer execution
- exact-diff and line-count governance

Terminal impact may conservatively add requirements but may not silently remove
one.

## Exit Criteria

1. Canonical authority binds the active thermal control volume, interface
   conduction, timestep subdivision, signs, guards, and projection.
2. Depositional layers smaller than the active-layer depth do not independently
   define the radiating/turbulent heat capacity.
3. Radiation, latent exchange, and `G_0` use a common substep state.
4. Active/lower and whole-pack energy close independently.
5. Sublimation mass/latent identity and liquid non-aliasing remain exact.
6. Real S and LS complete without a clamp and retain material, physical state.
7. Default/absent/empty/disabled, B, and L protected behavior passes the
   prospectively declared tests.
8. All current-scope validation requirements pass.
9. Dual independent reviews, finding disposition, dual verification,
   line-count governance, assurance impact, and terminal reconciliation are
   complete.

Any unmet current-scope criterion forces `HOLD`; it cannot be relabeled as
EB-04 scope.

## HOLD Legitimacy

`HOLD` is allowed only if evidence proves an out-of-envelope mechanism,
missing/contradictory authority after the admitted routes are exhausted,
invalid upstream input correctly rejected by a typed guard, or unavailable
required evidence. A hold must include `artifacts/hold-legitimacy-audit.md`.

## Security Impact Gate

No secrets, network services, authentication changes, unsafe Rust, external
mutation, or subprocess interpolation are authorized.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent code/science reviewers and two terminal
verification agents. Review scope is the exact EB-03A diff, authority,
conservation, consumer path, and gate legitimacy; expected outputs are the four
named review/verification artifacts. Review access is read-only. Verification
agents may write only their assigned package artifacts.

## Progress

- [x] (2026-07-30) Package authorized by operator request and scaffolded.
- [x] (2026-07-30) Authority and operand intake complete.
- [x] (2026-07-30) Contracts and contract-derived tests complete.
- [x] (2026-07-30) Production implementation complete.
- [x] (2026-07-30) Real-consumer and terminal validation executed; snow-domain
  gates pass and the full quick profile exposes a CQR evidence-fixture failure.
- [x] (2026-07-30) Review, verification, and `HOLD` disposition complete.

## Surprises & Discoveries

- EB-03's thermal provider uses the first persistent depositional layer. In the
  failing chronology that layer contains about `0.34 kg m^-2`, whereas
  Marks/SNOBAL defines an active thermal layer independently of snowfall-event
  boundaries.
- The former inherited EB-03 interlayer pass occurred after applying the
  complete hourly surface energy and reported zero whole-pack conduction by
  construction. EB-03A replaces it with in-substep `G_0` in the coupled
  active/lower balance.

## Decision Log

- Decision: treat persistent depositional layers and thermal control volumes as
  separate partitions.
  Rationale: snowfall chronology is not authority for thermal penetration
  depth; Marks/SNOBAL supplies an explicit active-layer definition.
  Date/Author: 2026-07-30 / Codex.
- Decision: port the admitted mass-dependent timestep hierarchy rather than a
  fitted shallow-snow clamp.
  Rationale: it is the documented Marks stability mechanism and introduces no
  user coefficient.
  Date/Author: 2026-07-30 / Codex.

## Outcomes & Retrospective

The Marks/SNOBAL active-layer correction closes the science defect
`GAP-SNOWENERGY-007`.
Persistent depositional layers are projected into a shared upper `0.25 m`
thermal state and a distinct lower state, coupled by exact libsnobal
`KTS+efcon` conductivity inside each mass-selected substep. All six protected
and candidate consumer cells complete with physical snow temperatures. Peak
requested `G_0` reconstructs from the published same-substep operands within
`3.2e-13 W m^-2`; requested/applied/rejected identity and internal conduction
cancellation are exact. SNOW-SURFACE-EB-03B subsequently lifted the terminal
validation hold without changing snow physics: complete quick 2109/2109, frost
324/324, and Critical full 2158/2158 profiles pass. EB-04 is admitted for
scaffolding.
