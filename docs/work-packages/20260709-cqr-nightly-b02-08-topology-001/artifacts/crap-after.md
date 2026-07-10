# CRAP After

Command:

`cargo crap --workspace --lcov /tmp/openwepp-cqr-b02-t08-final4.lcov --min 0 --format json --output /tmp/openwepp-cqr-b02-t08-final4-crap.json`

Evidence:

- LCOV SHA-256:
  `7bf6b81aefe36bc8dada1991aef481e380472684372520e45136ed48686a4692`
- CRAP JSON SHA-256:
  `57e289ecad3c692224d6b4ac29f25f7a559e47f398596fcf9b17e954a301d60a`
- Unique topology rows: 60.
- Rows above 30: 0.
- Maximum topology CRAP: 10.000 (`parse_contributors`, coverage 100%, CC 10).
- Filtered topology rows below 75% coverage: 2; both are retained
  fail-closed count-overflow branches marked `COVERAGE-EXCLUDE` because the
  overflow is type-impossible on supported openWEPP targets.

Original target rows after implementation:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `TopologyParseError::fmt` | 318 | 1 | 100.000% | 1.000 |
| `parse_topology_fixture_str` | 387 | 3 | 100.000% | 3.000 |
| `validate_pre_execution_topology` | 426 | 5 | 95.000% | 5.003 |
| `collect_channel_count_violation` | 735 | 3 | 53.571% | 3.901 |
| `collect_impoundment_count_violation` | 767 | 3 | 58.065% | 3.664 |

Status: PASS. Every eligible topology function is <= 30.
