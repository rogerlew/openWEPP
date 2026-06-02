# Pre-Implementation Contract Gate

Status: complete

Evidence mode: ran

Ran: targeted contract-derived tests before production code edits.

## Commands

- `cargo test -p openwepp-hillslope-orchestrator hphys0251_management_projection_preserves_crop_pltol -- --nocapture`
- `cargo test --test wb17_et_physics_kernel_contract hphys0251_ -- --nocapture`

## Results

- Runtime projection: failed as expected with missing
  `pl_growth_slot_0001_crop_0001_pltol` / primary `pltol=0.37` projection.
- WB17 root uptake normalization: failed as expected with
  `HKERNEL-WB17-SWU-E-003` for raw `pltol=0.0`, proving the current runtime
  rejects baseline-authorized legacy normalization inputs.
- WB17 layer uptake traces: failed as expected because `UPi_0001` and sibling
  layer uptake fluxes are not published.

## Logs

- `docs/work-packages/20260602-hphys0251-swu-root-uptake-stress-closure-001/artifacts/gate-logs/pre_impl_runtime_projection.log`
- `docs/work-packages/20260602-hphys0251-swu-root-uptake-stress-closure-001/artifacts/gate-logs/pre_impl_wb17_root_uptake.log`
- `docs/work-packages/20260602-hphys0251-swu-root-uptake-stress-closure-001/artifacts/gate-logs/pre_impl_status.txt`
