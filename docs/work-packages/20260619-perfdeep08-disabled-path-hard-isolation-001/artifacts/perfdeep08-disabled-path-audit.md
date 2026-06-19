# PERFDEEP08 Disabled-Path Audit

Status: queued.
Evidence mode: not run.

Audit every default-disabled path that can construct or resolve opt-in-only
machinery:

- dense surfaces and lane dense state;
- indexed shadow surfaces and scheduler resources;
- hot symbol tables and lookup maps;
- kernel request dense views;
- writeback payload surfaces;
- frame-shadow state;
- `SymbolRegistry::id_of` or symbol-name lookup;
- dense/logical refresh or flush work.

Record source locations, trigger conditions, disabled-state behavior, and
candidate isolation action.
