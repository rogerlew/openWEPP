# SNOW-SURFACE-EB-03 Shared Thermal and Energy Composition

Status: `executed / HOLD`

Date: `2026-07-30`

Campaign: `SNOW-SURFACE-EB`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Purpose / Big Picture

Make the snow campaign's four experimental cells mechanically real without
changing the production default. After this package, internal opt-in selectors
can independently enable canonical sub-canopy longwave and energy-consistent
snow sublimation while both mechanisms consume one Stage 3 snow-surface
temperature/cold-content state. A human can verify the result through focused
contract tests and a real direct-production consumer trace that distinguishes
`B`, `L`, `S`, and `LS`.

## Objective

Select the existing `layered_thermal_liquid_v1` top layer as the one coherent
hourly snow-surface temperature and cold-content provider; bind
`T_c = T_a`, polar-night failure behavior, and `R_a,min`; implement
`SC-SNOWENERGY-001` longwave in the shared Stage 3 carrier; separate
sublimation from `SnowMeltModel`; and debit vapor mass and latent energy from
the same hourly turbulent exchange exactly once.

## Implementation Intent

Intent: production implementation plus deterministic campaign-readiness
evidence. This is not empirical calibration or independent validation.

```text
science_implementation_status = NOT_IMPLEMENTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

`NOT_IMPLEMENTED` applies to the package objective: a mechanically viable
shared B/L/S/LS thermal carrier was not established. Canonical longwave and
sublimation components were implemented only as a default-off,
diagnostic/reproduction seam. The package introduces no fitted parameter.
Fixed constants retain their canonical literature authority. Demonstration
forcing is `ASSUMED_FOR_EXECUTION`.

## Included Scope

- Amend `SC-SNOWENERGY-001` and `SC-SNOWFREEZE-001` before production edits.
- Bind Stage 3 top-layer temperature/cold content as the sole provider.
- Bind `T_c = T_a` as the named homogeneous-stand approximation.
- Bind `R_a,min = 1e-9 MJ m^-2 d^-1`.
- Fail closed with typed unavailable cloud forcing when `R_a <= R_a,min`; do
  not reuse legacy cloud state.
- Expose independent internal opt-in longwave and sublimation selectors;
  absent/empty selectors remain disabled.
- Implement canonical Dilley-Unsworth atmospheric longwave and effective-cover
  sky-view translation in typed meteorology helpers.
- Compose net shortwave, optional net longwave, and optional latent heat in the
  existing Stage 3 hourly carrier.
- Derive sublimation vapor mass and latent heat from one exchange; prohibit
  simultaneous use of the legacy Stage A/B melt-enum variants.
- Remove sublimated ice from the layer and aggregate snow state without routing
  it as liquid or melt.
- Preserve hourly operands sufficient for independent mass, energy,
  longwave, and latent/mass reconstruction.
- Prove the direct-production runner consumes the path and can express
  `B/L/S/LS`.
- Produce human-readable diagnostic figures with Markdown sidecars.

## Excluded Scope

- No default activation or user-facing runfile/WEPPpy control.
- No empirical calibration, site fitting, forcing rescaling, or fixture edits.
- No prognostic canopy-temperature balance.
- No new user coefficient, sky-view input, or remote-data requirement.
- No replacement of the CoE melt/rain mass path with energy-balance melt.
- No density, phase, frost, public output schema, HBP, or watershed change.
- No EB-04 factorial adjudication or EB-05 promotion decision.
- No claim that the retained Stage A/B candidates were promoted.

## Dependencies

- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- EB-01, EB-01A, and EB-02 retained evidence.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-meteorology/src/surface_energy.rs`
- `crates/openwepp-meteorology/src/error.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`
- `Cargo.toml` integration-test registration.
- focused `tests/integration/` contract/runtime tests and affected fixture-free
  struct-construction tests.
- this package tree, `docs/ROADMAP.md`,
  `docs/planning/snow-surface-energy-balance-roadmap.md`, and
  `docs/work-packages/README.md`.
- typed assurance V2 source-adoption outputs for the already-declared
  `SC-SNOWFREEZE-001` dependency (generated identity, snow report descriptor,
  affected review locks, and transaction receipt), plus deterministic
  full-catalog review-draft rendering under `usersum/assurance/review-drafts/`.

The exact terminal diff must be reconciled against this set. Widening requires
a prospective package amendment before the new production edit.

## Conservation / Output Acceptance

Before production edits, author `artifacts/operand-lineage.csv` with units,
sign, time basis, area basis, authority, producer, and consumer for every mass
and energy operand. Acceptance requires:

- separate reconstruction of atmospheric/sub-canopy/net longwave;
- `latent_heat * dt = vapor_mass_exchange * latent_heat_of_sublimation`;
- vapor mass loss equals the layer/aggregate ice-state reduction and never
  appears in routed liquid or melt;
- thermal storage change closes from applied surface energy, refreeze energy,
  conduction, and cold-content export with explicitly stated signs;
- anti-tautology cases whose expected values differ from direct-cover sky view,
  air-temperature snow emission, vaporization heat, wrong latent sign, and
  double-debit candidates.

Producer self-consistency and one-sided bounds are supporting evidence only.

## Phase Plan

### Phase 0 — Scaffold and authority intake

Freeze scope, reading, operand lineage, protected boundaries, and exact gate
selection.

### Phase 1 — Contract and contract-derived tests

Amend canonical contracts, author failing-before-production contract tests, and
record the pre-implementation contract gate.

### Phase 2 — Typed implementation and real consumer

Implement typed meteorology equations, runtime forcing lineage, provider and
selectors, Stage 3 composition, state mutation, diagnostics, and the runner
consumer.

### Phase 3 — Evidence and closure

Run focused, quick, frost, contract/unit, formatting, lint, and documentation
requirements selected by terminal impact. Generate figures/sidecars, perform
dual review and dual verification, disposition findings, reconcile the exact
diff, and close truthfully.

## Validation Requirements

- `cargo fmt --all -- --check`
- focused Nextest for the new EB-03 integration tests
- `cargo nextest run -p openwepp-meteorology`
- `cargo nextest run --workspace --profile quick`
- `cargo nextest run --workspace --profile frost`
- `cargo clippy --workspace --all-targets -- -D warnings`
- contract unit-compliance and strict Binding Exposure checks for both touched
  contracts
- raw unit-conversion guard for touched production files
- scoped Markdown lint and validation
- deterministic figure/evidence regeneration and diff hygiene
- direct-production consumer-path and rollback proof

Exact terminal impact may conservatively escalate these requirements but may
not silently narrow them.

## Exit Criteria

1. The canonical contracts bind the provider, selectors, cadence, threshold,
   signs, guards, state mutation, and closure identities.
2. The `B/L/S/LS` cells differ only by independent longwave/sublimation
   selectors while sharing melt, density, phase, liquid, forcing, and thermal
   provider selections.
3. Default/empty selector behavior is bit-for-bit unchanged on focused
   protected tests.
4. Longwave and sublimation are consumed by the real Stage 3 carrier; no
   producer-only or shadow evidence carries the claim.
5. Vapor mass, latent energy, longwave operands, and thermal storage close
   independently within contract-bound tolerances.
6. Sublimation is absent from routed melt/liquid and cannot be double-debited
   by legacy Stage A/B variants.
7. Polar night and missing provider states fail closed with typed errors.
8. All current-scope gates pass; any unmet required criterion forces `HOLD`.
9. Dual reviews, finding disposition, dual verification, line-count
   governance, exact-diff reconciliation, and final disposition are complete.

## Security Impact Gate

No secrets, network service, authentication, subprocess interpolation, unsafe
Rust, or external mutation is authorized. Unknown selector values fail closed.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent code/science reviewers and two terminal
verification agents. Review scope is the exact EB-03 diff, authority,
conservation, consumer path, and gate legitimacy; expected outputs are the four
named review/verification artifacts. Review access is read-only. Verification
agents may write only their assigned package artifact.

## Progress

- [x] (2026-07-30) Reconciled EB-02 handoff with the current Stage 3 and
  sublimation paths.
- [x] (2026-07-30) Fixed the package boundary and implementation intent.
- [x] (2026-07-30) Amended contracts and authored contract-derived tests.
- [x] (2026-07-30) Implemented the diagnostic/reproduction candidate and
  real-consumer selectors.
- [x] (2026-07-30) Generated deterministic evidence and figures.
- [x] (2026-07-30) Completed validation, dual review, dual verification,
  exact-diff reconciliation, and `HOLD / CLOSE_AS_MODEL_LIMITATION`
  disposition. The required full `quick` profile remains explicitly
  `NOT PASS`; closure is a truthful HOLD, not promotion.

## Surprises & Discoveries

- The retained Stage B path reconstructs `T_s = min(T_a, 0)` before the Stage 3
  provider runs, so it cannot satisfy the shared-provider requirement without
  selector and carrier reconciliation.
- Stage 3 currently consumes shortwave only and runs after the CoE snow mass
  path. EB-03 must preserve that protected CoE boundary while carrying
  longwave/latent effects into cold content and vapor mass.
- The quick profile correctly treats `SC-SNOWFREEZE-001` as an identified
  assurance input. Its amendment requires the typed report-source adoption
  transaction and full-catalog rebuild before assurance consumers are current.
- The real direct-production S cell reaches the Stage 3 provider's
  `-273.15 degC` cold-content bound after only 18 days, with `0.0361 m` SWE
  still present and only `0.0013524 m` cumulative sublimation. The result
  reproduces with both bulk and multilayer density models, so density geometry
  is not the cause.
- Exact hourly mass/latent closure is necessary but not sufficient: the
  currently admitted shortwave-plus-latent carrier lacks an authority-complete
  feedback that keeps the shared thermal state physical. An arbitrary
  temperature clamp would hide rather than resolve that failure.

## Decision Log

- Decision: use the supported internal Stage 3 layer carrier, not the legacy
  frost `tmpadj` surface temperature.
  Rationale: Stage 3 already owns persistent snow-layer temperature and cold
  content and is the only provider shared with refreeze/liquid routing.
  Date/Author: 2026-07-30 / Codex.
- Decision: retain `T_c = T_a` as the explicitly labeled homogeneous-stand
  approximation.
  Rationale: EB-02 admits it and a prognostic canopy balance is outside the
  bounded campaign.
  Date/Author: 2026-07-30 / Codex.
- Decision: keep both new mechanisms internal diagnostic/reproduction opt-ins
  and default-off.
  Rationale: the seam records the negative result, but EB-04 is not admitted
  because S/LS are mechanically invalid.
  Date/Author: 2026-07-30 / Codex.
- Decision: invoke the campaign stop-loss and hold EB-04.
  Rationale: the real S consumer collapses the selected thermal provider to
  absolute zero; no authority-backed coupled-temperature correction is in
  scope, and a clamp or fitted limiter is prohibited.
  Date/Author: 2026-07-30 / Codex.

## Outcomes & Retrospective

EB-03 implemented and independently reconstructed the contract-bound
longwave, vapor-mass, and latent-energy pieces, established same-binary
absent/empty/disabled selector equivalence, and made B and L executable through
the real direct runner. It did not establish a viable common provider for
S/LS: real execution reaches the Stage 3 absolute-zero cold-content bound with
material snow remaining.

Disposition: `HOLD / CLOSE_AS_MODEL_LIMITATION`. Do not execute EB-04 or add a
temperature clamp, tuning round, or another coefficient. Reopen only with an
authoritative coupled snow-surface temperature/energy formulation that can be
tested independently and supplies the missing physical feedback without
changing the frozen factorial controls.
