# CRAP Before

Evidence label: Static/Ran.

Status: `BASELINE-RECORDED`

Baseline command:

- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-crap.json` - exit `0`.

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs`

Summary:

- Unique target functions reported: `11`.
- Functions above CRAP `30`: `2`.
- Max target CRAP: `547.2389753179175`.
- Total excess above `30`: `525.8809022887736`.

Rows above CRAP `30`:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `Ws10ChannelImpoundmentKernel::impoundment_outflow_at_stage` | `40` | `43.0` | `35.15151515151515` | `547.2389753179175` |
| `Ws10ChannelImpoundmentKernel::integrate_impoundment_stage_with_adaptive_retry` | `364` | `16.0` | `55.44554455445545` | `38.64192697085608` |
