# Climate Implementation Work-Package Queue

Status: `complete`
Evidence mode: `Ran + Static`

Static:
- Queue built from CLIM01 gap register and current openWEPP parser/orchestrator state.

Ran:
- Validated current parser/runtime seam coverage and climate parser implementation presence.

| wp_id | title | objective | dependencies |
|---|---|---|---|
| `CLIM02` | Climate Parser-to-Runtime Seam Adapters | Implement `HS-CLIM-SEAM-001` and `WS-CLIM-SEAM-001` adapters with typed `CLIM-RUNTIME-E-*` taxonomy, immutable runtime climate surfaces, explicit `datver=0.0` override + `datver>=4.0` guard enforcement, and strict breakpoint `dtime>0` guard enforcement. | `SC-INFILE-CLIMATE-001`, ARCH17 seam pattern |
| `CLIM03` | Continuous-Daily Climate Runtime Kernel Port | Port baseline non-breakpoint event normalization and disaggregation semantics (`iclig`, unit conversions, `mxint`, invalid-duration behavior) into openWEPP climate runtime module, carrying forward `iclig=0` (`datver=0.0`) and `iclig=1` (`datver>=4.0`) only. | `CLIM02` |
| `CLIM04` | Breakpoint Runtime Port and Policy Reconciliation | Port breakpoint event semantics (`stmstr`, elapsed-time normalization, interval-intensity build) and implement ratified breakpoint policies (`1500` cardinality target and strict `dtime>0` interval guard) with explicit compat controls. | `CLIM02`, `CLIM03` |
| `CLIM05` | Winter Coupling Closure | Integrate climate runtime outputs with winter hourly partition contracts under the ratified policy to keep the active temperature-threshold branch (no dewpoint-branch carry-forward). | `CLIM03`, `CLIM04`, `SC-SNOWFREEZE-001` |
| `CLIM06` | ET/Water Balance/Irrigation Consumer Closure | Wire climate forcing payloads to ET, water balance, and irrigation guards with typed invariant checks and no fallback defaults. | `CLIM03`, `CLIM04`, `SC-EVAP-001`, `SC-WATBAL-001`, `SC-IRRIG-001` |
| `CLIM07` | Climate Comparator and Closure Evidence | Add targeted integration tests and comparator vectors for continuous-daily and breakpoint modes, including parser-to-kernel seam checks and legacy confidence-tier reporting. | `CLIM03`..`CLIM06` |
| `CLIM08` | Climate Governance Disposition Closeout | Close remaining CLIM HOLD items (`parser/runtime seam`, climate seam integration-test closure) and update climate contracts/specs to promotable status. | `CLIM07` |

## Queue Notes

1. `CLIM02` is the highest-priority blocker: parser output is present, but runtime seam ownership is not yet implemented.
2. `CLIM04` must implement the ratified parser/runtime cardinality target (`1500`) as explicit behavior, not an implicit runtime artifact.
3. `CLIM07` should reuse ARCH17 integration-test patterns for deterministic seam evidence.
