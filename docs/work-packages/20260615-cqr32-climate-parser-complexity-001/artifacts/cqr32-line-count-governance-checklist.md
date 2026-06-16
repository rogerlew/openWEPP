# CQR32 Line-Count Governance Checklist

Ran: `wc -l` before and after package edits.

| File | Before | After | Status |
|---|---:|---:|---|
| `crates/openwepp-input-contract/src/parsers/climate.rs` | 877 | 883 | PASS |
| `tests/integration/infile_climate_parser_contract.rs` | not captured | 400 | PASS |
| `docs/work-packages/README.md` | 678 | 682 | PASS |
| `docs/work-packages/cqr-burndown-execplan.md` | 760 | 760 | PASS |

No touched `.rs` file is at or above the `2000` line WARN threshold or the
`3000` line closure-blocking threshold.
