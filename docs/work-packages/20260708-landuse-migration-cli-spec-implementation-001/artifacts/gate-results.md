# Gate Results

Status: `EXECUTED-COMPLETE`.

| Gate | Status | Evidence |
|---|---|---|
| YAML authorization dependency | PASS | `20260708-openwepp-management-yaml-canonical-authorization-001` is `EXECUTED-COMPLETE`. |
| CLI spec finalized | PASS | `landuse-migration-cli.spec.md` is `draft-implemented`, version `1.0.0`, with class-map, args-file, report, output, and publish-order schemas resolved. |
| Rust crate and binary | PASS | Added `crates/openwepp-landuse-migrate` library and `openwepp-landuse-migrate` binary. |
| Disturbed table embedded | PASS | `src/disturbed.rs` embeds the table and checksum strategy; current checksum `sha256:55b0d88fe89a968a4d19c80b55f766a16c0c92c84ba97e97366ea664ac3cd051`. |
| Discovery and validation | PASS | CLI tests cover `--args-for-migration-to`, `--validate`, missing/unknown/partial/conflicting authority. |
| Migration behavior | PASS | CLI tests cover legacy cropland migration, default `.man.yaml`, flat native bridge, native YAML latest pass-through, dry-run/report, overwrite failure, and producer extension rejection. |
| Runtime consumer proof | PASS | `cargo test --test landuse_migration_cli_contract` proves migrated YAML is parsed and all five route coefficients project to PL surfaces. |
| Crates.io readiness check | PASS | Metadata present; no WEPPpy/network runtime dependency; schema crate packages/verifies; parser/migrator package file lists produced; publish order recorded. |
| Review and disposition | PASS | `review-rust-code-reviewer.md`, `review-qa-reviewer.md`, and `disposition.md` record findings and fixes. |
| Line-count governance | PASS | No new `.rs` file is at or above 2000 lines; largest new source is `src/lib.rs` at 1266 lines. |
| `cargo fmt --check` | PASS | Ran after final edits; no output. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran after final edits; passed. |
| `cargo nextest run --workspace --profile full` | PASS | 1459 tests run: 1459 passed, 3 skipped. |
| `cargo deny check` | PASS | advisories ok, bans ok, licenses ok, sources ok. |
| Markdown lint | PASS | 17 files validated, 0 errors, 0 warnings. |
| `git diff --check` | PASS | No output. |

## Publish-Order Note

Direct `cargo package` verification for `openwepp-input-contract` and
`openwepp-landuse-migrate` cannot complete against crates.io until their
openWEPP dependencies are published in order. This package did not publish
crates. The release handoff is:

1. publish and verify `openwepp-management-schema`;
2. publish and verify `openwepp-input-contract`;
3. publish and verify `openwepp-landuse-migrate`.
