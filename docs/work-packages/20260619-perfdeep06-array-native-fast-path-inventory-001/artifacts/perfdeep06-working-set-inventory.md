# PERFDEEP06 Working-Set Inventory

Status: complete 2026-06-19.
Evidence class: Static + Ran.

## Static Source Map

- The fixed phase order is 14 phases in
  `crates/openwepp-hillslope-orchestrator/src/phase.rs`: normalization,
  storage bounds, decomposition transition, residue partition transition,
  annual growth transition, perennial growth transition, percolation deep
  seepage, evapotranspiration, drainage, lateral transfer, plant root uptake,
  runoff reconciliation, storage reconciliation, closure diagnostics.
- `OfeLanePersistentState` currently carries `HillslopeWritebackSurface`, an
  optional indexed surface, and optional `HillslopeLaneDenseState` in
  `scheduler.rs`.
- `HillslopeLaneDenseState` in `day_frame.rs` is compact but still stores
  `Option<BoundaryValue>` slots addressed through `SymbolId`; it is a transition
  adapter, not the production frame.
- `HillslopeKernelRequest` in
  `02_boundary_values_and_kernel_requests.rs` still borrows logical
  `BTreeMap<BoundarySymbol, BoundaryValue>` state/flux surfaces and optional
  dense/indexed views.
- `state_access.rs` reads through hot indexed lookup, dense slot lookup, then
  logical fallback.
- The same dense-first lookup chain is now a default-path concern, not just an
  opt-in island concern: PERFDEEP05 default-disabled measured `701.95 s` versus
  the `669.97 s` activation reference, and PERFDEEP03 default-disabled measured
  in the `697-708 s` band. PERFDEEP07 must make this plumbing zero-cost when
  all PERFDEEP opt-ins are disabled.
- WB13/WAT/PASS/HBP publication assembly is in
  `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`.

Ran:

- `wc -l` over inspected files recorded the current line-count context.
- `rg` over hydrology, scheduler, runner, and kernel-contract files found 451
  symbol/runtime-surface/writeback access sites in the inspected hot-path and
  publication scopes. This is a planning signal, not a complete semantic count.

## Frame Inventory

| Group | Current source | Lifetime | Unit/shape | Phase owner / consumers | Direct-frame disposition |
|---|---|---|---|---|---|
| Calendar keys | `ClimateDayProjection`, WB13 calendar helpers | borrowed day context | year/month/day/julian ints | scheduler, WB13 projection | Keep in `HillslopeDayContext`, not mutable frame state. |
| Lane identity/area | `OfeLanePersistentState`, `TransferInput.area_ratio`, static lane slices | lane context | `usize`, `f64` area ratio | MOFE transfer, publication | Keep in `HillslopeLaneContext`; only per-day transfer totals enter frame. |
| MOFE transfer input | `TransferInput { surface_carry, lateral_carry, upstrmq, subrin }` | start-of-day seed / same-day lane transfer | `[f64; 24]`, scalar depths | scheduler before phase execution; runoff/lateral consumers | Store as fixed arrays and scalar totals in frame; do not materialize `ui_SUrunf_*`, `ui_LfUrf_*`, `UpStrmQ`, `SubRIn` symbols. |
| MOFE transfer output | `TransferOutput { surface_carry, lateral_carry, qofe, lateral_export }` | phase-owned output / downstream seed | `[f64; 24]`, scalar totals | runoff/lateral phases, next OFE input, PASS/WAT outlet | Store as fixed arrays plus typed totals; downstream transfer reads frame directly. |
| Core water scalars | `WB11_SYMBOL_SOIL_WATER`, `WB12_SYMBOL_INFILTRATION`, `WB12_SYMBOL_RAINFALL_INPUT`, `WB12_SYMBOL_RUNOFF_Q`, `WB11_SYMBOL_ET`, `WB11_SYMBOL_WS`, `Q`, `D`, `q`, `Qd`, `Qdd` | lane-persistent or phase-owned | mostly `WaterDepthMeters`, rates where named | percolation, ET, drainage, lateral, runoff, storage, WB13 | Named typed fields; raw `f64` only where unit wrapper is unavailable. |
| Soil/profile layers | `nsl`, `solthk`, `dg`, `thetdr`, `thetfc`, `ssc`, `ul`, `theta`, `fc`, lateral conductivity/threshold vectors | lane-persistent / start-of-day seed | bounded layer count; SoA slices | percolation, lateral drainage, runoff storage, WB13 profile | `SoilLayerColumns` struct-of-arrays with active count; preallocate once per lane. |
| Frost fine layers | `frost.runtime_*`, fine-layer symbols, frozen water and active frost depth | lane-persistent / phase-owned | scalar + layer arrays | frost coupling, percolation, runoff, WB13 | `FrostColumns` and `FrostRuntimeState`; no symbol fallback in migrated path. |
| Snow runtime | `snow.runtime_swe`, `snow.runtime_depth_m`, density, settle count, routed melt/post-winter rain fluxes | lane-persistent / same-day flux | typed scalars and hourly series | snow coupling, runoff, WB13 | `SnowRuntimeState` plus borrowed hourly forcing/results view; publication fields captured in projection. |
| Plant/growth state | `cancov`, `lai`, `vdmt`, `rtd`, `rtmass`, `sumgdd`, management schedule symbols | lane-persistent / transition output | typed/f64 scalars, schedule tables | growth transitions, ET, runoff | Direct-frame fields for hot scalars; schedule tables remain borrowed/static until growth port. |
| Climate/hyetograph forcing | `prcp`, `tmax`, `tmin`, `rad`, `vwind`, `tdpt`, `intsty_*`, winter hourly arrays | borrowed read-only day forcing | slices/arrays, not per-phase copies | ET, irrigation, runoff, snow | `HillslopeDayForcing<'a>` borrowed by frame/context. Do not copy into slots. |
| Erosion diagnostics | `peakro`, `watdur`, `total_detachment_kg`, `total_deposition_kg`, `sediment_concentration_kg_m3_*` | phase-owned / I/O projection | typed scalars + `[f64; 5]` sediment classes | storage/erosion, HBP/PASS | Capture as typed terminal projection fields; not logical hot-path state. |
| WB13 publication terms | `P`, `RM`, `Q`, `Ep`, `Es`, `Er`, `Dp`, `UpStrmQ`, `SubRIn`, `latqcc`, `Total-Soil`, `frozwt`, `frdp`, `Snow-Water`, `QOFE`, `Tile`, `Irr`, `Area`, profile stores | per-day projection | mostly mm, `Area` m2 | WB13/WAT/PASS publication | `HillslopeDayPublicationProjection` built from frame fields at I/O edge. |
| Replay/diagnostic symbols | HPHYS traces, indexed shadow surface, manifest provenance | diagnostic/replay only | symbol/value report | optional diagnostics | Keep compatibility adapters outside migrated phase loop. |
| Default-disabled compatibility plumbing | dense-first resolution, optional compact dense views, indexed/logical fallback setup | current `main` default path unless bypassed | branch/path-selection overhead, not physical state | scheduler and `state_access.rs` | PERFDEEP07 P0 cleanup: no dense/indexed frame resolution on default path unless the opt-in is active or a shadow gate explicitly requests it. |

## Gate

PASS. PERFDEEP07 can derive its initial frame schema from this inventory:
`HillslopeDayContext`, `HillslopeDayForcing<'a>`, `HillslopeDayFrame`,
`SoilLayerColumns`, `FrostColumns`, `SnowRuntimeState`,
`MofeTransferFrame`, and `HillslopeDayPublicationProjection`.
