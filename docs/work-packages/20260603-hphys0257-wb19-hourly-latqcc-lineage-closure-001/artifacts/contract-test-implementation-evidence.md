# Contract-Test Implementation Evidence

Status: completed

Evidence mode: static

## Tests Added or Updated

- Static: `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
  adds `hphys0257_hourly_modern_lanes_use_ui_ssh_lateral_conductivity`, proving
  hourly modern lanes consume `wb19_lateral_ssh_####` instead of vertical
  `wb18_perc_ssc_####`.
- Static: `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
  adds `hphys0257_hourly_modern_lanes_fail_closed_without_ui_ssh_lateral_conductivity`,
  proving missing modern hourly `wb19_lateral_ssh_####` returns
  `HKERNEL-WB11-LAT-E-001` / `MissingRequiredInput`.
- Static: `tests/integration/hphys0256_wb19_latqcc_lane_branch_contract.rs`
  seeds `wb19_lateral_ssh_####` so the HPHYS0256 lane-regression tests remain
  authoritative under the stricter hourly surface requirement.
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  verifies runtime soil projection publishes `wb19_lateral_ssh_0001` and
  `wb19_lateral_ssh_0002`.
- Static: the same runtime projection test verifies modern UI soils publish
  profile `wb19_lateral_anisotropy_ratio = 1.0`, preventing double application
  of layer `ui_anisrt` after `ui_ssh` projection.
