# CRAP Before

Evidence label: Static/Ran.

Status: `BASELINE-RECORDED`

Source: `/tmp/openwepp-cqr-nightly-crap.json` from the live nightly baseline.

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs`

Summary:

- Unique target functions reported: `25`.
- Functions above CRAP `30`: `10`.
- Max CRAP: `306.0`.
- Total excess CRAP above `30`: `892.0`.

Rows above CRAP `30`:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `Ws10ChannelImpoundmentKernel::ws20_case12_class_update` | `781` | `17.0` | `0.0` | `306.0` |
| `Ws10ChannelImpoundmentKernel::ws20_segment_hydraulics` | `164` | `14.0` | `0.0` | `210.0` |
| `Ws10ChannelImpoundmentKernel::ws20_flow_partition` | `52` | `10.0` | `0.0` | `110.0` |
| `Ws10ChannelImpoundmentKernel::ws20_try_case12_transition` | `884` | `10.0` | `0.0` | `110.0` |
| `Ws10ChannelImpoundmentKernel::ws20_route_case12_segment_family_core` | `992` | `10.0` | `0.0` | `110.0` |
| `Ws10ChannelImpoundmentKernel::ws20_route_case4_segment` | `619` | `9.0` | `0.0` | `90.0` |
| `Ws10ChannelImpoundmentKernel::ws20_route_case34_segment` | `383` | `8.0` | `0.0` | `72.0` |
| `Ws10ChannelImpoundmentKernel::ws20_route_case12_segment` | `718` | `8.0` | `0.0` | `72.0` |
| `Ws10ChannelImpoundmentKernel::ws20_prepare_class_transport` | `102` | `7.0` | `0.0` | `56.0` |
| `Ws10ChannelImpoundmentKernel::ws20_transport_snapshot` | `266` | `7.0` | `0.0` | `56.0` |

Rows at or below CRAP `30` are not target rows unless needed for
characterization or helper extraction.
