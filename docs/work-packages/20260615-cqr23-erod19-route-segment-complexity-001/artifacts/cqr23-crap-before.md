# CQR23 CRAP Before

Status: complete.

Ran:
`cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr23-erod19-route-segment-complexity-001/artifacts/lcov_before.info`

Ran:
`cargo crap --workspace --lcov docs/work-packages/20260615-cqr23-erod19-route-segment-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr23-erod19-route-segment-complexity-001/artifacts/crap_before.json`

Ran: `cargo crap` emitted the known warning that `126` test/source files had no
matching LCOV report entry. The target production file had matching LCOV data.

Ran: target-file before coverage:

- Lines: `476/647` (`73.57%`)
- Functions: `15/17` (`88.24%`)

Ran: target identity and before metric:

- `Wb11HydrologyKernel::run_erod19_route_segment_migration`
- File:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`
- Line: `446`
- CC: `79.0`
- Coverage: `64.76868327402136`
- CRAP: `351.9234211799049`

Ran: pre-existing out-of-scope same-file row:

- `Wb11HydrologyKernel::erod19_depend`, line `320`, CC `28.0`, coverage
  `57.54716981132076`, CRAP `87.98408081839372`
