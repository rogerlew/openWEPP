# CRAP After

Evidence label: Static/Ran.

Status: `EXECUTED`

Source:
`/tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted-crap.json`

Command:

- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted-crap.json`

Command result:

- PASS, exit `0`.
- Warning: `303` source files had no matching LCOV entry because the LCOV was a
  targeted watershed-orchestrator measurement; target rows are valid for this
  Phase D targeted equivalent.

Artifact:

- bytes: `2682021`
- sha256:
  `67aa4decc562d9dcc337ff7da42e0224641564d5491dd6824e533ec1f1c34c64`

Summary:

- Deduplicated target rows: `24`
- Rows above CRAP `30`: `0`
- Max target CRAP: `16.153567674676058`

Top target rows:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `Ws10ChannelImpoundmentKernel::ws23_detach_case4_iterative_closure` | 394 | 16.0 | 91.56626506024097 | 16.153567674676058 |
| `Ws10ChannelImpoundmentKernel::ws27_case4_enddet_bracket_closure` | 781 | 14.0 | 100.0 | 14.0 |
| `Ws10ChannelImpoundmentKernel::ws26_dcap_expanding_width_outcome` | 322 | 9.0 | 75.40983606557377 | 10.204395962657667 |
| `Ws10ChannelImpoundmentKernel::ws26_dcap_midlayer_step` | 215 | 10.0 | 100.0 | 10.0 |
| `Ws10ChannelImpoundmentKernel::ws22_table_column2_to_column1` | 69 | 8.0 | 100.0 | 8.0 |
| `Ws10ChannelImpoundmentKernel::ws23_validate_detach_input` | 491 | 8.0 | 100.0 | 8.0 |
