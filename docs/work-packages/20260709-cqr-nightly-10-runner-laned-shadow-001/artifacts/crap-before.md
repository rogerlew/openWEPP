# CRAP Before

Evidence label: Static.

Status: `SCAFFOLDED`

Source:

- `/tmp/openwepp-cqr-nightly-crap.json`
- SHA-256:
  `636ce39bc06a7172ee9e62ee9946afd2dda25f0cd56a76cfd5cad047d6438289`

Target:

- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- Deduplicated target rows: `16`
- Rows above CRAP `30`: `3`
- Max CRAP: `210.0`
- Total excess over `30`: `374`

Rows above CRAP `30`:

| Function | Line | Cyclomatic | Coverage | CRAP | Excess |
|---|---:|---:|---:|---:|---:|
| `LanedShadowCollector::observe_row` | `219` | `14.0` | `0.0` | `210.0` | `180.0` |
| `LanedShadowCollector::validate_lane_day_operands` | `298` | `13.0` | `0.0` | `182.0` | `152.0` |
| `LanedShadowCollector::commit_day` | `354` | `8.0` | `0.0` | `72.0` | `42.0` |

Near-threshold row to monitor:

| Function | Line | Cyclomatic | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `LanedShadowCollector::finalize` | `516` | `5.0` | `0.0` | `30.0` |

Other target rows observed below threshold include
`LanedShadowCollector::route_buffered_day` (`7.000238583302091`),
`LanedShadowCollector::build_day_rate_series` (`6.216378662659654`),
`LanedShadowCollector::record_operand_build` (`6.0`),
`LanedShadowCollector::new` (`2.011661807580175`),
`LanedShadowCollector::emit_profile_report` (`2.0`), and the single-complexity
helpers/tests at or near `1.0`.
