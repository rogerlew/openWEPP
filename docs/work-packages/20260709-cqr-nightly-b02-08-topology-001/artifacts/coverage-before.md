# Coverage Baseline

The high-CRAP parser, error display, and validator need exact fixture/error
characterization before decomposition.

Batch baseline source: `/tmp/openwepp-cqr-nightly-new-isolated.lcov`
(`87811062b33fe1c79176843204e01410aca9b2530c09cb60a98e3ec4a2f60cce`) and
`/tmp/openwepp-cqr-nightly-new-isolated-crap.json`
(`5e3abb273dffdc8c8308da0af9f3d85283307b2b2c77f0c9ba00c3a55741f765`).

Baseline target rows:

| Function | Line | Coverage | CRAP |
|---|---:|---:|---:|
| `parse_topology_fixture_str` | 426 | 60.577% | 110.056 |
| `TopologyParseError::fmt` | 318 | 0% | 110.000 |
| `validate_pre_execution_topology` | 585 | 70.175% | 30.612 |

Disposition: baseline coverage was insufficient for behavior-preserving
decomposition, so this package added public characterization tests before
production refactor.
