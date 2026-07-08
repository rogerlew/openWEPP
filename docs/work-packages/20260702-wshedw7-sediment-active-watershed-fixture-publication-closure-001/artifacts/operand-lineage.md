# Operand Lineage

Status: `passed`

Evidence mode: `Static:` source review plus `Ran:` reconstruction.

| Operand | Units | Source authority | Public output | Rejected aliases |
| --- | --- | --- | --- | --- |
| `total_detachment_kg` | kg | Generated HBP latest event payload parsed by `PassInventory::validate` | `totalwatsed3.tdet` | zero-fill, pass-file manual edit, legacy `loss.dat` soil-loss summary |
| `total_deposition_kg` | kg | Generated HBP latest event payload parsed by `PassInventory::validate` | `totalwatsed3.tdep` | zero-fill, pass-file manual edit, detached-only alias |
| `sediment_yield_kg` | kg | Typed `RoutedChannelState::sediment_yield_kg` from `execute_watershed_dispatch_with_frame` | `ebe_pw0.sediment_yield`, `totalwatsed3.sed_del` | `tdet`, `tdep`, `tdet - tdep`, zero-fill |
| `runoff_volume_m3` | m^3 | Typed routed channel state | `ebe_pw0.runoff_volume`, `totalwatsed3.runvol`, `chanwb.Inflow` | depth-only recomputation as source of truth |
| `runoff_mm` | mm | `runoff_volume_m3 / contributing_area_m2 * 1000` | `totalwatsed3.Runoff` | independent volume source |

W7R does not change publication formulas, schemas, units, or sediment physics.
The new fixture/test proves the existing lineage on a nonzero sediment case.
