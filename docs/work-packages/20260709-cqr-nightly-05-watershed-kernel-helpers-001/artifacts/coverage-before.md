# Coverage Before

Evidence label: Static/Ran.

Status: `BASELINE-RECORDED`

Baseline command:

- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly.lcov` - exit `0`.

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs`

LCOV line coverage:

- `LF:484`
- `LH:262`
- Line coverage: `54.13223140495868%`

Region coverage:

- `NOT AVAILABLE` from LCOV.

Function coverage from CRAP baseline:

| Function | Line | Coverage |
|---|---:|---:|
| `Ws10ChannelImpoundmentKernel::missing_required` | `2` | `0.0` |
| `Ws10ChannelImpoundmentKernel::non_finite` | `13` | `100.0` |
| `Ws10ChannelImpoundmentKernel::domain_violation` | `26` | `100.0` |
| `Ws10ChannelImpoundmentKernel::impoundment_outflow_at_stage` | `40` | `35.15151515151515` |
| `Ws10ChannelImpoundmentKernel::impoundment_area_at_stage` | `230` | `66.66666666666666` |
| `Ws10ChannelImpoundmentKernel::impoundment_continuity_rate` | `246` | `51.21951219512195` |
| `Ws10ChannelImpoundmentKernel::impoundment_rk4_step` | `290` | `73.58490566037736` |
| `Ws10ChannelImpoundmentKernel::crosses_threshold` | `346` | `100.0` |
| `Ws10ChannelImpoundmentKernel::impoundment_crosses_regime_transition` | `350` | `100.0` |
| `Ws10ChannelImpoundmentKernel::integrate_impoundment_stage_with_adaptive_retry` | `364` | `55.44554455445545` |
| `Ws10ChannelImpoundmentKernel::route_impoundment_stage_over_duration` | `477` | `64.40677966101694` |
