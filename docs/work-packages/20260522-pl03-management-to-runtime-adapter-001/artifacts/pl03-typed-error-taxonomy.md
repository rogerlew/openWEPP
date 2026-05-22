# PL03 Typed Error Taxonomy

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- PL03 requires stable typed seam errors that distinguish closure/domain/finite-state failures.

Ran:
- Extended `HillslopeRuntimeInputError` with PL seam variants and stable IDs.

## PL Seam Error Family (`HS-RUNTIME-E-036..045`)

| code | variant | failure class |
|---|---|---|
| `HS-RUNTIME-E-036` | `ManagementTopologyCountMismatch` | topology/schedule closure mismatch |
| `HS-RUNTIME-E-037` | `ManagementScheduleSlotCountMismatch` | expected slot count closure mismatch |
| `HS-RUNTIME-E-038` | `ManagementScheduleSlotArityMismatch` | slot crop-slot arity mismatch |
| `HS-RUNTIME-E-039` | `ManagementInitialReferenceOutOfRange` | dangling or zero initial reference |
| `HS-RUNTIME-E-040` | `ManagementYearlyReferenceOutOfRange` | dangling or zero yearly reference |
| `HS-RUNTIME-E-041` | `UnsupportedPlLanduse` | unsupported landuse branch for executable profile |
| `HS-RUNTIME-E-042` | `UnsupportedPlManagementOption` | unsupported perennial option domain |
| `HS-RUNTIME-E-043` | `NonFinitePlProjectionField` | non-finite required projected control/value |
| `HS-RUNTIME-E-044` | `PlProjectionCountOutOfRange` | projection cardinality conversion overflow guard |
| `HS-RUNTIME-E-045` | `ManagementScheduleOfeIndexOutOfRange` | schedule slot OFE index out of declared range |

## Negative-Path Test Coverage

1. `HS-RUNTIME-E-039` asserted by out-of-range initial reference test.
2. `HS-RUNTIME-E-041` asserted by unsupported landuse test.
3. `HS-RUNTIME-E-043` asserted by non-finite `rw` test.
4. `HS-RUNTIME-E-042` asserted by unsupported perennial `mgtopt` test.

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:159`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:248`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:551`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:2310`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:2332`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:2353`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:2381`
