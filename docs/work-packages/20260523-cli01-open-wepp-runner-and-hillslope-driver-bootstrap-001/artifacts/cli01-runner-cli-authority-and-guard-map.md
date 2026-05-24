# CLI01 Runner/CLI Authority and Guard Map

Status: complete
Evidence mode: Static

## Static

| Surface | Authority | Guard / Hard-Fail Requirement |
|---|---|---|
| runner launch ownership | `docs/contracts/openwepp-runner-contract.md` | `open_wepp_runner` is sole launcher boundary; explicit selector required; no shell interpolation; no fallback |
| hillslope executable role | `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md` | `openwepp-cli-hill` role identity; required include outputs `H5.wat.dat` + `H5.plot.dat` |
| sidecar discovery | `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md` + `docs/specifications/science-contracts/legacy-sidecar-bridge-contract.md` | blind run-directory discovery only; strict unknown sidecar reject (`LSB-E-009`); compat unknown warning (`LSB-W-002`); required sidecar missing fail (`LSB-E-007`) |
| run manifest | `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md` | schema id required; explicit argv capture; sorted checksum maps; required output checksums |
| release sidecar | `docs/contracts/openwepp-binary-release-contract.md` | `<binary>.json` required; schema id required; required field validation blocking |
| runner release lint | `docs/contracts/openwepp-runner-contract.md` + `docs/contracts/openwepp-binary-release-contract.md` | filename regex + sidecar existence + schema field checks + role pairing checks |

## Ran
- None in this phase.
