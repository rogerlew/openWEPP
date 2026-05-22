# INIMPL23 Worktree Ownership Manifest

Evidence mode: `Static`

## Purpose

Define disjoint write ownership for Wave 4 worker packages and isolate shared
coupling files to a single coordinator-owned quarantine stream.

## 1. Worker-Owned Write Sets

| Package | Contract surface | Allowed parser file(s) | Allowed integration test file(s) | Allowed fixture namespace |
| --- | --- | --- | --- | --- |
| `INIMPL24` | `SC-INFILE-CHANINP-001` | `crates/openwepp-input-contract/src/parsers/chaninp.rs` | `tests/integration/infile_chaninp_parser_contract.rs` | `tests/fixtures/infile/chaninp/**` |
| `INIMPL25` | `SC-INFILE-TC-001` | `crates/openwepp-input-contract/src/parsers/tc.rs` | `tests/integration/infile_tc_parser_contract.rs` | `tests/fixtures/infile/tc/**` |
| `INIMPL26` | `SC-INFILE-GWCOEFF-001` | `crates/openwepp-input-contract/src/parsers/gwcoeff.rs` | `tests/integration/infile_gwcoeff_parser_contract.rs` | `tests/fixtures/infile/gwcoeff/**` |
| `INIMPL27` | `SC-INFILE-TCR-001` | `crates/openwepp-input-contract/src/parsers/tcr.rs` | `tests/integration/infile_tcr_parser_contract.rs` | `tests/fixtures/infile/tcr/**` |
| `INIMPL28` | `SC-INFILE-PHOSPHORUS-001` | `crates/openwepp-input-contract/src/parsers/phosphorus.rs` | `tests/integration/infile_phosphorus_parser_contract.rs` | `tests/fixtures/infile/phosphorus/**` |
| `INIMPL29` | `SC-INFILE-LCWB-001` | `crates/openwepp-input-contract/src/parsers/lcwb.rs` | `tests/integration/infile_lcwb_parser_contract.rs` | `tests/fixtures/infile/lcwb/**` |

[DIRECT] Write-set surfaces map directly to Wave 4 parser contract scope and
ratified Wave 4 decision surfaces (`W4DR-001..012`).

## 2. Shared-File Quarantine Ownership

These files are shared coupling points and are not worker-owned:

- `crates/openwepp-input-contract/src/parsers/mod.rs`
- `crates/openwepp-input-contract/src/lib.rs`
- `crates/openwepp-input-contract/Cargo.toml`
- `Cargo.toml`
- `Cargo.lock`
- `tests/integration/mod.rs` (if used)
- `tests/fixtures/infile/README.md` (if used)

Owner:
- integration/scaffold coordinator stream (`INIMPL23` governance + `INIMPL30`
  integration authority).

## 3. Prohibited Overlap

1. No worker edits another worker parser file, test file, fixture namespace, or
   package artifacts.
2. No worker directly edits quarantine-owned shared files.
3. No worker modifies Wave 4 contract/spec authorities during implementation
   packages.
4. No worker mutates another worker's worktree branch registry artifacts.
5. No worker changes output-contract-owned row grammar authorities
   (`W4DR-012`) inside parser-surface packages.

## 4. Shared-Change Request Protocol

When a worker needs shared-file changes:
1. Record requested change in `artifacts/worker-handoff.md`.
2. Do not commit direct shared-file edits on worker branch.
3. Integration/scaffold coordinator applies or rejects during `INIMPL30` intake.

## 5. Ownership Acceptance Gate

Before first code commit in each worker package:
1. Worker acknowledges this manifest in package artifacts.
2. Coordinator confirms Wave 4 shared scaffold baseline commit exists.
3. Worker branch has no unauthorized edits outside owned write set.
4. Worker checklist includes applicable `W4DR` decision checks.

If any check fails: worker start remains `HOLD`.
