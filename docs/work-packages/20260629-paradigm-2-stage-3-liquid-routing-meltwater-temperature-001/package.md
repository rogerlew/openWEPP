# PARADIGM-2 Stage 3 Liquid Routing and Meltwater Temperature

Status: `HOLD-H2637-ENDPOINT-NOT-RUN-NON-PROMOTION`
Date: `2026-06-29`
Contract: `SC-SNOWFREEZE-001` v110, `INV-SNOWFREEZE-080`,
`OBL-SNOWFREEZE-P-055`
Selectors:

- `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_multilayer_density_v1`
- `OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1`

## Objective

Execute Paradigm 2 Stage 3 as an opt-in diagnostic candidate: consume the Stage
1 persistent snow-layer stack, add per-layer thermal/liquid/refreeze diagnostics,
route the existing CoE melt/rain liquid through the stack, and produce a typed
meltwater-flux temperature for the future stream-temperature program. The
current no-env default, CoE melt mass path, rollback selectors, frost behavior,
and public output schemas remain intact.

## Read-First Basis

- `docs/planning/paradigm2-multilayer-snow-specification.md` §1.1, §4 reqs
  3/4, and §6 Stage 3
- `docs/planning/snow-frost-fidelity-strategy.md` §10.3 step 10
- ADR-0029, ADR-0028, ADR-0026, ADR-0025, ADR-0011
- Stage 0 surface-energy primitives in `crates/openwepp-meteorology`
- Stage 1 and Stage 2 packages and reviews:
  `docs/work-packages/20260628-paradigm-2-stage-1-layered-snow-density-001/`
  and
  `docs/work-packages/20260628-paradigm-2-stage-2-snow-frost-insulation-profile-001/`
- `docs/backlog/20260627-stream-water-temperature-surface-energy-balance.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`

## Scope

- Contract-first v110 amendment for Stage 3 opt-in diagnostics.
- Add a package-bound Stage 3 selector with disabled default and fail-closed
  unsupported values.
- Extend the Stage 1 layer state with diagnostic temperature, cold-content,
  retained-liquid, and refreeze fields.
- Keep existing CoE snow/rain melt mass authoritative for routed melt and public
  water-balance surfaces.
- Compute a diagnostic per-layer thermal/liquid routing pass with mass, liquid,
  and energy closure, and publish typed meltwater-flux temperature internally.
- Prove no public schema, fixture, default, frost, cap, parser, runfile, user
  CLI, `.run`, Qwet/frzftp, compatibility-runtime, or full stream-temperature
  routing change.

## Non-Scope

- No default activation.
- No replacement of CoE melt with energy-balance melt.
- No snow-density promotion claim.
- No public output schema change beyond internal diagnostic state/trace.
- No full in-stream water-temperature routing.
- No fixture/site calibration.

## Gates

1. Contract-first amendment exists and tests bind `INV-SNOWFREEZE-080`.
2. Opt-in selector is fail-closed; absent selector preserves the disabled path.
3. Stage 3 consumes real Stage 1 layer state and fails closed when snow is
   present but layer state is missing.
4. Mass, liquid, and energy ledgers close from produced operands.
5. Meltwater temperature is typed and physically reasonable: ripe melt near
   `0 degC`, no layer or flux temperature above freezing, and cold-content/
   refreeze behavior captured.
6. Runoff/melt-timing forcing-robust signatures are no-worse; snow-density
   cross-SNOTEL rubric is a no-regression guardrail only.
7. ADR-0025 H2637 performance evidence is recorded.
8. Rust/doc gates pass or the package closes `HOLD`/non-promotion with explicit
   blockers.

## Evidence Artifacts

- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/authority-provenance.md`
- `artifacts/stage3-conservation-temperature.md`
- `artifacts/runoff-and-snow-guardrails.md`
- `artifacts/performance-h2637.md`
- `artifacts/review.md`
- `artifacts/verification.md`
- `artifacts/line-count-governance.md`

## Disposition

Stage 3 was scaffolded and executed as an opt-in diagnostic candidate. The
implementation is retained but not promoted:

- `SC-SNOWFREEZE-001` v110 authorizes only explicit opt-in
  `layered_thermal_liquid_v1`.
- Stage 3 adds per-layer temperature, cold-content, retained-liquid, and
  refreeze state; uses Stage 0 surface-energy and conduction primitives; routes
  existing CoE liquid through Stage 1 layers diagnostically; and emits typed
  meltwater-flux temperature.
- The disabled default, CoE melt mass path, rollback boundaries, frost behavior,
  density cap, fixtures, and public output schemas are preserved.
- Focused conservation/temperature tests, adjacent snow tests, workspace clippy,
  full workspace tests, and `cargo deny check` passed.
- ADR-0025 hot-frame size guard passed after Stage 3 diagnostics were moved to
  optional boxed direct-runtime trace carry.

Promotion/default activation is blocked:

- Real H2637 endpoint timing/RSS was not run.
- Cross-SNOTEL snow rubric and forcing-robust runoff timing were not rerun as
  observed-data gates.
- Full in-stream water-temperature routing remains separate stream-temperature
  program scope.

Closeout disposition: retain the opt-in diagnostic candidate only. No activation
or promotion is authorized from this package.
