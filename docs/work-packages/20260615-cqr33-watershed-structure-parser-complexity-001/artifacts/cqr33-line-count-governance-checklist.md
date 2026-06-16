# CQR33 Line-Count Governance Checklist

Ran: `wc -l` before package registration and after package edits.

| File | Before | After | Status |
|---|---:|---:|---|
| `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` | 706 | 712 | PASS |
| `tests/integration/infile_watershed_structure_parser_contract.rs` | 269 | 400 | PASS |
| `docs/work-packages/README.md` | 687 | 696 | PASS |
| `docs/work-packages/cqr-burndown-execplan.md` | 766 | 766 | PASS |

No touched `.rs` file is at or above the `2000` line WARN threshold or the
`3000` line closure-blocking threshold.
