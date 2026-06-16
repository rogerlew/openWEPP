# CQR27 Line-Count Governance Checklist

Status: complete.

Ran: package line counts:

| Path | Before package | After package | Result |
| --- | ---: | ---: | --- |
| `crates/openwepp-input-contract/src/parsers/management.rs` | `1660` | `1705` | below hard `3000` line ceiling |
| `tests/integration/infile_management_parser_contract.rs` | `618` | `835` | below hard `3000` line ceiling |
| `docs/work-packages/README.md` | `641` | `645` | catalog update only |
| `docs/work-packages/cqr-burndown-execplan.md` | `729` | `729` | unchanged during package commit |

Ran: target-file suppression census:

| Path | Suppression |
| --- | --- |
| `crates/openwepp-input-contract/src/parsers/management.rs:1` | `#![allow(clippy::missing_errors_doc, clippy::too_many_lines)]` |

Static: no new suppression attribute was added.

Static: touched Rust files remain below the `3000` line hard stop from
`docs/decisions/0021-module-coverage-closure-thresholds.md`.
