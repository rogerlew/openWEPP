# PL03 Runtime Adapter Contract

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- `PL-MAN-SEAM-001` requires strict typed parser-to-runtime projection from `ManagementParseOutput` with no silent defaults.
- Scheduler-facing outputs must preserve PL ordering prerequisites and canonical symbol continuity.

Ran:
- Implemented strict PL management runtime adapter in `openwepp-hillslope-orchestrator`.
- Added typed PL runtime surface family struct and merged runtime-surface builder.
- Implemented strict typed rejects for topology/schedule closure, branch/domain policy, and non-finite required controls.

## Contract Closure

| contract facet | implementation outcome |
|---|---|
| seam id | `PL-MAN-SEAM-001` |
| owner | `openwepp-hillslope-orchestrator::runtime_inputs` |
| input type | `ManagementParseOutput` only |
| output type | `HillslopePlRuntimeSurfaces` (`pl_schedule_surface`, `pl_growth_surface`, `pl_decomp_surface`) and merged `HillslopeWritebackSurface` |
| failure policy | typed `HillslopeRuntimeInputError` (`HS-RUNTIME-E-036..045`) |
| defaulting policy | no fallback defaults for required projected fields |
| branch policy | explicit typed reject for unsupported `landuse`; explicit typed reject for unsupported perennial `mgtopt` |

## Scope Notes

1. PL03 closes parser-to-runtime PL adaptation and strict seam error taxonomy.
2. PL03 does not claim kernel behavior closure for growth/decomposition phases.
3. PL03 retains executable profile policy where unsupported `landuse` branches fail typed.

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:677`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:702`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1119`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:159`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:248`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-seam-requirements.md`
