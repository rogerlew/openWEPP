# Stage 1 Seed-Authority Migration

Status: EXECUTED-HOLD-STAGE1C-TYPED-LANE-SEED-AUTHORITY-MISSING

Package id: `20260630-stage1-seed-authority-migration-001`

Continuation of
`docs/work-packages/20260630-typed-direct-setup-symbol-map-carrier-deletion-001/`.

## Objective

Migrate the remaining production direct `HillslopeWritebackSurface` seed
authority surfaces to typed inputs, one surface at a time, preserving identity
and monotonically burning down direct-publication seed reads from `208` to `0`.

## Required Reading

- `docs/work-packages/20260630-typed-direct-setup-symbol-map-carrier-deletion-001/package.md`.
- `docs/work-packages/20260630-typed-direct-setup-symbol-map-carrier-deletion-001/artifacts/disposition.md`.
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`.
- `crates/openwepp-runner/src/hillslope/direct_publication/`.
- `crates/openwepp-input-contract/`.
- `docs/architecture/array-native-runtime-specification.md` section 4.11 and
  the single-authority rules.

## Scope

Stage 1B: lane seeding to typed lane/topology construction.

Stage 1C: day-zero constructor seeding to typed values.

Stage 1D: `DirectProductionDayInputBuilder` authority construction to typed
values.

Stage 1E: coupling/publication metadata seeding to typed values.

Stage 2 and Stage 3 are allowed only after the Stage 1 seed-read count reaches
`0`.

## Non-Scope

- No Stage 2 symbol-map runtime deletion until seed reads equal `0`.
- Do not delete the explicit `--compatibility-runtime` replay seam.
- No physics change.
- No output-schema change.

## Gates

- Per-surface identity: H2637 and the multi-OFE/Wave-2 fixture remain
  identical. HBP/loss/manifest byte identity; WAT/PASS schema, row count, and
  value identity.
- Seed-read count is reported after each surface and must monotonically
  decrease.
- RSS does not regress from the run-length-flat endpoint; any additional drop is
  recorded.
- Stage 1 completes only when production direct constructs no
  `HillslopeWritebackSurface` seed authority and the seed-read count is `0`.

## Initial Inventory

Static starting point:

- Direct-publication seed-read matches: `208`.
- Narrowed production direct/publication helper `HillslopeWritebackSurface`
  references: `59`.

## Disposition

Executed HOLD. Stage 1B landed and verified as a typed lane/topology seeding
slice, reducing the direct-publication seed-read inventory from `208` to `207`
and narrowed helper `HillslopeWritebackSurface` references from `59` to `58`.

The package stops before Stage 1C because the next surface is the day-zero
constructor authority for soil/layers, evapotranspiration stage, plant growth,
and snow/frost. Those values are still only assembled through
`HillslopeWritebackSurface` seed surfaces. A truthful Stage 1C requires a typed
per-lane seed-authority carrier built from parsed input-contract data and
threaded into direct execution before the constructor can stop reading the
symbol-map surface.
