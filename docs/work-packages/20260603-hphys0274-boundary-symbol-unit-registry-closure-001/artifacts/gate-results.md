# Gate Results

Status: completed
Evidence mode: ran

Static: HPHYS0274 validation used focused Rust contract gates and the package
unit-registry wrapper.

Ran: commands below executed locally in `/home/workdir/openWEPP`.

## Commands

### `cargo test --test sim_contract_boundary_unit_registry`

- Result: pass.
- Evidence: 9 tests passed.
- Coverage: registry construction, WAT publication metadata alignment,
  `prcp`/`P` unit separation, climate/snow/soil/WB13 aliases, required-alias
  manifest, missing-unit failure, scalar-exception failure, duplicate/ambiguous
  aliases, invalid templates, ambiguous template lookup, and duplicate
  publication aliases.

### `cargo clippy --test sim_contract_boundary_unit_registry -- -D warnings`

- Result: pass.
- Evidence: focused clippy completed with warnings denied.

### `tools/release/check_unit_registry.sh`

- Result: pass.
- Evidence: wrapper ran the focused registry test and focused clippy gate.

### `cargo fmt --check`

- Result: pass.
- Evidence: formatting check completed with no diff.

### `cargo test -p openwepp-sim-contract`

- Result: pass.
- Evidence: crate unit tests and doc tests completed successfully.

### `markdown-doc lint --path ...`

- Result: pass.
- Evidence: `27 files validated, 0 errors, 0 warnings` for touched
  specification and HPHYS0274 work-package docs.

## Not Run

- `cargo test --workspace`: not run for HPHYS0274 because this package changed
  the sim-contract registry, tests, tooling, and docs only; no kernel/runtime
  behavior or physics path changed.
- `cargo deny check`: not run; no dependency additions beyond a root dev-dep on
  an existing workspace crate.
