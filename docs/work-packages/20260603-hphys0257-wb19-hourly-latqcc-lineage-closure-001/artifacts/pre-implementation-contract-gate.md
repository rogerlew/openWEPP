# Pre-Implementation Contract Gate

Status: completed

Evidence mode: ran

## Red-Gate Command

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator soil_runtime_surface_contains_canonical_state_symbols -- --exact --nocapture
cargo test --test wb19_lateral_drainage_physics_kernel_contract hphys0257 -- --nocapture
```

## Result

- Ran: the first command used an over-specific filter and ran `0` tests.
- Ran: the HPHYS0257 WB19 contract-derived tests failed before production
  edits, as required:
  - `hphys0257_hourly_modern_lanes_use_ui_ssh_lateral_conductivity` observed
    `q=0.030547012947258874` instead of expected
    `0.015273506473629437`.
  - `hphys0257_hourly_modern_lanes_fail_closed_without_ui_ssh_lateral_conductivity`
    returned OK instead of `HKERNEL-WB11-LAT-E-001`.
