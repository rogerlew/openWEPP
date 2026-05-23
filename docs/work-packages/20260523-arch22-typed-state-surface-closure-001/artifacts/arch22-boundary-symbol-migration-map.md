# ARCH22 Boundary Symbol Migration Map

Status: `completed`
Evidence mode: `Static`

## Covered Accessor Surface Migration

| file | previous surface | ARCH22 migrated surface |
|---|---|---|
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | guard/accessor signatures consumed string symbol ids (`&'static str` / `&str`) | guard/accessor signatures consume `HillslopeProductionStateSymbol` / `HillslopeProductionFluxSymbol` |
| `crates/openwepp-watershed-orchestrator/src/lib.rs` | guard/accessor signatures consumed string symbol ids (`&str`) | guard/accessor signatures consume `WatershedProductionStateSymbol` / `WatershedProductionFluxSymbol` |

## Hillslope Symbol Family Migration

| lane/family | prior usage form | ARCH22 typed form |
|---|---|---|
| WB11 state inputs | string constants (`"wb11_*"`) | `HillslopeProductionStateSymbol::Wb11*` |
| WB11 flux outputs | string constants (`"ET"`, `"Ws"`, `"D"`, `"Pe"`, `"q"`, `"Qdd"`, `"Qd"`) | `HillslopeProductionFluxSymbol::Wb11*` |
| WB12 state inputs | string constants (`"wb12_*"`) | `HillslopeProductionStateSymbol::Wb12*` |
| WB12 flux/closure symbols | string constants (`"Q"`, `"S"`, `"wb12_*_delta"`) | `HillslopeProductionFluxSymbol::Wb12*` |
| WB14 coupling symbols | string constants (`ninten`, `nbrkpt`, `snow.*`, `frost.*`) | `HillslopeProductionStateSymbol::Wb14*` |
| WB15 coupling symbols | string constants (`"I"`, `"cancov"`, `"lai"`, `"vdmt"`) | `HillslopeProductionFluxSymbol::Wb15InterceptionI`, `HillslopeProductionStateSymbol::Wb15Plant*` |
| WB16 symbols | string constants (`timep`, `efflen`, `ealpha`, `m`, `peakro`, `watdur`, `wb16_*`) | `HillslopeProductionStateSymbol::Wb16*` |
| Irrigation runtime controls | string constants (`irrigation.runtime_*`) | `HillslopeProductionStateSymbol::IrrigRuntime*` |
| Irrigation fixed-date indexed fields | ad-hoc string formatting | `HillslopeProductionStateSymbol::IrrigationFixedDateEvent { event_index, field }` + `HillslopeIrrigationFixedDateEventField::*` |
| Irrigation depletion indexed fields | ad-hoc string formatting | `HillslopeProductionStateSymbol::IrrigationDepletionPeriod { period_index, field }` + `HillslopeIrrigationDepletionPeriodField::*` |
| Daily irrigation forcing flux | string constant (`"Irr"`) | `HillslopeProductionFluxSymbol::IrrigDailyIrrigation` |

## Watershed Symbol Family Migration

| lane/family | prior usage form | ARCH22 typed form |
|---|---|---|
| global WS10 controls | string literals (`"dtchr"`, `"nchnum"`) | `WatershedProductionStateSymbol::{Dtchr,Nchnum}` |
| global WS10 flux control | string literal (`"cbase"`) | `WatershedProductionFluxSymbol::Cbase` |
| channel node state fields | formatted strings (`ws10_channel_{id}_{field}`) | `WatershedProductionStateSymbol::ChannelNode { node_id, field: WatershedChannelStateField::* }` |
| channel node flux fields | formatted strings (`ws10_channel_{id}_roff`) | `WatershedProductionFluxSymbol::ChannelNode { node_id, field: WatershedChannelFluxField::Roff }` |
| impoundment node state fields | formatted strings (`ws10_impoundment_{id}_{field}`) | `WatershedProductionStateSymbol::ImpoundmentNode { node_id, field: WatershedImpoundmentStateField::* }` |
| impoundment node flux fields | formatted strings (`ws10_impoundment_{id}_outflow_volume`) | `WatershedProductionFluxSymbol::ImpoundmentNode { node_id, field: WatershedImpoundmentFluxField::OutflowVolume }` |
| hillslope contributor payloads | formatted strings (`hs{id}_peakro`, `hs{id}_watdur`) | `WatershedProductionStateSymbol::{HillslopeContributorPeak,HillslopeContributorDuration}` |

## Write-Scope Matrix

| scope | files |
|---|---|
| Contract authority | `SC-PLANT-001.md`, `SC-RESIDUE-001.md`, `SC-WATBAL-001.md`, `SC-RUNOFFPART-001.md`, `SC-ROUTE-001.md`, `SC-HYDRAULICS-001.md`, `SC-IMPOUND-001.md`, `SC-SYSTEM-001.md`, `science-contracts/index.md`, `symbol-alias-registry.md` |
| Typed symbol definitions | `crates/openwepp-kernel-contract/src/lib.rs` |
| Production hillslope migration | `crates/openwepp-hillslope-orchestrator/src/lib.rs` |
| Production watershed migration | `crates/openwepp-watershed-orchestrator/src/lib.rs` |
| Contract-derived migration proof tests | `tests/integration/arch22_typed_state_surface_contract.rs`, `Cargo.toml` |
