# Climate Runtime Error Taxonomy

Evidence mode: `Static`
Status: `complete`

## Typed Codes

| code | class | trigger | seam(s) | source enum variant(s) |
|---|---|---|---|---|
| `CLIM-RUNTIME-E-001` | policy/version | unsupported climate version branch (`0.0<datver<4.0`) | hillslope + watershed | `UnsupportedDatver` |
| `CLIM-RUNTIME-E-002` | policy/mode | unsupported single-storm mode at seam (`itemp!=1`) | hillslope + watershed | `UnsupportedItemp` |
| `CLIM-RUNTIME-E-003` | structural/closure | daily forcing collection is empty | hillslope + watershed | `EmptyDailyRecords` |
| `CLIM-RUNTIME-E-004` | structural/index | requested day index exceeds available records | hillslope + watershed | `DayIndexOutOfRange` |
| `CLIM-RUNTIME-E-005` | numeric/non-finite | non-finite required climate scalar | hillslope + watershed | `NonFiniteField` |
| `CLIM-RUNTIME-E-006` | numeric/domain | negative value in non-negative climate field | hillslope + watershed | `NegativeField` |
| `CLIM-RUNTIME-E-007` | runtime guard | positive precipitation with non-positive event duration | hillslope + watershed | `PositivePrecipWithNonPositiveDuration` |
| `CLIM-RUNTIME-E-008` | structural/closure | breakpoint record has zero points | hillslope + watershed | `EmptyBreakpointSeries` |
| `CLIM-RUNTIME-E-009` | runtime guard | duplicate/decreasing breakpoint time (`dtime<=0`) | hillslope + watershed | `NonMonotoneBreakpointTime` |
| `CLIM-RUNTIME-E-010` | runtime guard | breakpoint interval violates strict elapsed-time guard during intensity derivation | hillslope + watershed | `PositiveBreakpointDrainWithNonPositiveDeltaTime` |
| `CLIM-RUNTIME-E-011` | conversion/range | breakpoint count cannot be represented in lossless runtime projection | hillslope + watershed | `BreakpointCountOutOfRange` |
| `CLIM-RUNTIME-E-012` | structural/ownership | no watershed climate assignment payload supplied | watershed only | `EmptyClimateAssignments` |

## Taxonomy Notes
1. Codes are adapter-surface failures, not parser-local grammar failures.
2. Parser-local climate errors remain in `ClimateParseError`; adapter errors are emitted only after parser output enters runtime seam projection.
3. Watershed assignment-specific ownership closure (`E-012`) is intentionally separate from parser concerns.

## Evidence
- [DIRECT] `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:166`
- [DIRECT] `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:183`
- [DIRECT] `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:160`
- [DIRECT] `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:209`
