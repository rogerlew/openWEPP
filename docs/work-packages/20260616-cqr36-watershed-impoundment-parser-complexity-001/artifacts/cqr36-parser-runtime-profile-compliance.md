# CQR36 Parser Runtime Profile Compliance

Status: complete.

Scope: `.imp` watershed impoundment parser and downstream watershed runtime
projection surfaces.

Static: public parser entry points are unchanged:

- `parse_watershed_impoundment_from_path`
- `parse_watershed_impoundment_from_str`

Static: public data structures, enums, and stable parser error/warning IDs are
unchanged. The package adds only private helper structs and private parser
helpers inside `watershed_impoundment.rs`.

Protected behavior preserved:

- strict and compatibility mode preamble handling;
- `IMP-E-*` and `IMP-W-*` identifiers;
- branch arity contexts such as `drop.ids2.line1`, `emergency.rating.hes`,
  `riser.payload.line4`, and `curve_baseline`;
- optional branch comments and payload order;
- vector consumption order for `hest`, `qes`, `hal`, `area`, and `length`;
- domain and invariant guards for `deltat`, `qinf`, `hot >= h`,
  `hfull >= hmin`, positive `nalpts`, positive `npts`, and monotone stage;
- downstream `StructureFlags`, code fields, culvert payloads, storage fields,
  curve fields, and impoundment runtime projection inputs.

Ran:
`cargo test --test infile_watershed_impoundment_parser_contract`

Result: `22 passed; 0 failed`.

Ran: final workspace LCOV covered the parser integration suite and watershed
runtime tests including `watershed_impoundment_runtime_seed_*`.
