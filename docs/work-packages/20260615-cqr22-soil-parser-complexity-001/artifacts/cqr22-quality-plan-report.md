# CQR22 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in `crates/openwepp-input-contract/src/parsers/soil.rs`.

Static: protected boundaries are public API, typed guards, stable error IDs,
parser compatibility, warnings, aliases, units, field ordering, formulas, float
expression order, and science-contract behavior.

Ran: before target-file line count:

```text
1359 crates/openwepp-input-contract/src/parsers/soil.rs
```

Ran: after target-file line count:

```text
1485 crates/openwepp-input-contract/src/parsers/soil.rs
```

Ran: before LCOV and CRAP showed the live CQR22 target was
`parse_policy_row` at line `648`, CC `29.0`, coverage
`26.041666666666668`, and CRAP `369.2180435745803`.

Static: refactor plan was private extraction of DATVER-specific policy row
parsers plus tiny validation helpers. No public parser API, error code, field,
message, compatibility, warning, symbol, unit, field ordering, or
science-contract behavior change was authorized.

Ran: after LCOV and CRAP showed final scoped target `parse_policy_row` at CRAP
`5.0`. New helper rows were `parse_v9005_policy_row` CRAP `8.004096`,
`parse_v9002_policy_row` CRAP `7.0`, `parse_v9003_policy_row` CRAP `5.003125`,
`parse_lkeff_policy_value` CRAP `4.0`, `parse_burn_code` CRAP `3.0`, and
`parse_texid_enum` CRAP `3.0`.

Status: complete.
