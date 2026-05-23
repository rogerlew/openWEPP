# CLIM05 Contract-Test Implementation Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Implemented Test Target
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- Registered in `Cargo.toml` as `clim05_snow_runtime_kernel_contract`

## Contract-Derived Tests
1. `clim05_contract_conformance_couples_snow_controls_into_hydrology_reconciliation`
- Verifies active snow-control coupling expectations for deterministic
  hydrology reconciliation surfaces (`S`, `snow.runtime_swe`, `Q`,
  `wb12_storage_reconciled`).

2. `clim05_contract_conformance_rejects_missing_active_snow_control_symbol`
- Verifies typed missing-input guard posture for active-coupling
  `snow.options.*` requirements.

3. `clim05_contract_conformance_rejects_non_finite_active_snow_control_symbol`
- Verifies typed non-finite guard posture for active-coupling snow controls.

4. `clim05_contract_conformance_rejects_invalid_active_snow_control_domain`
- Verifies typed domain-failure guard posture for invalid control domains
  (`newsnw > ssd`).

## Pre-Implementation Gate Execution
Command:
```bash
cargo test --test clim05_snow_runtime_kernel_contract
```

Observed result before production CLIM05 kernel implementation:
- `1 passed; 3 failed`
- Failure signatures show current runtime hydrology behavior does not yet
  satisfy active snow-coupling contract semantics.
- Observed typed message ids for missing/non-finite active-coupling tests were
  `HKERNEL-WB14-RUNOFF-E-003` (legacy domain-closure posture), not the
  CLIM05-required missing/non-finite guard posture.
