# PERFDEEP07 Disabled-Path Audit

Status: HOLD.
Evidence mode: Static/Ran.

## Findings

Default-disabled execution was not paying direct-frame construction, but it was
still paying dense/indexed compatibility lookup cost in hot hydrology reads.

- `HillslopeKernelRequest::indexed_state_value` and `indexed_flux_value`
  checked dense slot views and compact dense slots before falling through to the
  indexed surface even when no dense surface was active.
- `Wb11HydrologyKernel::state_value_for_symbol` and `flux_value_for_symbol`
  performed hot/dense lookup attempts before logical-map fallback.
- `HotSymbolTables` used `BTreeMap<String, _>` for repeated hot scalar,
  series, and grid lookups.
- Runner persistent execution still needs the production indexed runtime
  surface for the current default path; removing it entirely measured worse
  (`753.38 s` and `755.48 s`) and is not the correct zero-cost fix.

## Patch Applied

- Added dense-absent fast paths in
  `HillslopeKernelRequest::indexed_state_value` and
  `indexed_flux_value`.
- Added hydrology state/flux access bypasses for requests with no hot tables,
  registry, indexed surface, or dense surface.
- Split dense-surface presence from indexed-surface presence with
  `has_dense_state_surface` and `has_dense_flux_surface`.
- Changed `HotSymbolTables` hot lookup maps from `BTreeMap` to `HashMap`.
- Made runner scheduler lifecycle references optional/fail-closed so explicit
  diagnostics fail clearly if requested without the required registry.

## Stop Reason

The retained patch improved the PERFDEEP05 default-disabled regression but did
not pass the P0 timing gate. Direct-frame implementation was not started.
