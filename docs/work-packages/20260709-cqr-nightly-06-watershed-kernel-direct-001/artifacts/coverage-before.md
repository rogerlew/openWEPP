# Coverage Before

Evidence label: Static/Ran.

Status: `BASELINE-RECORDED`

Baseline command:

- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly.lcov` - exit `0`.

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`

LCOV line coverage:

- `LF:1506`
- `LH:1042`
- Line coverage: `69.18990703851262%`

Region coverage:

- `NOT AVAILABLE` from LCOV.

Function coverage from CRAP baseline:

| Function | Line | Coverage |
|---|---:|---:|
| `Ws10ChannelImpoundmentKernel::assemble_direct_incoming_sediment_load_and_capacity` | `1262` | `63.30275229357798` |
| `Ws10ChannelImpoundmentKernel::read_direct_channel_sediment_payload` | `1189` | `0.0` |
| `Ws10ChannelImpoundmentKernel::run_direct_channel_node` | `45` | `85.15625` |
| `Ws10ChannelImpoundmentKernel::direct_ws20_channel_profile` | `1642` | `0.0` |
| `Ws10ChannelImpoundmentKernel::direct_ws20_crfrac` | `1686` | `0.0` |
| `Ws10ChannelImpoundmentKernel::assemble_direct_incoming_peak_partition` | `612` | `71.42857142857143` |
| `Ws10ChannelImpoundmentKernel::run_direct_impoundment_node` | `364` | `69.56521739130434` |
