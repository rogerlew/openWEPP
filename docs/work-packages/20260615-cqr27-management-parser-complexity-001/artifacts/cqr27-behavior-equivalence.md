# CQR27 Behavior Equivalence

Status: complete.

Static: production changes are limited to private decomposition inside
`crates/openwepp-input-contract/src/parsers/management.rs`.

Static: preserved parser surfaces:

- no public type, enum, struct, function, or module export changed;
- no `ManagementParseError` variant, `contract_error_id`, display text, field
  name, or allowed-domain string changed;
- annual/fallow record order is unchanged: `jdharv`, `jdplt`, `rw`, `resmgt`,
  then the residue-management extension payload;
- `resmgt` domain remains `1..6` for legacy datver families and `1..7` for
  `DatverFamily::V2016Plus`;
- annual cut records still parse flag, count, then `ncut` entries;
- annual cut count `0` still raises `InvalidCount` for `annual_cut.ncut`;
- short annual cut entries still raise `RecordArityError` with expected `2+`;
- extra annual cut entry tokens remain accepted as before;
- parsed `YearlyAnnualFallowData` and `YearlyAnnualExtension` shape is
  unchanged.

Ran: characterization tests were added before the production refactor for:

- annual/fallow residue-management extensions `1` through `6`;
- 2016.3 annual cut branch `resmgt == 7`;
- legacy rejection of `resmgt == 7`;
- annual cut zero-count rejection;
- annual cut short-entry rejection.

Ran: `cargo test --test infile_management_parser_contract` passed after
characterization additions and again after production refactor.
