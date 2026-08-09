# Pre-Implementation Contract Gate

Status: `complete / expected red`

Evidence mode: `Ran`

Ran from `/home/workdir/openWEPP`:

```text
TMPDIR=/home/workdir/openwepp-task-tmp cargo nextest run --test peak_hourly_authority_contract
```

Result: expected failure before production edits. The production source lacked
`hourly_peak_runoff_depth_rate_m_s`, and publication lacked the exactly-once
area multiplier. The first run also exposed one test-text mismatch (`same`
versus canonical `exact` area wording); that assertion was corrected before
implementation. No production Rust had been edited when the red gate ran.
