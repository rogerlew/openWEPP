# No-Compatibility Proof Plan

Status: complete for planning-only scope.
Evidence mode: Static/Ran.

## Static Inventory

Ran:

```text
rg -n "execute_with_kernel|HillslopeKernelRequest|KernelWritebackPayload|HillslopeWritebackSurface|state_value_for_symbol|flux_value_for_symbol|SymbolRegistry::id_of|build_registry_for_run|HotSymbolTables|IndexedWritebackSurface" crates/openwepp-hillslope-orchestrator/src crates/openwepp-runner/src/hillslope crates/openwepp-kernel-contract/src/lib_mod/core_types
```

The current runtime still contains these compatibility mechanisms in scheduler,
day-frame, runner, and kernel-contract paths. Future direct-mode packages must
prove they do not enter those paths during direct phase execution.

## Required Proof Method

Future direct-mode implementation packages must provide:

- direct executor entrypoint allowlist;
- static call-graph audit proving direct execution does not call
  `execute_with_kernel*`, `HillslopeKernelRequest`,
  `KernelWritebackPayload`, `HillslopeWritebackSurface`,
  `state_value_for_symbol`, `flux_value_for_symbol`, registry/hot-table,
  indexed-surface, dense-refresh, or dirty-flush paths;
- H2637 runtime counters or audit hooks showing zero calls to the forbidden
  APIs during direct phase execution;
- zero `BoundarySymbol` or owned legacy-symbol construction in direct phase
  execution;
- explicit allowance list for edge-only compatibility adapters before or after
  direct execution.

## Gate

PASS for planning-only scope. No direct-mode implementation exists in this
package.
