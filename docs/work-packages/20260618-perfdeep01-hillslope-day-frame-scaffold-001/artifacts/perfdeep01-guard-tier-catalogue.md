# PERFDEEP01 Guard Tier Catalogue

Evidence: Static.

## Scope and Exhaustive Site Inventory

Production bounded-writeback constructor sites were enumerated with:

```bash
rg -n "WritebackField::bounded|IndexedWritebackField::bounded" crates/openwepp-hillslope-orchestrator/src --glob '!**/tests/**'
```

Observed inventory size:

- `236` production callsites
- Full source list artifact: `artifacts/perfdeep01-writeback-bounded-sites-production.txt`

(Reference count including test modules: `247`.)

## Two-Tier Guard Policy (Stage-0 Contract)

Every bounded writeback site is assigned to exactly one tier:

1. `STATIC_BOUND`
- Bound expressions are literals or compile-time constants (`None`, `Some(0.0)`, `Some(1.0)`, `Some(WB16_MAX_DURATION_S)`, etc.).
- Migration rule: encode as frame-field invariant/schema contract at write site.

2. `RUNTIME_DERIVED_BOUND`
- Any min/max expression depends on runtime state, flux, layer arrays, lane carry values, geometry, or computed intermediates.
- Migration rule: preserve as explicit runtime checks at the phase write site; do not collapse to schema-only validation.

This policy is the Stage-0 closure for the review finding that guard semantics cannot be treated as compile-time-only.

## Representative Static-Bound Sites

- `WB11_SYMBOL_WS` in plant/percolation with `Some(0.0)..Some(1.0)`.
- `WB16_SYMBOL_WATDUR` in peak runoff with `Some(0.0)..Some(WB16_MAX_DURATION_S)`.
- Binary lineage/flag symbols written as `[0,1]` bounds.

## Representative Runtime-Derived-Bound Sites

- Layer uptake constrained by per-layer runtime potential (`Some(layer_potential_uptake[layer_index])`).
- EROD18 segment coordinates constrained by runtime segment geometry (`segment.xu`, `segment.xl`).
- Lateral unrealized flow constrained by runtime target totals.
- Tile drainage constrained by runtime drainage capacity.

## Diagnostic Attribution Contract (Preserved)

Guard failures must continue to propagate through typed hydrology guard errors with phase/symbol attribution.
Key invariant for Stage-1+: changing storage representation must not alter:

- failure classification (`MissingRequiredInput`, `NonFinite`, domain/range violation)
- message-id family attribution used in kernel diagnostics
- symbol-oriented rejection context at failure boundaries

## Stage-0 Disposition for Guard Migration

- Stage-0 does not migrate guard execution into frame-writes.
- Stage-0 codifies the two-tier policy and captures exhaustive source inventory so Stage-1 phase migrations can preserve guard semantics without hidden seam loss.
