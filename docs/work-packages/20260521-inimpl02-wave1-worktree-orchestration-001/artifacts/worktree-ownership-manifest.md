# INIMPL02 Worktree Ownership Manifest

Evidence mode: `Static`

## Purpose

Define disjoint write ownership for Wave 1 worker packages and isolate shared-file mutation to a single owner stream.

## 1. Worker-Owned Write Sets

| Package | Contract surface | Allowed parser file(s) | Allowed integration test file(s) | Allowed fixture namespace |
| --- | --- | --- | --- | --- |
| `INIMPL03` | `SC-INFILE-SLOPE-001` | `crates/openwepp-input-contract/src/parsers/slope.rs` | `tests/integration/infile_slope_parser_contract.rs` | `tests/fixtures/infile/slope/**` |
| `INIMPL04` | `SC-INFILE-SOIL-001` | `crates/openwepp-input-contract/src/parsers/soil.rs` | `tests/integration/infile_soil_parser_contract.rs` | `tests/fixtures/infile/soil/**` |
| `INIMPL05` | `SC-INFILE-CLIMATE-001` | `crates/openwepp-input-contract/src/parsers/climate.rs` | `tests/integration/infile_climate_parser_contract.rs` | `tests/fixtures/infile/climate/**` |
| `INIMPL06` | `SC-INFILE-MANAGEMENT-001` | `crates/openwepp-input-contract/src/parsers/management.rs` | `tests/integration/infile_management_parser_contract.rs` | `tests/fixtures/infile/management/**` |

[DIRECT] Worker write-set intent is sourced from each package's `Intended Write Set` section.

## 2. Shared-File Quarantine Ownership

These files are shared coupling points and are not worker-owned:

- `Cargo.toml` (workspace members)
- `crates/openwepp-input-contract/Cargo.toml`
- `crates/openwepp-input-contract/src/lib.rs`
- `crates/openwepp-input-contract/src/parsers/mod.rs`
- `tests/integration/mod.rs` (if used)
- `tests/fixtures/infile/README.md` (if used)

Owner: integration/scaffold coordinator stream (`INIMPL02` preflight + `INIMPL07` integration authority).

## 3. Prohibited Overlap

1. No worker may edit another worker's parser file or test file.
2. No worker may edit shared quarantine files directly.
3. No worker may modify `docs/specifications/science-contracts/contracts/SC-INFILE-{SLOPE,SOIL,CLIMATE,MANAGEMENT}-001.md` during implementation packages.
4. No worker may alter another worker's fixture namespace.

## 4. Shared-Change Request Protocol

When a worker requires shared-file changes:
1. Record requested change in worker handoff artifact.
2. Do not commit direct shared-file edits on worker branch.
3. Integration/scaffold coordinator applies or rejects during intake.

## 5. Ownership Acceptance Gate

Before first code commit in each worker package:
1. Worker acknowledges this manifest in package artifacts.
2. Integration coordinator confirms scaffold baseline is present.
3. Worker branch contains no unauthorized file edits.

If any check fails: `HOLD` for that worker start.
