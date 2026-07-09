# Worker Handoff

Status: implementation complete; publish-order handoff remains.

## Completed

- Added `crates/openwepp-landuse-migrate` as a publishable library/binary crate.
- Embedded the Disturbed route-coefficient table in Rust source.
- Implemented `--args-for-migration-to`, `--validate`, migration, dry-run, and
  reports.
- Implemented legacy cropland to coefficient-complete `ow-lanuse-1` YAML with
  required disturbed-class authority.
- Implemented flat `ow-lanuse-1` to YAML and native YAML `latest`
  pass-through.
- Enforced lowercase `.yaml` producer output and default `.man.yaml` naming.
- Proved migrated YAML is read by the real management YAML parser and projects
  all five route coefficients into PL runtime surfaces.

## Publish Order

No crates were published by this package. Cargo package verification for crates
that depend on unpublished openWEPP crates cannot complete against the crates.io
index until the dependencies exist there.

Publish order:

1. `openwepp-management-schema`
2. `openwepp-input-contract`
3. `openwepp-landuse-migrate`

Evidence:

- `cargo package -p openwepp-management-schema --allow-dirty`: passed and
  verified.
- `cargo package -p openwepp-input-contract --allow-dirty --list`: package file
  list produced.
- `cargo package -p openwepp-landuse-migrate --allow-dirty --list`: package
  file list produced.
- Full package verification for `openwepp-input-contract` and
  `openwepp-landuse-migrate` is expected only after the preceding crates are in
  the registry.

## Follow-On

Before an actual crates.io release, publish the crates in the order above and
rerun `cargo package` without `--allow-dirty` for each crate from a clean
worktree. No sidecars, hidden coefficient inference, or native flat `.man`
writer should be added during that release step.
