# CRAP After

Evidence label: Static/Ran.

Status: `PROVISIONAL-ROLLED-BACK-NOT-CLOSURE`

Provisional focused command:

- `cargo llvm-cov clean --workspace && cargo llvm-cov -p openwepp-watershed-orchestrator --lcov --output-path /tmp/openwepp-cqr-nightly-04-ws20-focused.lcov && cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-04-ws20-focused.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-04-ws20-focused-crap.json` - exit `0`.

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs`

Summary:

- Unique target functions reported: `42`.
- Functions above CRAP `30`: `0`.
- Max target CRAP: `30.0`.

Highest target rows after:

| Function | CC | Coverage | CRAP |
|---|---:|---:|---:|
| `Ws10ChannelImpoundmentKernel::ws20_case3_xdbeg_value` | `5.0` | `0.0` | `30.0` |
| `Ws10ChannelImpoundmentKernel::ws20_route_case34_segment` | `5.0` | `0.0` | `30.0` |
| `Ws10ChannelImpoundmentKernel::ws20_try_case4_iterative_closure` | `5.0` | `0.0` | `30.0` |
| `Ws10ChannelImpoundmentKernel::ws20_finish_case4_enddet` | `4.0` | `0.0` | `20.0` |
| `Ws10ChannelImpoundmentKernel::ws20_case12_class_update` | `17.0` | `89.33333333333333` | `17.350738962962964` |

Baseline-to-provisional-after:

- Before: `10` functions above `30`, max CRAP `306.0`, total excess `892.0`.
- After: `0` functions above `30`, max CRAP `30.0`, total excess `0.0`.

Hold disposition:

- This after measurement was produced before review and rollback.
- It is retained as evidence that CRAP could be reduced mechanically, but it is
  not closure evidence because accepted review findings blocked package
  completion and the target file was restored to the scaffold state.
