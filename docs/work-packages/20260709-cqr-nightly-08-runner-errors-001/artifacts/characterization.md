# Characterization

Evidence label: Static/Ran.

Status: `EXECUTED`

## Behavior Oracle

Static:

- Target module:
  `crates/openwepp-runner/src/errors.rs`.
- Characterization lives in:
  `tests/integration/cli01_runner_contract_derived_tests.rs`.
- The package preserves public runner error APIs:
  `ReleaseMetadataError`, `ReleaseLintError`, `RunnerError`, and
  `HillslopeCliError` remain public error enums re-exported by
  `openwepp_runner`.

## Added Characterization

Static:

- `runner_release_metadata_errors_preserve_codes_display_and_sources` covers
  every `ReleaseMetadataError` variant for stable `code()`, display text, and
  source chaining.
- `runner_release_lint_errors_preserve_codes_display_and_sources` covers every
  `ReleaseLintError` variant for stable `code()`, display text, and source
  chaining.
- `runner_errors_preserve_codes_display_and_sources` covers every
  `RunnerError` variant for stable `code()`, display text, and source chaining.
- `hillslope_cli_path_and_core_errors_preserve_codes_display_and_sources`,
  `hillslope_cli_sidecar_and_runtime_errors_preserve_codes_display_and_sources`,
  `hillslope_cli_output_errors_preserve_codes_display_and_sources`, and
  `hillslope_cli_metadata_manifest_and_io_errors_preserve_codes_display_and_sources`
  cover every `HillslopeCliError` variant for stable `code()`, display text,
  and source chaining, including topology mismatch formatting.

## Focused Runs

Ran:

| Command | Result | Evidence |
|---|---|---|
| `cargo nextest run --test cli01_runner_contract_derived_tests` | PASS, exit `0`; `13` tests passed | parent shell, 2026-07-09 |
| `cargo fmt --check` | PASS, exit `0` | parent shell, 2026-07-09 |
| `cargo clippy -p openwepp-runner --all-targets -- -D warnings` | PASS, exit `0` | parent shell, 2026-07-09 |
| `cargo clippy --test cli01_runner_contract_derived_tests -- -D warnings` | PASS, exit `0` | parent shell, 2026-07-09 |

Disposition:

- Characterization closes the target risk because all target functions already
  have cyclomatic complexity `<= 30`; the baseline CRAP excess came from low
  coverage rather than irreducible high CC.
- The tests bind existing runner error codes, display fragments, and source
  ownership only; they do not introduce new error taxonomy or message authority.
