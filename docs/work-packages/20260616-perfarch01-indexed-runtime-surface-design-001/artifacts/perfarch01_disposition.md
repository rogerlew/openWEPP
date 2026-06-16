# PERFARCH01 Disposition

Status: COMPLETE 2026-06-16
Evidence mode: **Ran** + **Static**

## Outcome

PERFARCH01 is complete as an architecture design and feasibility package. It
lands no production Rust and no science contract change.

The selected design is a frozen, run-scoped `SymbolRegistry` with sorted-order
`SymbolId`s and dense indexed state/flux surfaces. The logical
`BoundarySymbol` interface remains available, while hot paths migrate to
pre-resolved ids.

## Feasibility

Prototype evidence supports the architecture:

- dense clone: 109.85x faster than `BTreeMap<String, f64>` clone,
- dense pre-resolved lookup: 219.16x faster than formatted string lookup,
- dense update batch: 115.77x faster than clone+insert batch,
- sorted id order matched string sort.

The <=10x target is plausible if staged implementation migrates about 89-90% of
current elapsed time out of string-keyed surface mechanics. The <=5x target
requires about 95-96% effective migration and is not a storage-only promise.

## Deliverables

- `artifacts/indexed-runtime-surface-design.md`
- `artifacts/feasibility-and-projected-speedup.md`
- `artifacts/staged-implementation-plan.md`
- `artifacts/risk-register.md`
- `artifacts/prototypes/indexed_surface_microbench.rs`
- `docs/decisions/0022-indexed-runtime-surface-representation.md`
- `artifacts/perfarch01-worker-handoff.md`

## Closure

Next stage: `PERFIDX01-run-scoped-symbol-registry-001`.
