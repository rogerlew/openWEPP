# Review Agent B

Status: complete.

Static: review stance focused on scope control, metric closure, and public API
parity.

Findings: none.

Static: checked that no public parser type or `parse_soil` signature changed.

Static: checked that production changes are limited to private helpers and a
private sentinel constant in `soil.rs`.

Static: checked that out-of-scope high-CRAP rows remain untouched future CQR
targets rather than newly introduced helpers.

Ran: after CRAP shows scoped target/helper closure:

```text
parse_policy_row           CRAP 5.0
parse_v9005_policy_row     CRAP 8.004096
parse_v9002_policy_row     CRAP 7.0
parse_v9003_policy_row     CRAP 5.003125
parse_lkeff_policy_value   CRAP 4.0
parse_burn_code            CRAP 3.0
parse_texid_enum           CRAP 3.0
```

Residual risk: low. Remaining high-CRAP rows in the file are pre-existing
out-of-scope functions.
