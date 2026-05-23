# CLIM06 Cold-Season Fixture Replay Evidence

Status: `completed`
Evidence mode: `Ran`

## Fixture Shape
- Test harness: `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
- Cold-season forcing vector:
  - `Tmax = -2.0 degC`
  - `Tmin = -10.0 degC`
  - Hyetograph: `ninten=4`, `timem=[0,1,2,3]`, `intsty=[1,1,1,0]`
- Active frost controls:
  - `frost.options.frost_file_present = 1`
  - `frost.options.wintRed = 1`
  - `frost.options.kfactor1/2/3 = 0.2/0.4/0.5`

## Replay Assertions (Ran)
- Freeze-active branch emitted deterministic runtime surfaces:
  - `frost.runtime_dfrost = 0.2`
  - `frost.runtime_dthaw = 0.0`
  - `frost.runtime_nft = 1.0`
  - `frost.runtime_ws_frz = 0.2`
  - `frost.runtime_infcap_frz = 0.1`
- Active-coupling run remained scheduler-successful.
- Inactive-coupling comparator run (`frost_file_present=0`) remained scheduler-successful.

## Command Evidence
```bash
cargo test --test clim06_frost_frozen_soil_kernel_contract
```
Result: pass (`4 passed`).
