# CLIM06 Frost Runtime Authority and Guard Map

Status: `completed`
Evidence mode: `Static + Ran`

## Authority Map
- Runtime seam authority: `SC-INFILE-FROST-001` parsed frost controls project to `frost.options.*`.
- Runtime behavior authority: CLIM06 addenda in `SC-CLIMATE-001`, `SC-SNOWFREEZE-001`, `SC-SOIL-001`.
- WB14 coupling authority: CLIM06 addenda in `SC-WATBAL-001` and `SC-RUNOFFPART-001`.

## Active-Coupling Gate
- Active when `frost.options.frost_file_present = 1` and `frost.options.wintRed = 1`.
- Freeze/thaw branch selector: `Tmin <= 0 degC` (freeze-active), `Tmin > 0 degC` (thaw branch).

## Runtime Outputs Published
- `frost.runtime_dfrost`
- `frost.runtime_dthaw`
- `frost.runtime_nft`
- `frost.runtime_ws_frz`
- `frost.runtime_infcap_frz`

## Guard Surface Mapping
| Condition | Runtime seam / kernel code path | Typed code |
|---|---|---|
| Missing required active frost symbol | `Wb11HydrologyKernel::require_state_scalar` in runoff reconciliation | `HKERNEL-WB14-RUNOFF-E-001` |
| Non-finite active frost symbol | `Wb11HydrologyKernel::require_state_scalar` non-finite guard | `HKERNEL-WB14-RUNOFF-E-002` |
| Active frost domain violation | `Wb11HydrologyKernel::require_state_range` and explicit domain checks | `HKERNEL-WB14-RUNOFF-E-003` |
| Non-finite frost seam projection symbol | `validate_frost_control_finite` in runtime seam | `HS-RUNTIME-E-054` |
| Frost seam projection domain violation | `seed_hillslope_runtime_surface_from_frost` range checks | `HS-RUNTIME-E-055` |

## Ran Evidence
- `cargo test --test clim06_frost_frozen_soil_kernel_contract` -> pass (`4 passed`).
- `cargo test --test parser_runtime_seam_integration` -> pass (`45 passed`, includes frost seam vectors).
