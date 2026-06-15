# Verification Agent A

Status: complete.

Ran: before LCOV and CRAP were generated from workspace coverage.

Ran: after LCOV and CRAP were generated from workspace coverage after
characterization and production refactor.

Ran: after target-file coverage:

```text
lines 847/1124 75.36%
functions 45/52 86.54%
```

Ran: after target/helper metric closure:

```text
parse_policy_row           CRAP 5.0
parse_v9005_policy_row     CRAP 8.004096
parse_v9002_policy_row     CRAP 7.0
parse_v9003_policy_row     CRAP 5.003125
parse_lkeff_policy_value   CRAP 4.0
parse_burn_code            CRAP 3.0
parse_texid_enum           CRAP 3.0
```

Conclusion: metric closure verified.
