# CQR18 CRAP Before

Status: closed.

Ran: before CRAP command:

```text
cargo crap --workspace \
  --lcov docs/work-packages/20260615-cqr18-hbp-payload-validator-complexity-001/artifacts/lcov_before.info \
  --min 0 --format json \
  --output docs/work-packages/20260615-cqr18-hbp-payload-validator-complexity-001/artifacts/crap_before.json
```

Result: exit code `0`; JSON saved to `crap_before.json`.

Live before row for
`crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`:

```text
validate_payload line 11 CC 80.0 Cov 61.111111111111114 CRAP 456.4060356652947
```

Target identity: `validate_payload`.

Target baseline: CRAP `456.4060356652947`, CC `80.0`, coverage
`61.111111111111114`.
