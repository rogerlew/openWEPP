# CQR06 CRAP After

Evidence class: Ran

Command:

```bash
cargo crap --workspace --lcov docs/work-packages/20260615-cqr06-lateral-drainage-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr06-lateral-drainage-complexity-001/artifacts/crap_after.json
```

Exit code: `0`

Target result:

- Max target CRAP after: `26.541362973760947`.
- Closure target `<= 30`: passed.

Top after rows:

| Function | Coverage | CRAP |
| --- | ---: | ---: |
| `Wb11HydrologyKernel::wb19_lateral_transfer_inputs` | `70.23809523809523` | `26.541362973760947` |
| `Wb11HydrologyKernel::wb19_drainage_inputs` | `78.3132530120482` | `19.947685064543265` |
| `Wb11HydrologyKernel::wb19_lateral_layer_parameters` | `70.12987012987013` | `19.22356981861114` |
| `Wb11HydrologyKernel::wb19_lateral_lane_config` | `60.0` | `14.184000000000001` |
| `Wb11HydrologyKernel::wb14_effective_ks_9002_exponent` | `44.73684210526316` | `12.075885697623558` |

Note: `cargo crap` emitted the existing workspace warning that `124` source
files had no matching LCOV entry. The target file had a matching LCOV entry and
target rows were emitted.
