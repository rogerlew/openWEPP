# CQR18 CRAP After

Status: closed.

Ran: after CRAP command:

```text
cargo crap --workspace \
  --lcov docs/work-packages/20260615-cqr18-hbp-payload-validator-complexity-001/artifacts/lcov_after.info \
  --min 0 --format json \
  --output docs/work-packages/20260615-cqr18-hbp-payload-validator-complexity-001/artifacts/crap_after.json
```

Result: exit code `0`; JSON saved to `crap_after.json`.

Highest target-file rows:

```text
parse_runoff_event_payload      line 259 CC 13.0 Cov 93.75 CRAP 13.041259765625
validate_payload                line 33  CC 9.0  Cov 100.0 CRAP 9.0
validate_state_snapshot         line 369 CC 8.0  Cov 95.23809523809523 CRAP 8.006910700788252
read_payload_header             line 147 CC 7.0  Cov 100.0 CRAP 7.0
parse_latest_event_payload      line 228 CC 5.0  Cov 64.28571428571429 CRAP 6.138848396501457
parse_non_runoff_event_payload  line 252 CC 2.0  Cov 0.0 CRAP 6.0
read_state_schema_fields        line 433 CC 6.0  Cov 100.0 CRAP 6.0
extract_schema2_payload         line 109 CC 5.0  Cov 68.57142857142857 CRAP 5.776093294460641
supported_payload_minor         line 201 CC 4.0  Cov 55.55555555555556 CRAP 5.404663923182442
read_state_envelope             line 393 CC 5.0  Cov 100.0 CRAP 5.0
validate_expected_state_schema  line 468 CC 5.0  Cov 100.0 CRAP 5.0
```

Closure:

- `validate_payload` is CRAP `9.0`.
- Maximum target-file helper CRAP is `13.041259765625`.
- Defensive query for target-file rows with CRAP `> 30` returned no rows.
