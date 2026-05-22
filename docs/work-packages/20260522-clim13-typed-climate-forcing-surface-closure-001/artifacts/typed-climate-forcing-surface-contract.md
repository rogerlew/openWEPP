# Typed Climate Forcing Surface Contract

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Introduced typed climate forcing symbol-surface model in `openwepp-kernel-contract`:
  - `ClimateForcingSymbolSurface`
  - `ClimateForcingSymbolSurfaceError`
  - `MAX_CLIMATE_FORCING_SERIES_POINTS`
- Rewired hillslope and watershed climate runtime seams to precompute per-day typed symbol surfaces and consume those symbols during runtime seeding.
- Removed hot-path series string synthesis for `timem_*`/`intsty_*` writes in both orchestrators.

Ran:
- `rg -n "format!\(\"timem_|format!\(\"intsty_" crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
  - observed: no matches.

## Boundary Contract
1. Canonical alias continuity is preserved:
- hillslope series aliases remain `timem_XXXX` and `intsty_XXXX`.
- watershed scoped series aliases remain `hs{hillslope_id}_timem_XXXX` and `hs{hillslope_id}_intsty_XXXX`.

2. Typed projection boundary:
- series alias synthesis now occurs in typed-surface builders, not inside day-seeding loops.
- runtime day seeding consumes precomputed symbol vectors from typed request/assignment surfaces.

3. Cardinality boundary:
- typed symbol-surface builders enforce `MAX_CLIMATE_FORCING_SERIES_POINTS`.
- hillslope and watershed seam errors map cardinality overflow to existing climate-runtime taxonomy.

## CLIM11/CLIM12 Alignment
- CLIM11 ownership boundary is preserved.
- CLIM12 shared climate adaptation logic remains authoritative for forcing values; CLIM13 only changes typed alias projection surfaces.

## Acceptance Check
1. Typed forcing symbol surface implemented: `met`.
2. Hot-path `timem/intsty` dynamic string synthesis removed: `met`.
3. Canonical alias continuity preserved: `met`.
