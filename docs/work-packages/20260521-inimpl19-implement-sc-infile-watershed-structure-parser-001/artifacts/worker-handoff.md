# INIMPL19 Worker Handoff

Evidence: `Ran` + `Static`

## Scope Delivered
Implemented `SC-INFILE-WATERSHED-STRUCTURE-001` parser surface for `infile-watershed-structure (.str)` with strict/compat behavior, typed errors, and fixture-backed integration tests.

Implemented in owned paths:
- `crates/openwepp-input-contract/src/parsers/watershed_structure.rs`
- `tests/integration/infile_watershed_structure_parser_contract.rs`
- `tests/fixtures/infile/watershed_structure/**`

## Contract-to-Code Mapping

| Contract area | Implementation evidence |
| --- | --- |
| datver gate (`>= 94.301`) and strict no-datver rejection | `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` (`UnsupportedDatver`, `LegacyNoDatverDisallowed`) |
| compatibility no-datver acceptance warning `STR-W-001` | `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` (`WatershedStructureWarningCode::StrW001`) |
| row arity closure (10 integer fields) | `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` (`RecordArityError`) |
| element domain (`elmt` in `{2,3}`) | `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` (`ElementTypeDomainError`) |
| non-isolated downstream element | `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` (`DisconnectedElementError`) |
| contributor domain/reference checks | `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` (`ContributorDomainError`) |
| derived `element_id`, `nchan`, `npond`, `nhmax` closure | `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` (`WatershedStructureRow`, `WatershedStructureSummary`) |
| cross-file count closure hooks (`expected_*_count`) | `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` (`ChannelCountMismatch`, `ImpoundmentCountMismatch`) |
| strict row-count closure mismatch | `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` (`RecordCountMismatch`) |

## Test Surface Added

`tests/integration/infile_watershed_structure_parser_contract.rs` adds strict/compat coverage for:
- valid strict parse,
- compat no-datver warning path,
- datver incompatibility,
- row arity, token, domain, connectivity failures,
- record-count closure mismatch,
- channel/impoundment cross-count mismatches,
- hillslope coverage mismatch,
- typed open error and nhill context failures.

Fixtures added under:
- `tests/fixtures/infile/watershed_structure/`

## Shared-File Change Requests (Quarantine-Owned)

Per Wave 3 ownership manifest, these were not edited in this worker stream:
1. `crates/openwepp-input-contract/src/parsers/mod.rs`
   - Requested integration action: add `pub mod watershed_structure;`
2. `Cargo.toml`
   - Requested integration action: add `[[test]]` entry for `tests/integration/infile_watershed_structure_parser_contract.rs`

Reason: both files are quarantine-owned by `INIMPL18`/`INIMPL22` governance.

## Gates and Evidence

Ran:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `CARGO_MANIFEST_DIR=/home/workdir/openWEPP/.worktrees/inimpl19-watershed-structure rustc --edition 2024 --test tests/integration/infile_watershed_structure_parser_contract.rs -o /tmp/infile_watershed_structure_parser_contract_test`
- `/tmp/infile_watershed_structure_parser_contract_test --nocapture`

Notes:
- `cargo test --workspace` does not include this new test target yet because `Cargo.toml` has explicit `[[test]]` entries and is quarantine-owned.
- direct `rustc --test` execution provides `Ran` evidence for this worker-owned integration test until `INIMPL22` wires shared test registry.
