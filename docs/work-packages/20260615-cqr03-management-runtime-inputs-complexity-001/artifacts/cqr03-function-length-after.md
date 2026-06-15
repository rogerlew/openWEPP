# Function Length After

Static: measured from the refactored working tree with a top-level function span
scan of
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`.

| Lines | Count | Function |
|---|---:|---|
| 87-101 | 15 | `build_hillslope_pl_runtime_surfaces_from_management` |
| 259-333 | 75 | `build_initial_seed_projection` |
| 635-707 | 73 | `project_yearly_crop_slot_surfaces` |
| 907-962 | 56 | `project_annual_or_fallow_crop_slot` |
| 992-1054 | 63 | `project_perennial_crop_slot` |
| 1247-1255 | 9 | `apply_primary_initial_live_canopy_assimilation` |
| 1256-1274 | 19 | `read_primary_initial_live_canopy_inputs` |

Static: the formerly long dispatcher and live-canopy helper were reduced from
`869` and `181` lines to `15` and `9` lines respectively. The largest extracted
helper in the target file is `75` lines.

File line count after: `1551`.
