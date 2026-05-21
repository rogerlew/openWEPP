# INIMPL19 Owned File Manifest

Evidence: `Ran` + `Static`

## Owned Write-Set Changes

| File | Status | Notes |
| --- | --- | --- |
| `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` | added | Wave 3 watershed-structure parser with strict/compat logic + typed errors/warnings. |
| `tests/integration/infile_watershed_structure_parser_contract.rs` | added | Surface integration tests for parser contract and guard/error paths. |
| `tests/fixtures/infile/watershed_structure/strict_valid_two_rows.str` | added | strict valid fixture |
| `tests/fixtures/infile/watershed_structure/compat_no_datver_valid.str` | added | compatibility no-datver fixture |
| `tests/fixtures/infile/watershed_structure/strict_unsupported_datver_invalid.str` | added | datver rejection fixture |
| `tests/fixtures/infile/watershed_structure/strict_invalid_arity_invalid.str` | added | row arity mismatch fixture |
| `tests/fixtures/infile/watershed_structure/strict_invalid_element_type_invalid.str` | added | invalid `elmt` domain fixture |
| `tests/fixtures/infile/watershed_structure/strict_disconnected_invalid.str` | added | disconnected-element fixture |
| `tests/fixtures/infile/watershed_structure/strict_invalid_hillslope_domain_invalid.str` | added | invalid hillslope id fixture |
| `tests/fixtures/infile/watershed_structure/strict_invalid_upstream_reference_invalid.str` | added | invalid upstream element reference fixture |
| `docs/work-packages/20260521-inimpl19-implement-sc-infile-watershed-structure-parser-001/artifacts/worker-handoff.md` | added | required package artifact |
| `docs/work-packages/20260521-inimpl19-implement-sc-infile-watershed-structure-parser-001/artifacts/owned-file-manifest.md` | added | required package artifact |
| `docs/work-packages/20260521-inimpl19-implement-sc-infile-watershed-structure-parser-001/artifacts/review_agent_a.md` | added | required package artifact |
| `docs/work-packages/20260521-inimpl19-implement-sc-infile-watershed-structure-parser-001/artifacts/review_agent_b.md` | added | required package artifact |
| `docs/work-packages/20260521-inimpl19-implement-sc-infile-watershed-structure-parser-001/artifacts/inimpl19_disposition.md` | added | required package artifact |
| `docs/work-packages/20260521-inimpl19-implement-sc-infile-watershed-structure-parser-001/artifacts/verification_agent_a.md` | added | required package artifact |
| `docs/work-packages/20260521-inimpl19-implement-sc-infile-watershed-structure-parser-001/artifacts/verification_agent_b.md` | added | required package artifact |

## Quarantine-Owned Files Not Edited

- `crates/openwepp-input-contract/src/parsers/mod.rs`
- `Cargo.toml`

Requested integration-owner updates are recorded in `worker-handoff.md`.
