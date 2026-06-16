# CQR23 CRAP After

Status: complete.

Ran:
`cargo crap --workspace --lcov docs/work-packages/20260615-cqr23-erod19-route-segment-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr23-erod19-route-segment-complexity-001/artifacts/crap_after.json`

Ran: `cargo crap` emitted the known warning that `126` test/source files had no
matching LCOV report entry. The target production file had matching LCOV data.

Ran: target and high rows after:

- `Wb11HydrologyKernel::run_erod19_route_segment_migration`, line `1084`, CC
  `9.0`, coverage `96.15384615384616`, CRAP `9.00460855712335`
- `Wb11HydrologyKernel::erod19_route_topology`, line `555`, CC `7.0`,
  coverage `45.83333333333333`, CRAP `14.787398726851855`
- `Wb11HydrologyKernel::erod19_route_drivers`, line `807`, CC `12.0`,
  coverage `100.0`, CRAP `12.0`
- `Wb11HydrologyKernel::erod19_route_theta_from_erosion_inputs`, line `653`,
  CC `11.0`, coverage `93.75`, CRAP `11.029541015625`
- `Wb11HydrologyKernel::erod19_route_segment_scalars`, line `607`, CC `11.0`,
  coverage `100.0`, CRAP `11.0`
- `Wb11HydrologyKernel::erod19_route_phi`, line `689`, CC `9.0`, coverage
  `95.83333333333334`, CRAP `9.005859375`
- `Wb11HydrologyKernel::erod19_route_phi_from_erosion_inputs`, line `727`, CC
  `7.0`, coverage `100.0`, CRAP `7.0`
- `Wb11HydrologyKernel::erod19_route_tauc`, line `745`, CC `7.0`, coverage
  `100.0`, CRAP `7.0`
- `Wb11HydrologyKernel::erod19_route_tauc_from_shear_inputs`, line `769`, CC
  `7.0`, coverage `100.0`, CRAP `7.0`

Warning: pre-existing out-of-scope `Wb11HydrologyKernel::erod19_depend` remains
line `430`, CC `28.0`, coverage `57.54716981132076`, CRAP
`87.98408081839372`. It predates CQR23 and was not introduced as a new helper.

Closure: CQR23 target and every newly extracted helper are CRAP `<= 30`.
