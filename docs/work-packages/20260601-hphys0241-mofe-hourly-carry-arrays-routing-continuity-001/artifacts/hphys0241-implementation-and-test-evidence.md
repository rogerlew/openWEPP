# HPHYS0241 Implementation and Test Evidence

Status: complete
Evidence mode: static + ran

Static: production implementation landed in the declared kernel/runner scope.

- `crates/openwepp-hillslope-orchestrator/src/constants.rs`: added canonical
  MOFE hourly carry-array constants and symbol roots.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`:
  added binary activation guard, 24-slot array validation, upstream
  array-derived carryover resolver, aggregate anti-shadow validation, and
  fail-closed saturation-carry handling for cadence-dependent material
  top-layer excess.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`:
  WB19 now publishes realized hourly `ui_LfCrf` substep values; WB14/WB12
  consumes upstream arrays, publishes `wb12_runoff_carryover`, publishes
  `ui_SCrunf`, and copy-forwards `ui_SCrunf -> ui_SUrunf` plus
  `ui_LfCrf -> ui_LfUrf`.
- `crates/openwepp-runner/src/hillslope/mod.rs`: runner seeds carry activation,
  24-slot arrays, area-ratio provenance, and manifest `mofe_hourly_carry`
  provenance with required arrays and aggregate totals.
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`: watershed intake
  validates active 24-slot carry metadata for multi-OFE contributors before
  accepting HBP payloads.
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`: valid
  multi-OFE manifest fixture includes required HPHYS0241 metadata.

Static: no heuristic or surrogate positive saturation distribution was added.
When material top-layer saturation excess is present without cadence-complete
hourly lineage, the implementation hard-fails rather than synthesizing an
hourly `ui_SCrunf` distribution.

Ran: see `gate-results.md` for workspace gates and targeted command results.
