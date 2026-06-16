# PERFARCH01 Worker Handoff

Status: COMPLETE 2026-06-16
Evidence mode: **Static** + **Ran**

## Summary

PERFARCH01 chose the indexed runtime-surface architecture and drafted ADR-0022.
The design keeps the logical `BoundarySymbol` seam but makes storage dense and
id-backed. Sorted-id order is the compatibility invariant.

## Stage-1 Package

Open:

```text
PERFIDX01-run-scoped-symbol-registry-001
```

Stage-1 scope:

- add `SymbolId` and frozen `SymbolRegistry`,
- build the registry from existing runtime surfaces,
- prove id order equals sorted string order,
- add BTreeMap export/equality adapters,
- do not make indexed storage authoritative yet.

## Hard Constraints

- No lazy interning after freeze.
- Unknown post-freeze symbols fail closed.
- Every diagnostic still names the logical `BoundarySymbol`.
- No FP reduction, phase-order, or OFE-order change.
- Gate production stages with `anchor_mismatches = 0` and determinism.

## Evidence To Reuse

- Prototype: `artifacts/prototypes/indexed_surface_microbench.rs`
- Design: `artifacts/indexed-runtime-surface-design.md`
- Projection: `artifacts/feasibility-and-projected-speedup.md`
- Plan: `artifacts/staged-implementation-plan.md`
- Risk register: `artifacts/risk-register.md`
