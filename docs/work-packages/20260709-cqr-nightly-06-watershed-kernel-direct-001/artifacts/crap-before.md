# CRAP Before

Evidence label: Static/Ran.

Status: `BASELINE-RECORDED`

Baseline command:

- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-crap.json` - exit `0`.

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`

Summary:

- Unique target functions reported: `28`.
- Deduplicated functions above CRAP `30`: `7`.
- Max target CRAP: `296.51689035535117`.
- Deduplicated total excess above `30`: `504.9508413968741`.
- Raw CRAP entries include duplicated monomorphization rows for this include
  file; closure uses deduplicated source function rows.

Rows above CRAP `30`:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `Ws10ChannelImpoundmentKernel::assemble_direct_incoming_sediment_load_and_capacity` | `1262` | `68.0` | `63.30275229357798` | `296.51689035535117` |
| `Ws10ChannelImpoundmentKernel::read_direct_channel_sediment_payload` | `1189` | `11.0` | `0.0` | `132.0` |
| `Ws10ChannelImpoundmentKernel::run_direct_channel_node` | `45` | `73.0` | `85.15625` | `90.42916631698608` |
| `Ws10ChannelImpoundmentKernel::direct_ws20_channel_profile` | `1642` | `8.0` | `0.0` | `72.0` |
| `Ws10ChannelImpoundmentKernel::direct_ws20_crfrac` | `1686` | `6.0` | `0.0` | `42.0` |
| `Ws10ChannelImpoundmentKernel::assemble_direct_incoming_peak_partition` | `612` | `26.0` | `71.42857142857143` | `41.7667638483965` |
| `Ws10ChannelImpoundmentKernel::run_direct_impoundment_node` | `364` | `24.0` | `69.56521739130434` | `40.23802087614038` |
