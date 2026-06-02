# HPHYS0245 Gate Results

Status: completed
Evidence mode: Ran

## Rust Gates
- `cargo fmt --check`: pass.
- `cargo clippy -p openwepp-runner --all-targets -- -D warnings`: pass.
- `cargo test -p openwepp-runner`: pass.
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`: pass.

## Documentation and Diff Gates
- `wctl doc-lint --path docs/work-packages/README.md`: pass; tool reported
  `1 files validated, 0 errors, 0 warnings`.
- `wctl doc-lint` per HPHYS0245 package Markdown file: pass; tool reported no
  errors or warnings but `0 files validated` for each package-local file.
- `git diff --check`: pass.

## Telemetry Gates
- H1 telemetry run: pass; return code `0`; `480` trace rows.
- H7 telemetry run: pass; return code `0`; `480` trace rows.
- H39 telemetry run: pass; return code `0`; `480` trace rows.
- Default-disabled smoke run: pass; no `hphys0245` sidecar generated.

## Not Run
- `cargo clippy --workspace --all-targets -- -D warnings`: not run; package
  changed runner diagnostics only and the targeted runner clippy gate passed.
- `cargo test --workspace`: not run; targeted runner package tests passed.
- `cargo deny check`: not run; no dependency, license, advisory, or source
  configuration changes were made.
