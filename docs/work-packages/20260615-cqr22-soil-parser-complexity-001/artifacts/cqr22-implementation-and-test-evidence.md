# CQR22 Implementation and Test Evidence

Status: complete.

Static: implementation summary:

- Added private `POLICY_LKEFF_SENTINEL`.
- Split `parse_policy_row` into DATVER dispatch plus private
  `parse_v9002_policy_row`, `parse_v9003_policy_row`, and
  `parse_v9005_policy_row`.
- Added private `parse_burn_code`, `parse_texid_enum`, and
  `parse_lkeff_policy_value`.
- Added focused tests under the existing private module scope for CQR22
  characterization.

Ran: initial characterization attempts exposed exact current messages that had
to be captured rather than inferred:

```text
binary flag error message: "binary flag must be 0 or 1"
policy arity message: "variant arity mismatch: expected 5 token(s), found 4"
non-negative numeric validation message: "value must be >= 0"
```

Ran: focused characterization before production refactor:

```bash
cargo test -p openwepp-input-contract cqr22_parse_policy_row_characterizes
```

Result: PASS, `2` passed.

Ran: focused characterization after production refactor:

```bash
cargo test -p openwepp-input-contract cqr22_parse_policy_row_characterizes
```

Result: PASS, `2` passed.

Ran: full workspace tests after production refactor:

```bash
cargo test --workspace
```

Result: PASS.
