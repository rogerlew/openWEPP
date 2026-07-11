# CRAP After

Ran during the rejected attempt: focused LCOV plus cargo-crap exited `0`. CRAP
JSON SHA-256:
`189d8b24fd590c16fc69c62c5d44bb5348ed8e4b39354c9f1e7ef84903e48d7c`.

| Row | Before | After | Disposition |
|---|---:|---:|---|
| `read_wat_batch` | 65.469 | 4.002 | Closed by decomposition. |
| `Totalwatsed3Error::code` | 110.000 | 110.000 | ADR-0021 error-code mapping disposition. |
| `Totalwatsed3Error::fmt` | 110.000 | 110.000 | ADR-0021 observability-format disposition. |
| `date_ofe_key_from_columns` | 72.000 | 72.000 | Optional-input coverage disposition. |

All attempted helpers were `<=30`; maximum was `read_wat_values` at `23.384`, while an
unchanged generic column helper is exactly `30`. No new high-CRAP row exists.
Independent review rejected two retained-row dispositions and found the
cover-first gate unmet. The source is rolled back, so this is attempt evidence,
not an accepted after state or metric-closure claim.
