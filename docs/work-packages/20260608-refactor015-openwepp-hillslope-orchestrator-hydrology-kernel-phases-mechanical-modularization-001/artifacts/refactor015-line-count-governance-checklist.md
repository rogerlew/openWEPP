# REFACTOR015 line count governance checklist

Status: complete
Evidence mode: static
Date: 2026-06-08

## Static
Baseline inventory:
- pre-refactor file `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`: `6996` lines
- exceeds 3000-line governance threshold

Post-refactor inventory:
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`: `1` line
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_infiltration_evap.rs`: `1257` lines
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`: `1027` lines
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`: `1597` lines
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`: `1020` lines
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion.rs`: `2110` lines
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/mod.rs`: `5` lines

Governance outcome:
- files >=2000 lines: `hydrology_phase_storage_erosion.rs` (2110)
- files >=3000 lines: none
- decomposition objective achieved: yes
