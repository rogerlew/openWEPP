# Coverage Closure

Evidence label: Static/Ran.

Status: `EXECUTED-PASS`

Target module:
`crates/openwepp-runner/src/errors.rs`

## ADR-0021 Tier

Static:

- Tier: `glue`.
- Rationale: runner error taxonomy/display/source plumbing, not kernel math or
  conservation-law code.

## Threshold Status

Ran:

| ADR-0021 Gate | Required | Observed | Status |
|---|---:|---:|---|
| Line coverage | `>=85%` | `99.625468164794%` | PASS |
| Region coverage | `>=85%` | `98.73417721518987%` | PASS |
| CRAP per eligible function | `<=30` | max `20.0` | PASS |
| Per-function floor | `>=75%` or disposition | all target functions `>=95.45454545454545%` | PASS |

Coverage evidence:

- `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted-llvmcov.json`
- `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted.lcov`
- `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted-crap.json`

Metric file provenance:

| Path | Bytes | SHA-256 |
|---|---:|---|
| `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted.lcov` | `2065951` | `cb7ae88ba17dcca138c89872ed74749911f902d347be7d0d144d953ce83baa72` |
| `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted-llvmcov.json` | `8909904` | `7b09382aa2c85bd5c980b4518eb48da86fec75f7e997cf73237f871add562ed4` |
| `/tmp/openwepp-cqr-nightly-08-runner-errors-targeted-crap.json` | `2671921` | `2163d74f2e21dd8cd94ca04b8e59e9a0e0894543422480b1393e49e242a5473b` |

## Obligation Binding

Static/Ran:

| Obligation surface | Test binding | Covered variants |
|---|---|---|
| Stable release metadata error codes, display text, and source ownership | `runner_release_metadata_errors_preserve_codes_display_and_sources` | `Io`, `JsonSerialize`, `JsonParse`, `MissingField`, `InvalidField` |
| Stable release lint error codes, display text, and source ownership | `runner_release_lint_errors_preserve_codes_display_and_sources` | `DirectoryRead`, `InvalidBinaryName`, `MissingSidecar`, `SidecarInvalid`, `SidecarRoleMismatch`, `SidecarBinaryNameMismatch`, `HbpPairMismatch`, `NoReleaseCandidates` |
| Stable top-level runner error codes, display text, and source ownership | `runner_errors_preserve_codes_display_and_sources` | `MissingArgument`, `HillslopeBinaryMissing`, `LaunchFailure`, `NonZeroExit`, `ReleaseLint`, `ReleaseMetadata` |
| Stable hillslope CLI path/core error codes, display text, and source ownership | `hillslope_cli_path_and_core_errors_preserve_codes_display_and_sources` | `MissingArgument`, `RunDirectoryMissing`, `RunFileMissing`, `OutputDirectoryCreate`, `CoreInputMissing`, `CoreInputAmbiguous` |
| Stable hillslope CLI sidecar/runtime error codes, display text, and source ownership | `hillslope_cli_sidecar_and_runtime_errors_preserve_codes_display_and_sources` | `SidecarContractInvalid`, `SidecarAdapter`, `SidecarBindingMissing`, `ParseFailure`, `RuntimeSurfaceFailure`, `OfeTopologyMismatch` |
| Stable hillslope CLI output error codes, display text, and source ownership | `hillslope_cli_output_errors_preserve_codes_display_and_sources` | `OutputWrite`, `MissingRequiredOutput` |
| Stable hillslope CLI metadata/manifest/IO error codes, display text, and source ownership | `hillslope_cli_metadata_manifest_and_io_errors_preserve_codes_display_and_sources` | `ReleaseMetadata`, `ManifestSerialize`, `ManifestWrite`, `Io`, `TimeFormat` |

Disposition:

- ADR-0021 module thresholds pass.
- The characterization tests bind existing runner error behavior and do not
  create new error taxonomy or CLI contract authority.
