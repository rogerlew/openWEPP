# CRAP After

Ran: full workspace CRAP-after computed from emitted fullcov LCOV.

Command:

`cargo crap --workspace --lcov /tmp/openwepp-cqr-b02-t10-fullcov.lcov --min 0 --format json --output /tmp/openwepp-cqr-b02-t10-fullcov-crap.json`

Output:

- JSON: `/tmp/openwepp-cqr-b02-t10-fullcov-crap.json`
- SHA-256:
  `2524cc8eef2bdf1122c1b42987a09abac99a0e63fab08036c8530553bec836aa`

Target module:
`crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `parse_runoff_event_payload` | 292 | 21 | 91.667% | 21.255 |
| `parse_non_runoff_event_payload` | 262 | 13 | 100.000% | 13.000 |
| `validate_payload` | 33 | 9 | 100.000% | 9.000 |
| `validate_state_snapshot` | 461 | 8 | 95.238% | 8.007 |
| `read_payload_header` | 145 | 7 | 100.000% | 7.000 |
| `read_state_schema_fields` | 525 | 6 | 100.000% | 6.000 |
| `extract_schema2_payload` | 107 | 5 | 68.571% | 5.776 |
| `parse_latest_event_state` | 224 | 5 | 71.429% | 5.583 |
| `supported_payload_minor` | 199 | 4 | 55.556% | 5.405 |
| `validate_hourly_surface` | 379 | 4 | 60.000% | 5.024 |

Maximum target-module CRAP after implementation: `21.255`.

Rows above `30`: none.
