# INIMPL10 Worktree Ownership Manifest

Evidence mode: `Static`

## Purpose

Define disjoint write ownership for Wave 2 worker packages and isolate shared coupling files to a single coordinator-owned quarantine stream.

## 1. Worker-Owned Write Sets

| Package | Contract surface | Allowed parser file(s) | Allowed integration test file(s) | Allowed fixture namespace |
| --- | --- | --- | --- | --- |
| `INIMPL11` | `SC-INFILE-PMETPARA-001` | `crates/openwepp-input-contract/src/parsers/pmetpara.rs` | `tests/integration/infile_pmetpara_parser_contract.rs` | `tests/fixtures/infile/pmetpara/**` |
| `INIMPL12` | `SC-INFILE-IRRIGATION-DEPLETION-001` | `crates/openwepp-input-contract/src/parsers/irrigation_depletion.rs` | `tests/integration/infile_irrigation_depletion_parser_contract.rs` | `tests/fixtures/infile/irrigation_depletion/**` |
| `INIMPL13` | `SC-INFILE-IRRIGATION-FIXEDDATE-001` | `crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs` | `tests/integration/infile_irrigation_fixeddate_parser_contract.rs` | `tests/fixtures/infile/irrigation_fixeddate/**` |
| `INIMPL14` | `SC-INFILE-FROST-001` | `crates/openwepp-input-contract/src/parsers/frost.rs` | `tests/integration/infile_frost_parser_contract.rs` | `tests/fixtures/infile/frost/**` |
| `INIMPL15` | `SC-INFILE-SNOW-001` | `crates/openwepp-input-contract/src/parsers/snow.rs` | `tests/integration/infile_snow_parser_contract.rs` | `tests/fixtures/infile/snow/**` |
| `INIMPL16` | `SC-INFILE-WEPPUI-001` | `crates/openwepp-input-contract/src/parsers/wepp_ui.rs` | `tests/integration/infile_weppui_parser_contract.rs` | `tests/fixtures/infile/weppui/**` |

[DIRECT] Worker write-set intent is sourced from each package `Intended Write Set` section.

## 2. Shared-File Quarantine Ownership

These files are shared coupling points and are not worker-owned:

- `crates/openwepp-input-contract/src/parsers/mod.rs`
- `crates/openwepp-input-contract/src/lib.rs`
- `crates/openwepp-input-contract/Cargo.toml`
- `Cargo.toml`
- `tests/integration/mod.rs` (if used)
- `tests/fixtures/infile/README.md` (if used)

Owner:
- integration/scaffold coordinator stream (`INIMPL10` governance + `INIMPL17` integration authority).

## 3. Prohibited Overlap

1. No worker edits another worker parser file, test file, fixture namespace, or package artifacts.
2. No worker directly edits quarantine-owned shared files.
3. No worker modifies Wave 2 contract/spec authorities during implementation packages.
4. No worker mutates another worker's worktree branch registry artifacts.

## 4. Shared-Change Request Protocol

When a worker needs shared-file changes:
1. Record requested change in `artifacts/worker-handoff.md`.
2. Do not commit direct shared-file edits on worker branch.
3. Integration/scaffold coordinator applies or rejects during `INIMPL17` intake.

## 5. Ownership Acceptance Gate

Before first code commit in each worker package:
1. Worker acknowledges this manifest in package artifacts.
2. Coordinator confirms Wave 2 shared scaffold baseline commit exists.
3. Worker branch has no unauthorized edits outside owned write set.

If any check fails: worker start remains `HOLD`.
