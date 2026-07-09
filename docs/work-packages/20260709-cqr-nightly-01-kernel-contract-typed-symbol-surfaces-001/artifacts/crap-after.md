# CRAP After

Status: `COMPLETE`

Ran: after metrics were delegated to `comparator_suite_runner`; final CRAP
replay was run locally after the comparator final report overflowed.

Commands:

- First after run:
  - `cargo llvm-cov clean --workspace` exited `0`.
  - `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly-01-after.lcov` exited `0`.
  - `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-01-after.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-01-after-crap.json` exited `0`.
- Final replay:
  - `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-01-final2.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-01-final2-local-crap.json` exited `0`.

Artifacts:

- First after LCOV: `/tmp/openwepp-cqr-nightly-01-after.lcov`
  (`4039500` bytes).
- First after CRAP JSON: `/tmp/openwepp-cqr-nightly-01-after-crap.json`
  (`2692790` bytes).
- Final refreshed LCOV: `/tmp/openwepp-cqr-nightly-01-final2.lcov`
  (`4040147` bytes).
- Final refreshed CRAP JSON:
  `/tmp/openwepp-cqr-nightly-01-final2-crap.json` (`2692738` bytes).
- Local replay CRAP JSON:
  `/tmp/openwepp-cqr-nightly-01-final2-local-crap.json` (`2692738` bytes),
  byte-identical to the final2 CRAP JSON.

Target:
`crates/openwepp-kernel-contract/src/lib_mod/core_types/01_typed_symbol_surfaces.rs`

Final target summary:

- Deduplicated target rows: `24`.
- Rows above CRAP `30`: `0`.
- Rows below cargo-crap coverage `75`: `0`.
- Max CRAP: `22.035011574074073`.

| Function | Line | Cyclomatic | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `hillslope_snow_frost_state_symbol` | 447 | 22 | 95.83333333333334 | 22.035011574074073 |
| `BoundarySymbol::from` | 520 | 17 | 100.0 | 17.0 |
| `hillslope_irrigation_scalar_state_symbol` | 402 | 16 | 95.0 | 16.032 |
| `hillslope_wb12_state_symbol` | 381 | 14 | 93.75 | 14.0478515625 |
| `hillslope_peak_method_state_symbol` | 476 | 14 | 93.75 | 14.0478515625 |
| `BoundarySymbol::from` | 675 | 14 | 100.0 | 14.0 |
| `hillslope_wb11_state_symbol` | 363 | 11 | 92.3076923076923 | 11.055075102412381 |
| `hillslope_plant_hyetograph_soil_state_symbol` | 427 | 11 | 93.33333333333333 | 11.035851851851852 |
| `HillslopeIrrigationDepletionPeriodField::as_str` | 128 | 10 | 100.0 | 10.0 |
| `BoundarySymbol::from` | 262 | 9 | 100.0 | 9.0 |
| `HillslopeIrrigationFixedDateEventField::as_str` | 158 | 8 | 100.0 | 8.0 |
| `WatershedImpoundmentStateField::as_str` | 601 | 8 | 100.0 | 8.0 |
| `WatershedChannelStateField::as_str` | 559 | 6 | 100.0 | 6.0 |
| `BoundarySymbol::from` | 739 | 4 | 100.0 | 4.0 |
| `ClimateForcingSymbolSurface::build` | 53 | 3 | 100.0 | 3.0 |
| `build_series_symbol` | 100 | 3 | 100.0 | 3.0 |
| `ClimateForcingSymbolSurfaceError::fmt` | 85 | 2 | 100.0 | 2.0 |
| `WatershedChannelFluxField::as_str` | 579 | 2 | 100.0 | 2.0 |
| `WatershedImpoundmentFluxField::as_str` | 623 | 2 | 100.0 | 2.0 |
| `ClimateForcingSymbolSurface::hillslope` | 20 | 1 | 100.0 | 1.0 |
| `ClimateForcingSymbolSurface::watershed_hillslope` | 31 | 1 | 100.0 | 1.0 |
| `ClimateForcingSymbolSurface::timem_symbols` | 39 | 1 | 100.0 | 1.0 |
| `ClimateForcingSymbolSurface::intsty_symbols` | 44 | 1 | 100.0 | 1.0 |
| `ClimateForcingSymbolSurface::point_count` | 49 | 1 | 100.0 | 1.0 |

Closure:

- Before: `3` target rows above CRAP `30`; max CRAP `2833.422607238448`.
- Final after: `0` target rows above CRAP `30`; max CRAP
  `22.035011574074073`.
