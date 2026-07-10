# CRAP After

Command:

`cargo crap --workspace --lcov /tmp/openwepp-cqr-b02-t09-focused2.lcov --min 0 --format json --output /tmp/openwepp-cqr-b02-t09-focused2-crap.json`

Evidence:

- CRAP JSON:
  `/tmp/openwepp-cqr-b02-t09-focused2-crap.json`
- CRAP JSON SHA-256:
  `fd877e5780d261e03e2a85b6241d3ac0da819b2701d7f3e205cbe1569116f343`
- Rows above 30: 0.
- Maximum slope parser CRAP: 17.1852 (`derive_distance_mode`).

Original target rows after implementation:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `slope_parser_error_message` | 163 | 12 | 100.0000% | 12.0000 |
| `parse_slope_str` | 267 | 8 | 95.6522% | 8.0053 |

Status: PASS. Every eligible slope parser function is <= 30.
