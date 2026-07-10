# CRAP Before

Evidence label: Static.

Status: `EXECUTED`

Source: `/tmp/openwepp-cqr-nightly-crap.json`

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs`

Summary:

- Deduplicated rows: `24`
- Rows above CRAP `30`: `4`
- Max CRAP: `272.0`
- Total excess over `30`: `386`

Rows above `30`:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `Ws10ChannelImpoundmentKernel::ws23_detach_case4_iterative_closure` | 394 | 16.0 | 0.0 | 272.0 |
| `Ws10ChannelImpoundmentKernel::ws26_dcap_expanding_width_outcome` | 322 | 9.0 | 0.0 | 90.0 |
| `Ws10ChannelImpoundmentKernel::ws22_table_column2_to_column1` | 69 | 8.0 | 0.0 | 72.0 |
| `Ws10ChannelImpoundmentKernel::ws23_validate_detach_input` | 491 | 8.0 | 0.0 | 72.0 |

Additional zero-covered rows at or below `30` to consider during
characterization:

- `ws20_fall_velocity_ft_s`, CRAP `30`
- `ws26_dcap_low_width_shear_outcome`, CRAP `30`
- `ws23_initial_detach_working`, CRAP `30`
- `ws23_final_detach_outcome`, CRAP `20`
- `ws23_validate_detach_sums`, CRAP `20`
- `ws26_dcap_width_step`, CRAP `20`
