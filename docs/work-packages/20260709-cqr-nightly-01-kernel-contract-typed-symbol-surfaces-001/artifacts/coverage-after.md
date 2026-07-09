# Coverage After

Status: `COMPLETE`

Ran: first after coverage delegated to `comparator_suite_runner`.

Raw LCOV:

- `/tmp/openwepp-cqr-nightly-01-after.lcov` (`4039500` bytes).

Target coverage/CRAP rows:

| Function | Line | Coverage reported by cargo-crap | CRAP |
|---|---:|---:|---:|
| `HillslopeIrrigationDepletionPeriodField::as_str` | 128 | `100.0` | `10.0` |
| `HillslopeIrrigationFixedDateEventField::as_str` | 158 | `100.0` | `8.0` |
| `BoundarySymbol::from` | 262 | `100.0` | `9.0` |
| `hillslope_wb11_state_symbol` | 363 | `92.3076923076923` | `11.055075102412381` |
| `hillslope_wb12_state_symbol` | 380 | `93.75` | `14.0478515625` |
| `hillslope_irrigation_scalar_state_symbol` | 400 | `95.0` | `16.032` |
| `hillslope_plant_hyetograph_soil_state_symbol` | 424 | `93.33333333333333` | `11.035851851851852` |
| `hillslope_snow_frost_state_symbol` | 443 | `95.83333333333334` | `22.035011574074073` |
| `hillslope_peak_method_state_symbol` | 471 | `93.75` | `14.0478515625` |
| `BoundarySymbol::from` | 514 | `100.0` | `17.0` |

All target rows are at or below CRAP `30`.

Final refreshed measurement:

- `comparator_suite_runner` produced refreshed final2 artifacts before its final
  report overflowed:
  - `/tmp/openwepp-cqr-nightly-01-final2.lcov` (`4040147` bytes,
    SHA-256 `42e9434d12ba5b014654cffe0a27eb16becac0d5906464c9c2368d458f7d44e6`)
  - `/tmp/openwepp-cqr-nightly-01-final2-full.json` (`11017446` bytes,
    SHA-256 `c3fda5f7c4a5cbb5e42f475131f88577c8f94c494cee1fe0506414e8d34a84ac`)
  - `/tmp/openwepp-cqr-nightly-01-final2-crap.json` (`2692738` bytes,
    SHA-256 `d1fa13bcf178fac2929fb34a6762d0bbe5c48005dc724341ea21f04980d55671`)
- Local `cargo llvm-cov report --json --output-path
  /tmp/openwepp-cqr-nightly-01-final2-local-full.json` exited `0` and produced
  a byte-identical full JSON export. Package log:
  `artifacts/logs/final-current-llvm-cov-report-json.log`, SHA-256
  `ce16b32dad87f1bd7dc9db3352a50b75314f0d5a75f482b1e26753e061af82ed`.
- Local `cargo crap --workspace --lcov
  /tmp/openwepp-cqr-nightly-01-final2.lcov --min 0 --format json --output
  /tmp/openwepp-cqr-nightly-01-final2-local-crap.json` exited `0` and produced
  a byte-identical CRAP JSON export. Package log:
  `artifacts/logs/final-current-cargo-crap-replay.log`, SHA-256
  `4271b950b449ea18de1d61baf8c3884ef2c0ebaa5fa851e66e525f42f4d1f290`.

Target final coverage:

- LCOV line coverage: `278 / 284 = 97.88732394366197%`.
- LCOV branch fields: `BRF:0`, `BRH:0`; branch fields are not used as ADR-0021
  region evidence for this Rust target.
- Full JSON unique source-region coverage, de-duplicating duplicate
  monomorphized/source-span copies by taking max hit count per region span:
  `332 / 338 = 98.22485207100591%`.
- Deduplicated source functions below the ADR-0021 75% floor: `0`.
- Deduplicated source functions below 90% region coverage: `0`.
- Metrics extraction log: `artifacts/logs/final-current-coverage-metrics.log`,
  SHA-256 `d1d33852232ba2825fd0ba40eaad821eae219d2480eb15df42515be412a8c0ec`,
  `__EXIT_CODE__:0`.
