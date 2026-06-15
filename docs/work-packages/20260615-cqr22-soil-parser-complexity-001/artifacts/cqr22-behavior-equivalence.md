# CQR22 Behavior Equivalence

Status: complete.

Static: production behavior change intent is none. The refactor only moved
existing `parse_policy_row` DATVER arms into private helpers and shared the
existing burn-code, texture-id, and `lkeff` sentinel validation logic through
private helpers.

Static: preserved behavior boundaries:

- DATVER dispatch for V9002, V9003, V9005, and non-policy DATVER rejection.
- Token parsing through `parse_policy_tokens`.
- Binary flag parsing through `parse_binary_flag`.
- Numeric parsing through existing `parse_i32` and `parse_f64`.
- `burn_code` non-negative validation and message.
- `texid_enum` positive validation and message.
- `uksat` and `lkeff` non-negative validation message.
- `-9999.0` `lkeff` sentinel acceptance.
- `DisturbedPolicy` field assignment order and values.

Ran: focused CQR22 characterization passed before and after production
refactor:

```bash
cargo test -p openwepp-input-contract cqr22_parse_policy_row_characterizes
```

Ran: full workspace tests passed after production refactor:

```bash
cargo test --workspace
```
