# Runtime Input Surface Classification

Evidence mode: `Static`
Status: `complete`

## Runtime-Consumed (Authoritative)

| parser surface | runtime destination | symbols | rationale |
|---|---|---|---|
| `SoilProfile` primary OFE depth/theta fields | hillslope writeback `state_surface` | `solthk`, `dg`, `thetdr`, `thetfc` | Required initial-state semantics for soil depth and moisture-domain runtime context. |
| `ChaninpFile` parsed branch options | watershed writeback `state_surface` + `flux_surface` | `ipeak`, `nchan`, `dtchr`, `ntchr`, `nchnum`, `cbase` | Required watershed channel-routing timestep/baseflow control inputs. |

## Derived / Compatibility / Control-Only (Non-Authoritative)

| parser surface | class | handling |
|---|---|---|
| `SoilProfile.datver_alias_applied` | compatibility provenance | retained for diagnostics; not promoted to runtime state symbols. |
| `ChaninpFile.warnings`, `trailing_token_lines` | compatibility/provenance | retained for evidence only. |
| `ChaninpFile.parse_outcome != ParsedBranch` | gate control | explicit runtime rejection (`WS-RUNTIME-E-001`) instead of fallback ingestion. |

## Out of Scope for ARCH17 Runtime Consumption
- HBP parser/bridge authority convergence surfaces (`ARCH18` owner).
- Remaining parser families without implemented runtime adapter in this package (`ARCH19+` sequencing for top-level `.run`/parquet and additional cross-file closures).

## Evidence
- [DIRECT] `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:107-183`
- [DIRECT] `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:96-170`
- [DIRECT] `docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/disposition-register.md:19-24`
