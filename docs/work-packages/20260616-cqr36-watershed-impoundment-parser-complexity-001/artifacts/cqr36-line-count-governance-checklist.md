# CQR36 Line Count Governance Checklist

Status: complete.

| File | Before | After | Status |
| --- | ---: | ---: | --- |
| `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs` | 1390 | 1556 | below 3000 |
| `tests/integration/infile_watershed_impoundment_parser_contract.rs` | 393 | 615 | below 3000 |
| `docs/work-packages/README.md` | 715 | 720 | documentation catalog |
| `docs/work-packages/cqr-burndown-execplan.md` | 785 | 785 | unchanged in package commit |

Suppression census for target production file:

- Existing file-level allow block remains:
  `clippy::missing_errors_doc`, `clippy::module_name_repetitions`,
  `clippy::similar_names`, `clippy::struct_excessive_bools`,
  `clippy::too_many_lines`.
- No new `allow` attributes were added.
- No `unsafe` blocks were introduced.
- No production `.unwrap()` or `.expect()` calls were introduced.

Result: no touched non-exempt Rust file is at or above the hard 3000-line
ceiling.
