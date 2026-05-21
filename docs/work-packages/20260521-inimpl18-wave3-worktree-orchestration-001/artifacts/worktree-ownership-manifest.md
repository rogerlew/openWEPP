# INIMPL18 Worktree Ownership Manifest

Evidence mode: `Static`

## Purpose

Define disjoint write ownership for Wave 3 worker packages and isolate shared
coupling files to a single coordinator-owned quarantine stream.

## 1. Worker-Owned Write Sets

| Package | Contract surface | Allowed parser file(s) | Allowed integration test file(s) | Allowed fixture namespace |
| --- | --- | --- | --- | --- |
| `INIMPL19` | `SC-INFILE-WATERSHED-STRUCTURE-001` | `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` | `tests/integration/infile_watershed_structure_parser_contract.rs` | `tests/fixtures/infile/watershed_structure/**` |
| `INIMPL20` | `SC-INFILE-WATERSHED-CHANNEL-001` | `crates/openwepp-input-contract/src/parsers/watershed_channel.rs` | `tests/integration/infile_watershed_channel_parser_contract.rs` | `tests/fixtures/infile/watershed_channel/**` |
| `INIMPL21` | `SC-INFILE-WATERSHED-IMPOUNDMENT-001` | `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs` | `tests/integration/infile_watershed_impoundment_parser_contract.rs` | `tests/fixtures/infile/watershed_impoundment/**` |

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
- integration/scaffold coordinator stream (`INIMPL18` governance + `INIMPL22`
  integration authority).

## 3. Prohibited Overlap

1. No worker edits another worker parser file, test file, fixture namespace, or
   package artifacts.
2. No worker directly edits quarantine-owned shared files.
3. No worker modifies Wave 3 contract/spec authorities during implementation
   packages.
4. No worker mutates another worker's worktree branch registry artifacts.

## 4. Shared-Change Request Protocol

When a worker needs shared-file changes:
1. Record requested change in `artifacts/worker-handoff.md`.
2. Do not commit direct shared-file edits on worker branch.
3. Integration/scaffold coordinator applies or rejects during `INIMPL22` intake.

## 5. Ownership Acceptance Gate

Before first code commit in each worker package:
1. Worker acknowledges this manifest in package artifacts.
2. Coordinator confirms Wave 3 shared scaffold baseline commit exists.
3. Worker branch has no unauthorized edits outside owned write set.

If any check fails: worker start remains `HOLD`.
