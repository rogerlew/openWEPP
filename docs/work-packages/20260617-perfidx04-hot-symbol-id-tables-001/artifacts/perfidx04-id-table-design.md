# PERFIDX04 Id Table Design

Static:
- Added `HotSymbolTables` resolve-once tables in `openwepp-kernel-contract`, populated from the frozen `SymbolRegistry`.
- Added sparse id tables for exact scalars, `root_0001` series, `root_0001_0001` grids, and PL slot/crop naming patterns.
- Added `IndexedWritebackSurface` mutation helpers so the indexed mirror is synchronized after accepted logical writeback payloads without changing payload shape.

Static:
- `HillslopeKernelRequest` now accepts optional indexed state/flux surfaces plus optional hot tables.
- The scheduler carries the indexed execution mirror beside the logical `BTreeMap` surface, passes both into the request, and applies logical writebacks to both surfaces after acceptance.
- Persistent OFE execution rebuilds the indexed mirror after seed/climate/PL prep and then keeps it synchronized across same-day OFE transfer mutations.

Static:
- Logical `BoundarySymbol` names remain present in hot table entries and are still used in error construction.
- Unknown symbol behavior remains fail-closed via `SymbolRegistryError`.
- No public writeback payload shape changed.
