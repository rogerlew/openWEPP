# CQR27 Implementation and Test Evidence

Status: complete.

Static: implementation summary:

- extracted `YearlyAnnualFallowHeader`;
- extracted `parse_yearly_annual_fallow_header`;
- extracted `validate_yearly_annual_resmgt`;
- extracted `parse_yearly_annual_extension`;
- extracted `parse_yearly_annual_cut_records`;
- extracted `parse_yearly_annual_cut_entry`;
- left `parse_yearly_annual_fallow` as orchestration over the same parse
  sequence.

Static: no formula, unit, runtime symbol, alias, parser public API, or error
authority was changed.

Static: characterization tests added in
`tests/integration/infile_management_parser_contract.rs`:

- `strict_mode_parses_annual_residue_management_extensions`;
- `strict_mode_parses_2016_annual_cut_records`;
- `strict_mode_rejects_legacy_annual_residue_management_seven`;
- `strict_mode_rejects_annual_cut_zero_count`;
- `strict_mode_rejects_annual_cut_short_entry`.

Ran: focused test evidence:

| Command | Result |
| --- | --- |
| `cargo test --test infile_management_parser_contract` before production refactor | pass, `30` passed |
| `cargo fmt --check` after production refactor | pass |
| `cargo test --test infile_management_parser_contract` after production refactor | pass, `30` passed |

Ran: full validation is recorded in `gate-results.md`.
