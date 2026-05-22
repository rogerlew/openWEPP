# Climate Coverage and Exclusions Matrix

Status: `complete`
Evidence mode: `Ran + Static`

Static:
- Coverage classification from baseline climate runtime and openWEPP contract/parser surfaces.

Ran:
- Executed source-level inventory and line trace commands.

| capability | legacy_status | openwepp_target_status | in_scope | notes |
|---|---|---|---|---|
| continuous-daily climate (`itemp=1`, `ibrkpt=0`) | `implemented` | `specified` | `yes` | Fully mapped (`infile -> stmget -> idat/disag -> consumers`). |
| breakpoint climate (`itemp=1`, `ibrkpt=1`) | `implemented` | `specified` | `yes` | Fully mapped (`stmget -> brkpt -> idat bypass disag for rain`). |
| climate daily metadata/monthly normals propagation | `implemented` | `specified` | `yes` | Header, station metadata, monthly normals mapped to parser/runtime boundary requirements. |
| winter start-time coupling by mode | `implemented` | `specified` | `yes` | Non-breakpoint random `wnttim`; breakpoint `wnttim=stmstr`. |
| ET wind-mode branch (`iwind`) | `implemented` | `specified` | `yes` | `iwind=0` Penman branch, `iwind=1` non-wind branch selection semantics documented. |
| parser strict/compat mode behavior for `.cli` | `n/a (legacy runtime parser is implicit)` | `implemented` | `yes` | openWEPP parser includes typed strict/compat rules and error taxonomy. |
| parser-to-runtime climate seam ownership | `implicit common-block coupling` | `missing` | `yes` | `HOLD`: no climate runtime adapter seam in orchestrators yet. |
| breakpoint cardinality policy parity | `runtime arrays sized to 1500` | `target match 1500` | `yes` | `DECISION-CLIM01-001`: align parser/runtime to legacy capacity. |
| dewpoint-based winter rain/snow branch | `commented out in baseline` | `do not carry forward` | `yes` | `DECISION-CLIM01-002`: retain active temperature-threshold branch. |
| CLIGEN version policy (`datver`,`iclig`) | `legacy has datver=0 override plus pre-4 and 4+ branches` | `support datver=0 override and 4.0+; guard pre-4 nonzero` | `yes` | `DECISION-CLIM01-003`: carry forward `iclig=0` (`datver=0.0`) and `iclig=1` (`datver>=4.0`) branches; reject `iclig=2` branch by default. |
| breakpoint time monotonicity policy (`timem`) | `legacy zero-drain path can accept non-increasing time` | `strictly increasing breakpoint times required` | `yes` | `DECISION-CLIM01-004`: duplicate/decreasing breakpoint times hard-fail, regardless of `drain`. |
| multi-storm-per-day scientific extension | `known limitation` | `not specified for implementation` | `no` | out-of-scope for CLIM01; retain as limitation signal from climate contract lineage. |
| single-storm climate (`itemp=2`) | `legacy-supported` | `excluded` | `no` | explicit package exclusion and strict parser rejection by default. |
| single-storm modeling path | `legacy-supported` | `excluded` | `no` | explicit package exclusion. |

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/infile.for:1714-1833`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:113-258`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for:61-117`
- `/workdir/wepp-forest_260430_baseline/src/winter.for:205-233`
- `/workdir/wepp-forest_260430_baseline/src/evap.for:200-213`
- `/workdir/wepp-forest_260430_baseline/src/cdiss12.inc:7`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs:9`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs:358-361`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:98-183`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:86-170`
