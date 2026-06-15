# CQR11 Line-Count Governance Checklist

Status: complete.

Static: line-count governance applies to touched Rust files and the package
catalog.

Line counts:

| File | Before | After | Disposition |
| --- | ---: | ---: | --- |
| `crates/openwepp-input-contract/src/parsers/management.rs` | 1592 | 1660 | below 2000 |
| `tests/integration/infile_management_parser_contract.rs` | not captured | 618 | below 2000 |
| `docs/work-packages/README.md` | 517 after registration | 517 | docs catalog |

Static: no touched Rust file is at or above `2000` lines.

Static: no touched non-exempt Rust file is at or above `3000` lines.

Static: the existing crate-level
`#![allow(clippy::missing_errors_doc, clippy::too_many_lines)]` remains
unchanged; CQR11 did not add a new suppression.

Ran:

```console
wc -l crates/openwepp-input-contract/src/parsers/management.rs tests/integration/infile_management_parser_contract.rs docs/work-packages/README.md
```

Result: exit `0`.
