# PARADIGM-2 Stage 1 Layered Snow Density

Status: `HOLD-GATE-FAILURE-NON-PROMOTION`
Date: `2026-06-28`  
Contract: `SC-SNOWFREEZE-001` v108, `INV-SNOWFREEZE-078`,
`OBL-SNOWFREEZE-P-053`  
Selector: `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_multilayer_density_v1`

## Objective

Scaffold and execute Paradigm 2 Stage 1: an opt-in density-only snow candidate
with persistent winter-column layer state and per-layer densification under local
overburden. The current no-env default and `legacy_wepp` rollback remain intact.

## Read-First Basis

- ADR-0029, ADR-0028, ADR-0026, ADR-0025, ADR-0011
- `docs/planning/paradigm2-multilayer-snow-specification.md` §6 Stage 1
- `docs/planning/snow-frost-fidelity-strategy.md` §10.3
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/winter_column.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`

## Scope

- Add `DirectSnowLayerState` and persist `DirectSnowLaneState.layers` through the
  existing winter-column/carry/R4G snow-coupling path.
- Add opt-in `physics_bulk_multilayer_density_v1` behind the existing
  package-bound density selector only.
- Reuse the existing Anderson/SNOBAL compaction constants; the only physics delta
  is per-layer local overburden instead of bulk total-pack overburden.
- Keep aggregate public WAT/output schemas unchanged.

## Implementation Summary

Stage 1 is implemented as an opt-in density candidate only. The implementation
adds persistent `DirectSnowLayerState` entries under `DirectSnowLaneState.layers`
and carries that layer stack through constructor carry, winter-column state,
typed snow partition, R4G snow coupling, and direct-production snow traces.

The new selector is
`OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_multilayer_density_v1`. The
absent-selector default remains the activated
`physics_bulk_density_compaction_v1` density model composed with Harder-Pomeroy
hourly phase partitioning, and `legacy_wepp` remains an explicit rollback.

The only physics delta is local overburden in the existing Anderson/SNOBAL-form
compaction step: each layer uses the mass of overlying layers plus its own mass,
instead of applying total-pack overburden to a single bulk density state. New
snow is inserted as the surface layer. Boundary mass changes remove or add mass
at the surface while preserving aggregate SWE/depth closure. The active
`522 kg m^-3` density cap is unchanged and applies to each layer and the emitted
aggregate density.

The diagnostic trace now includes layer count and layer SWE/depth sums before
and after the snow partition step. This is diagnostic-only evidence; public WAT
schema and production output fixtures are unchanged.

## Non-Scope

- No default activation.
- No frost, melt, phase, canopy, radiation, fixture, density-cap, schema, `.run`,
  parser, runfile, user CLI, Qwet/frzftp, or compatibility-runtime change.
- No Stage 2 thermal solve and no Stage 3 liquid routing.

## Gates

1. Contract-first amendment exists and tests bind `INV-SNOWFREEZE-078`.
2. Opt-in selector accepted; absent selector still uses
   `physics_bulk_density_compaction_v1`; `legacy_wepp` rollback preserved.
3. Layer stack persists through direct-production snow consumer boundaries and
   aggregate SWE/depth/density reconstruct from layers.
4. Unit tests prove local-overburden densification and fail-closed invalid layer
   aggregates.
5. Cross-SNOTEL+cancov `INV-SNOWFREEZE-050` real run beats current default
   `15/179`, proves bidirectional densification improvement, creates no new
   persistence tail, and closes conservation.
6. ADR-0025 performance evidence recorded.
7. Rust/doc gates pass or the package closes `HOLD` with explicit blockers.

## Gate Disposition

The real coupled direct-production cross-SNOTEL+cancov WAT/trace run completed
for the current no-env default and the opt-in
`physics_bulk_multilayer_density_v1` candidate. The candidate is executable,
mass-conserving, and layer-persistent, but it fails the primary promotion gates:

- activated default rerun profile: `15` robust fails / `179` robust score;
- Stage 1 candidate profile: `16` robust fails / `177` robust score;
- candidate improvements: `3` robust cells;
- candidate regressions: `6` robust cells;
- bidirectional densification/persistence guard: not achieved.

Conservation and layer persistence evidence from candidate traces:

- candidate trace rows: `159986`;
- max snow-state residual: `8.881784197001252e-16 m`;
- max partition residual: `5.551115123125783e-17 m`;
- rows with nonempty layer stack after partition: `65459`;
- max layer SWE residual after partition: `4.440892098500626e-16 m`;
- max layer depth residual after partition: `0.0 m`;
- tolerance: `1e-9 m`.

| Gate | Status | Evidence |
|---|---|---|
| Contract-first amendment and selector binding | Pass | `SC-SNOWFREEZE-001` v108; `tests/integration/paradigm2_stage1_layered_snow_density.rs`. |
| Opt-in only; default and rollback preserved | Pass | Selector test and source guard; no default activation. |
| Layer stack persists through direct snow consumer | Pass | R4G focused test; observed trace layer rows `65459/159986`. |
| Local-overburden and fail-closed unit behavior | Pass | Stage 1 integration tests. |
| Cross-SNOTEL+cancov rubric beats current default | Fail | Candidate `16/177` vs default `15/179`. |
| Bidirectional densification/persistence guard | Fail | No robust densification/persistence improvements; `6` worse robust cells. |
| Whole-model conservation | Pass | Max snow-state residual `8.881784197001252e-16 m`; max partition residual `5.551115123125783e-17 m`. |
| ADR-0025 performance evidence | Pass | Real-run elapsed `712.821 s` for two models across the observed corpus. |
| Protected boundaries | Pass | No default, rollback, fixture, public schema, frost, density-cap, melt, phase, canopy, radiation, parser, runfile, user CLI, Qwet/frzftp, or compatibility-runtime activation change. |

## Evidence Artifacts

- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/paradigm2-stage1-layered-density-rubric.json`
- `artifacts/paradigm2-stage1-layered-density-rubric.md`
- `artifacts/review.md`
- `artifacts/verification.md`
- `artifacts/line-count-governance.md`

## Disposition

`HOLD-GATE-FAILURE-NON-PROMOTION`.

No activation is authorized. The Stage 1 candidate remains available only
through the package-bound opt-in selector for further diagnosis. The current
no-env default and `legacy_wepp` rollback remain unchanged.
