# relproc02-implementation-and-test-evidence

Status: complete  
Evidence mode: Ran

## Implementation Changes

Updated:
- `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
  - Added `release sidecar` subcommand parsing and execution.
  - Added helper parsing for `--role`.
  - Added help text for new command.
  - Added command-path unit tests.
- `crates/openwepp-runner/src/errors.rs`
  - Added `RunnerError::ReleaseMetadata` for typed propagation of
    `ReleaseMetadataError`.
- `docs/contracts/openwepp-runner-contract.md`
- `docs/contracts/openwepp-binary-release-contract.md`
- `docs/governance/openwepp-release-procedure-draft.md`

## Commands Run

```bash
cargo fmt --all
cargo clippy -p openwepp-runner --all-targets -- -D warnings
cargo test -p openwepp-runner
markdown-doc lint --path docs/contracts/openwepp-runner-contract.md --format plain
markdown-doc lint --path docs/contracts/openwepp-binary-release-contract.md --format plain
markdown-doc lint --path docs/governance/openwepp-release-procedure-draft.md --format plain
markdown-doc lint --path docs/work-packages/README.md --format plain
markdown-doc lint --path docs/work-packages/20260529-relproc02-runner-sidecar-emission-cli-001 --format plain
tmpdir=$(mktemp -d)
printf 'fixture-binary' > "${tmpdir}/openwepp_260529"
cargo run -p openwepp-runner --bin open_wepp_runner -- release sidecar --binary "${tmpdir}/openwepp_260529" --role watershed
```

## Observed Results

- `cargo clippy -p openwepp-runner --all-targets -- -D warnings`: passed.
- `cargo test -p openwepp-runner`: passed (`28` tests total across unit +
  integration surfaces for this crate package).
- Sidecar command execution produced:
  - `sidecar=/tmp/.../openwepp_260529.json`
  - `schema=openwepp-binary-release-metadata-v1`
  - `binary_role=watershed`
- All listed markdown lint commands passed with zero errors/warnings.
