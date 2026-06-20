# R2A No-Compatibility Proof Checklist

Status: complete.
Evidence mode: Static + Ran.

Forbidden in direct skeleton execution unless explicitly listed as edge-only:

| Surface | Result | Evidence |
|---|---|---|
| `execute_with_kernel*` | PASS | Direct runtime source scan returned no matches; no compatibility scheduler diff. |
| `HillslopeKernelRequest` | PASS | Direct runtime source scan returned no matches. |
| `KernelWritebackPayload` | PASS | Direct runtime source scan returned no matches. |
| `HillslopeWritebackSurface` | PASS | Direct runtime source scan returned no matches. |
| `state_value_for_symbol` / `flux_value_for_symbol` | PASS | Direct runtime source scan returned no matches. |
| `SymbolRegistry` / `SymbolRegistry::id_of` | PASS | Direct runtime source scan returned no matches. |
| `HotSymbolTables` | PASS | Direct runtime source scan returned no matches. |
| `IndexedWritebackSurface` | PASS | Direct runtime source scan returned no matches. |
| dense refresh / dirty flush | PASS | Direct runtime source scan returned no matches for `dense` or `dirty`; no scheduler diff. |
| owned legacy-symbol construction in direct execution | PASS | Direct runtime module imports only `std::error::Error`, `std::fmt`, and atomics; no `BoundarySymbol` or registry import. |

## Pre-Implementation Inventory

Static:

The forbidden APIs exist in the compatibility scheduler, day-frame, runner
setup/trace, and kernel-contract paths. R2A proof is therefore scoped to the
new direct runtime entrypoint and direct skeleton execution, not to the entire
repository.

Allowlist for direct skeleton execution:

- `DirectFrameExecutor::new`
- `DirectFrameExecutor::run_skeleton`
- `DirectRunFrame::skeleton`
- `DirectDayFrame::seed`
- `DirectDayFrame::phase_view`

No edge-only compatibility adapters are required for this skeleton.

## Executed Proof Commands

Ran:

- `rg -n "execute_with_kernel|HillslopeKernelRequest|KernelWritebackPayload|HillslopeWritebackSurface|state_value_for_symbol|flux_value_for_symbol|SymbolRegistry|HotSymbolTables|IndexedWritebackSurface|dense|dirty|build_registry_for_run" crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
  exited with no matches.
- `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`
  passed.
- `cargo test -p openwepp-runner r2a_ -- --nocapture` passed.
- `git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  produced no diff.

The compatibility APIs still exist in the repository's compatibility runtime.
This checklist is scoped to R2A direct skeleton execution and storage.

## Review Correction

Static:

- Review found the initial reserved `forbidden_compatibility_calls` and
  `compatibility_surface_constructions` counters were tautological because no
  compatibility entrypoint incremented them.
- Those counters were removed. R2A now uses static source/call-graph proof for
  forbidden compatibility calls and runtime counters only for claims they can
  prove: default-disabled direct skeleton inactivity and explicit opt-in
  skeleton execution.
