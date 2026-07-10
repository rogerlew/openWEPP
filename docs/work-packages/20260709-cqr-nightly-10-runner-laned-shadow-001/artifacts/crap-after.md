# CRAP After

Evidence label: Static/Ran.

Status: `EXECUTED`

Source:
`/tmp/openwepp-cqr-nightly-10-runner-laned-shadow-targeted-crap.json`

Command:

- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-10-runner-laned-shadow-targeted.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-10-runner-laned-shadow-targeted-crap.json`

Command result:

- PASS, exit `0`.
- Warning: `281` source files had no matching LCOV entry because the LCOV was a
  targeted openwepp-runner measurement; target rows are valid for this Phase D
  targeted equivalent.

Artifact:

- bytes: `2677085`
- sha256:
  `20c173b5b63817c21013608bdb615c3546ab4df21eea2b734a80f4c0222fa99a`

Summary:

- Deduplicated target rows: `16`
- Rows above CRAP `30`: `0`
- Max target CRAP: `14.016830348056178`

Top target rows:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `LanedShadowCollector::observe_row` | `219` | `14.0` | `95.58823529411765` | `14.016830348056178` |
| `LanedShadowCollector::validate_lane_day_operands` | `298` | `13.0` | `100.0` | `13.0` |
| `LanedShadowCollector::commit_day` | `354` | `8.0` | `100.0` | `8.0` |
| `LanedShadowCollector::route_buffered_day` | `449` | `7.0` | `98.30508474576271` | `7.000238583302091` |
| `LanedShadowCollector::build_day_rate_series` | `421` | `6.0` | `100.0` | `6.0` |
| `LanedShadowCollector::finalize` | `516` | `5.0` | `84.61538461538461` | `5.091033227127902` |
| `LanedShadowCollector::new` | `155` | `2.0` | `85.71428571428571` | `2.011661807580175` |
| `LanedShadowCollector::record_operand_build` | `210` | `2.0` | `100.0` | `2.0` |
