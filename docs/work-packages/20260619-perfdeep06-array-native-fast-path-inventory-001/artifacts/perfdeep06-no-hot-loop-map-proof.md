# PERFDEEP06 No-Hot-Loop-Map Proof

Status: complete 2026-06-19.
Evidence class: Static + Ran.

## Current Mechanism Proof

Static evidence shows why the current path is not the ADR-0025 target:

- `HillslopeKernelRequest` still carries `state_surface` and `flux_surface`
  `BTreeMap` borrows plus optional dense/indexed views.
- `state_access.rs` reads `hot_*` indexed values first, dense slots second, then
  falls back to logical maps.
- `HillslopeLaneDenseState::apply_kernel_writeback_payload` resolves each
  logical `WritebackField` through `symbol_registry.id_of`.
- `HillslopeLaneDenseState::refresh_cached_slots_from_writeback_surface` and
  `flush_dirty_to_writeback_surface` bridge to logical/indexed surfaces.
- `KernelWritebackPayload` and `IndexedKernelWritebackPayload` are `Vec`-backed
  payloads of state/flux update fields.
- WB13/HBP publication helpers read runtime surfaces through string symbols.

Ran:

- `rg` found 451 symbol/runtime-surface/writeback access sites across inspected
  hot-path and publication scopes.

## PERFDEEP07 Absence Checklist

The migrated normal success path must have zero direct use of:

- `BoundarySymbol`
- `BoundaryValue`
- `SymbolRegistry::id_of`
- `HillslopeKernelRequest`
- `KernelWritebackPayload`
- `IndexedKernelWritebackPayload`
- `HillslopeWritebackSurface`
- `state_surface.get`, `flux_surface.get`, `insert`, or `remove`
- `state_value_for_symbol`, `flux_value_for_symbol`
- `runtime_surface_symbol_value`, `require_runtime_surface_scalar`
- `refresh_cached_slots_from_writeback_surface`
- `flush_dirty_to_writeback_surface`
- `perfdeep02_apply_logical_frame_writeback`
- `HotSymbolTables::hot_state_symbols`, `hot_flux_symbols`
- `format!`/owned symbol construction for hot field access

The default-disabled path must additionally prove zero construction or
resolution of dense/indexed compatibility state unless a PERFDEEP opt-in or
shadow validation mode is active. This is load-bearing because PERFDEEP05
default-disabled measured `701.95 s` versus `669.97 s`; the compatibility
bridge cannot remain an always-on tax on `main`.

The static proof must be paired with the PERFDEEP07 disabled-path timing gate:
at least three clean H2637 no-UI runs, all PERFDEEP opt-ins disabled,
min/median/max/RSS recorded, same-machine control reported, and median
`<= 676.67 s` (`669.97 s + 1%`) unless an external environment change is
hard-attributed.

## Allowed Remaining Uses

- Initial seed into a direct frame before the migrated chain starts.
- Shadow comparison against the logical path while the package remains opt-in.
- Non-migrated boundary adapters if PERFDEEP07 has a smaller-than-all-phases
  scope and explicitly pays that edge outside the migrated success path.
- Output serialization, replay, indexed shadow diagnostics, and manifest
  provenance outside per-OFE-day phase execution.
- Tests that build logical fixtures for identity comparison.
- Explicit opt-in or shadow-validation code paths whose environment switch is
  off by default.

## Gate

PASS. This artifact is the PERFDEEP07 review checklist for map/symbol/writeback
absence.
