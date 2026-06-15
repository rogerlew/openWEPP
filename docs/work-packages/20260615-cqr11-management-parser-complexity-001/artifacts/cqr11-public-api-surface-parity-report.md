# CQR11 Public API Surface Parity Report

Status: complete.

Static: planned production edits are private helper extraction in a parser
module. No public parser API change is authorized.

Static: post-refactor public symbol scan in
`crates/openwepp-input-contract/src/parsers/management.rs` still reports the
same public parser data types and entry points:

- `ParseMode`
- `ManagementSectionCounts`
- `ScenarioMeta`
- management scenario/output data structs and enums
- `ManagementParseError`
- `ManagementParseError::contract_error_id`
- `parse_management_from_path`
- `parse_management_from_str`

Static: newly introduced production symbols are private:

- `YearlyPerennialHeader`
- `YearlyPerennialPayload`
- `parse_yearly_perennial_header`
- `validate_yearly_perennial_mgtopt`
- `parse_yearly_perennial_payload`
- `parse_yearly_perennial_cut_days`
- `parse_yearly_perennial_cut_day`
- `parse_yearly_perennial_grazing_cycles`
- `parse_yearly_perennial_grazing_cycle`

Ran:

```console
rg -n "pub (fn|struct|enum|type|use)|pub\\(" crates/openwepp-input-contract/src/parsers/management.rs
```

Result: exit `0`.

Disposition: no public API surface delta.
