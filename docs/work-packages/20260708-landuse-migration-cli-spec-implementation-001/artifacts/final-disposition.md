# Final Disposition

Status: `EXECUTED-COMPLETE`.

The work package implemented the Rust landuse migration library/CLI and
finalized the public CLI specification.

## Delivered

- `crates/openwepp-landuse-migrate` library and `openwepp-landuse-migrate`
  binary.
- Frozen legacy cropland flat `.man` to coefficient-complete `ow-lanuse-1`
  management YAML migration requiring disturbed-class authority.
- Flat `ow-lanuse-1` to canonical YAML bridge preserving explicit routing
  coefficients.
- Native YAML `ow-lanuse-1` to `latest` pass-through while `latest` resolves to
  `ow-lanuse-1`.
- `--args-for-migration-to`, `--validate`, `--dry-run`, `--report`, and
  report-format support.
- Lowercase `.yaml` producer enforcement and default `.man.yaml` naming.
- Embedded/versioned Disturbed route-coefficient table with checksum.
- Schema/runtime consumer proof for migrated YAML route-coefficient projection.

## Verification

Ran:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- `cargo test -p openwepp-landuse-migrate`
- `cargo test --test landuse_migration_cli_contract`
- `cargo package -p openwepp-management-schema --allow-dirty`
- `cargo package -p openwepp-input-contract --allow-dirty --list`
- `cargo package -p openwepp-landuse-migrate --allow-dirty --list`
- scoped `markdown-doc lint`
- `git diff --check`

Result: required implementation gates passed. Full nextest final result:
1459 tests run, 1459 passed, 3 skipped.

## Publish Handoff

No crates were published. Full package verification for dependent openWEPP
crates must be rerun after publishing dependencies in this order:

1. `openwepp-management-schema`
2. `openwepp-input-contract`
3. `openwepp-landuse-migrate`
