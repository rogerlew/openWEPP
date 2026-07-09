# CRAP Before

Evidence label: Static/Ran.

Status: `SCAFFOLDED`

Target module:
`crates/openwepp-input-contract/src/parsers/management.rs`

Baseline command provenance:

- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-crap.json` -
  exit `0`.

Deduplicated target rows above CRAP `30`:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `parse_operation_section` | `1663` | `37.0` | `50.442477876106196` | `203.62194460172833` |
| `parse_contour_section` | `1991` | `17.0` | `16.666666666666664` | `184.2453703703704` |
| `ManagementParseError::fmt` | `533` | `15.0` | `19.17808219178082` | `133.78728950148707` |
| `parse_management_from_str` | `1245` | `45.0` | `83.56164383561644` | `53.99497965384546` |
| `parse_initial_section` | `1794` | `24.0` | `67.08860759493672` | `44.53341824886061` |
| `yaml_yearly_extension_to_management` | `1134` | `6.0` | `0.0` | `42.0` |

Summary:

- Deduplicated rows above `30`: `6`.
- Deduplicated total excess above `30`: `482.1830023762918`.
- Max CRAP: `203.62194460172833`.
