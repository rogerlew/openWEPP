# CLIM06 Infiltration/Runoff Branch Coupling Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Coupling Rule Implemented
- Active coupling (`frost_file_present=1` and `wintRed=1`) computes `frost.runtime_infcap_frz` and uses it as WB14 infiltration conductivity.
- Inactive coupling falls back to baseline `ssc` conductivity.

## Active Branch Evidence
- Active frost vector produced `frost.runtime_infcap_frz = 0.1` from:
  - `ssc = 0.5`
  - `kfactor_floor = min(0.2, 0.4, 0.5) = 0.2`
  - `freeze_fraction = 1.0` (`Dfrost=0.2`)
- Contract test assertion confirms:
  - `wb12_infiltration(active) < wb12_infiltration(inactive)`
  - `Q(active) > Q(inactive)`

## Typed Guard Evidence
- Missing active frost symbol vector -> `HKERNEL-WB14-RUNOFF-E-001`.
- Non-finite active frost symbol vector -> `HKERNEL-WB14-RUNOFF-E-002`.
- Domain-invalid active frost symbol vector -> `HKERNEL-WB14-RUNOFF-E-003`.

## Command Evidence
```bash
cargo test --test clim06_frost_frozen_soil_kernel_contract
```
Result: pass (`4 passed`).
