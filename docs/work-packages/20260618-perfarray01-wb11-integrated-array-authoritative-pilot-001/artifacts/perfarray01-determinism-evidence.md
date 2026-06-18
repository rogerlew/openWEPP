# PERFARRAY01 Determinism Evidence

Evidence class: Static + Ran.

## Stage A

The array shell preserves deterministic ordering:

- `SymbolRegistry` remains frozen and sorted by existing ADR-0022 behavior;
- `ArrayWritebackPayload::with_updates` sorts state and flux updates by
  `SymbolId`;
- `ArrayHotState::export_btreemap_surfaces` iterates registry ids and exports
  to `BTreeMap`, preserving the logical sorted-symbol publication surface;
- the evaluator performs no floating-point reductions and changes no arithmetic
  order.

Ran:

```text
cargo test -p openwepp-kernel-contract
```

The identity tests require exported state/flux maps to match the current
logical writeback path.

## Stage B

Not run. No scheduler sequencing, per-OFE order, H2637 run, or pinned-seed
reproducibility evidence was generated because the valid pilot path does not
exist yet.

## Determinism Verdict

Stage A is deterministic and default-unwired. PERFARRAY01 does not establish
determinism for a WB11 array-authoritative execution path.
