# Climate Seam Adapter Ownership Contract

Evidence mode: `Static`
Status: `complete`

## Scope
CLIM02 closes climate parser-to-runtime seam ownership for hillslope and watershed orchestrators with typed adapter failures and explicit runtime projection boundaries.

## Ownership Boundary

| seam_id | parser producer | adapter owner | runtime consumer | seam entrypoint(s) | canonical symbols surfaced |
|---|---|---|---|---|---|
| `HS-CLIM-SEAM-001` | `openwepp_input_contract::parsers::climate::parse_climate_file/parse_climate_from_str` | `openwepp_hillslope_orchestrator::runtime_inputs` | `HillslopePhaseScheduler::execute_with_kernel` (immutable request views) | `build_hillslope_climate_runtime_request`, `seed_hillslope_runtime_surface_from_climate`, `build_hillslope_runtime_surface_from_climate` | `datver`, `iclig`, `itemp`, `ibrkpt`, `iwind`, `day`, `mon`, `year`, `prcp`, `stmdur`, `timep`, `ip`, `timem_*`, `intsty_*`, `tmax`, `tmin`, `rad`, `vwind`, `wind`, `tdpt` |
| `WS-CLIM-SEAM-001` | `openwepp_input_contract::parsers::climate::parse_climate_file/parse_climate_from_str` plus watershed hillslope-assignment map | `openwepp_watershed_orchestrator::runtime_inputs` | `execute_watershed_dispatch_with_kernel` (immutable request views) | `build_watershed_climate_runtime_request_from_assignments`, `seed_watershed_runtime_surface_from_climate`, `build_watershed_runtime_surface_from_climate_assignments` | `nclimhs` plus per-hillslope prefixed canonical symbols (`hs{ID}_datver`, `hs{ID}_iclig`, `hs{ID}_prcp`, `hs{ID}_stmdur`, `hs{ID}_timep`, `hs{ID}_ip`, `hs{ID}_timem_*`, `hs{ID}_intsty_*`, etc.) |

## Enforced Runtime Policy
1. `datver=0.0` is explicitly supported and mapped to `iclig=0`.
2. `datver>=4.0` is supported and mapped to `iclig=1`.
3. `0.0<datver<4.0` is rejected via typed `CLIM-RUNTIME-E-001`.
4. Single-storm `itemp=2` is rejected at seam boundaries via `CLIM-RUNTIME-E-002`.
5. Breakpoint intervals enforce strict `dtime>0` for every interval; duplicate/decreasing `timem` fail typed (`CLIM-RUNTIME-E-009`/`E-010`).
6. No compatibility-default silent fallback is introduced for required climate runtime inputs.

## Ownership and Mutability
1. Parser output remains source-faithful input data.
2. Adapter layer owns unit projection (`mm->m`, `hr->s`) and typed seam guards.
3. Scheduler/kernel boundaries consume immutable request views from orchestrator-owned surfaces.
4. Kernel code does not mutate parser/adapter-owned climate request structures.

## Evidence
- [DIRECT] `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:385`
- [DIRECT] `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:422`
- [DIRECT] `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:739`
- [DIRECT] `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:436`
- [DIRECT] `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:460`
- [DIRECT] `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:817`
