# CQR11 Implementation And Test Evidence

Status: complete.

Static: production edit was limited to private helper extraction in
`crates/openwepp-input-contract/src/parsers/management.rs`.

Implementation summary:

- `parse_yearly_perennial` now delegates to private header, option-domain, and
  payload parser helpers.
- Added private `YearlyPerennialHeader` and `YearlyPerennialPayload` structs.
- Added private helpers for cut-day and grazing-cycle payload parsing.
- Preserved parse order for `jdharv`, `jdplt`, `jdstop`, `rw`, and `mgtopt`.
- Preserved legacy `mgtopt` domains: legacy `1..3`, 2016-plus `1..7`, with
  2016-plus values `4..=7` still rejected as parser-unsupported.
- Preserved `ncut`, `ncycle`, `cutday`, `graze_cycle`, `gday`, and `gend`
  count, arity, and typed-error behavior.

Static: characterization tests were added in
`tests/integration/infile_management_parser_contract.rs` before production
refactor. They cover:

- accepted perennial cut-day yearly branch;
- accepted perennial grazing yearly branch;
- accepted perennial no-action branch;
- legacy `mgtopt` out-of-domain error;
- 2016-plus currently unsupported `mgtopt` error;
- zero cut count;
- cut-day arity;
- zero grazing cycle count;
- grazing-cycle arity.

Ran: focused characterization before production refactor:

```console
cargo test --test infile_management_parser_contract perennial -- --nocapture
```

Result: exit `0`, `9` passed.

Ran: focused characterization after production refactor and formatting:

```console
cargo test --test infile_management_parser_contract perennial -- --nocapture
```

Result: exit `0`, `9` passed.

Ran: full closure test command:

```console
cargo test --workspace
```

Result: exit `0`.
