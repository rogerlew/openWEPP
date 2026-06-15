# CQR17 CRAP Before

Status: closed.

Ran: before CRAP command:

```text
cargo crap --workspace \
  --lcov docs/work-packages/20260615-cqr17-hydrology-erod19-complexity-001/artifacts/lcov_before.info \
  --min 0 --format json \
  --output docs/work-packages/20260615-cqr17-hydrology-erod19-complexity-001/artifacts/crap_before.json
```

Result: exit code `0`; JSON saved to `crap_before.json`.

Live before rows for
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`:

```text
Wb11HydrologyKernel::erod19_xcrit_classification       line 38  CC 37.0 Cov 32.098765432098766 CRAP 465.5844995022966
Wb11HydrologyKernel::run_erod19_route_segment_migration line 274 CC 79.0 Cov 64.76868327402136 CRAP 351.9234211799049
Wb11HydrologyKernel::erod19_depend                     line 148 CC 28.0 Cov 57.54716981132076 CRAP 87.98408081839372
Wb11HydrologyKernel::erod19_root                       line 16  CC 5.0  Cov 0.0 CRAP 30.0
Wb11HydrologyKernel::erod19_shear                      line 5   CC 3.0  Cov 81.81818181818183 CRAP 3.0540946656649135
Wb11HydrologyKernel::erod19_depc                       line 129 CC 2.0  Cov 100.0 CRAP 2.0
```

Target identity: `Wb11HydrologyKernel::erod19_xcrit_classification`.

Target baseline: CRAP `465.5844995022966`, CC `37.0`, coverage
`32.098765432098766`.
