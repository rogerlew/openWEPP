# REFACTOR009 refactor009 line count governance checklist

Status: complete  
Evidence mode: Static

## Scope
Line-count decomposition outcome for this modularization work.

## Evidence
### Files >= 2000 lines
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` reduced from `2533` to `1424`.

### Files >= 3000 lines
- None.

### Current touched file lengths
- `.../intake_lane_setup/lane_setup_helpers.rs`: `137`
- `.../intake_lane_setup/runfile_helpers.rs`: `311`
- `.../intake_lane_setup/runtime_surface_helpers.rs`: `135`
- `.../intake_lane_setup/wb11_seed_helpers.rs`: `532`
- `.../tests03/simimpl.rs`: `131`
- `.../tests03/trace.rs`: `1228`
- `.../scheduler_trace/scheduler_seed_and_runtime.rs`: `1792`
- `.../scheduler_trace/hphys_trace.rs`: `1056`

## Governance disposition
- File at or above 2000 lines after split: `00_runner_intake_and_lane_setup.rs` at `1424` (meets WARN threshold, no decomposition required).
- File at or above 3000 lines: none.
- No generated/fixture exception required.
