# CQR06 CRAP Before

Evidence class: Ran

Command:

```bash
cargo crap --workspace --lcov docs/work-packages/20260615-cqr06-lateral-drainage-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr06-lateral-drainage-complexity-001/artifacts/crap_before.json
```

Exit code: `0`

Target-file rows:

| Function | Coverage | CRAP |
| --- | ---: | ---: |
| `Wb11HydrologyKernel::run_lateral_transfer` | `77.12121212121212` | `300.2455501433063` |
| `Wb11HydrologyKernel::run_drainage` | `65.02590673575129` | `239.22631966890793` |
| `Wb11HydrologyKernel::wb14_load_top_two_layer_ksatadj_metrics` | `40.909090909090914` | `202.523384673178` |
| `Wb11HydrologyKernel::resolve_wb14_effective_soil_conductivity` | `50.66666666666667` | `114.52838399999999` |
| `Wb11HydrologyKernel::wb14_ksatadj_flag` | `61.904761904761905` | `4.884569700896232` |

Max target CRAP before: `300.2455501433063`.

Note: `cargo crap` emitted the existing workspace warning that `124` source
files had no matching LCOV entry. The target file had a matching LCOV entry and
target rows were emitted.
