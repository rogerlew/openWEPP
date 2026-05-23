# CLIM06 Contract-Test Implementation Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Implemented Contract-Test Targets
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
- `tests/integration/parser_runtime_seam_integration.rs` (CLIM06 frost seam vectors)
- Registered `clim06_frost_frozen_soil_kernel_contract` in `Cargo.toml`.

## CLIM06 Contract-Derived Vectors
1. Active CLIM06 coupling publishes deterministic `frost.runtime_*` surfaces and reduces WB14 infiltration-capacity via `frost.runtime_infcap_frz`.
2. Missing required active-coupling frost symbol -> `HKERNEL-WB14-RUNOFF-E-001`.
3. Non-finite active-coupling frost symbol -> `HKERNEL-WB14-RUNOFF-E-002`.
4. Out-of-domain active-coupling frost symbol/state -> `HKERNEL-WB14-RUNOFF-E-003`.
5. Frost parser-to-runtime seam closure publishes required `frost.options.*` + `frost.runtime_*` seed surfaces.
6. Frost seam projection domain-invalid kfactor vector -> `HS-RUNTIME-E-055`.

## Pre-Implementation Gate Result
Command:
```bash
cargo test --test clim06_frost_frozen_soil_kernel_contract
```
Observed pre-implementation result:
- `0 passed; 4 failed`.
- Failures confirmed absence of CLIM06 active-coupling runtime outputs/guards before production implementation.
