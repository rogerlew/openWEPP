# CRAP Before

Evidence label: Static/Ran.

Status: `SCAFFOLDED`

Target module:
`crates/openwepp-runner/src/errors.rs`

Baseline command provenance:

- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-crap.json` -
  exit `0`.

Deduplicated target rows above CRAP `30`:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `HillslopeCliError::fmt` | `386` | `20.0` | `24.390243902439025` | `192.89940656693898` |
| `HillslopeCliError::code` | `359` | `20.0` | `27.27272727272727` | `173.86927122464314` |
| `ReleaseLintError::fmt` | `133` | `9.0` | `0.0` | `90.0` |
| `ReleaseMetadataError::code` | `35` | `6.0` | `0.0` | `42.0` |
| `ReleaseMetadataError::fmt` | `47` | `6.0` | `0.0` | `42.0` |
| `HillslopeCliError::source` | `526` | `6.0` | `0.0` | `42.0` |

Summary:

- Deduplicated rows above `30`: `6`.
- Deduplicated total excess above `30`: `402.76867779158215`.
- Max CRAP: `192.89940656693898`.
