# PERFARCH01 Risk Register

Status: COMPLETE 2026-06-16
Evidence mode: **Static** design risk review

| Risk | Impact | Mitigation / required gate |
|---|---|---|
| Sorted order drift | Applied-symbol vectors, HBP export, and deterministic diagnostics could change | Assign ids only after sorting by `BoundarySymbol::as_str()`; test id-order export against `BTreeMap` export |
| Lazy interning after freeze | New symbols would break sorted-id order and determinism | No lazy interning in scheduler/kernel paths; unknown post-freeze symbol is a typed error |
| Missing dynamic family registration | Runtime could fail on valid climate/frost/PL/OFE dimensions | Build registry from parsed dimensions; add shape tests for climate point count, `nsl`, PL slots/crops, irrigation events, MOFE hours |
| Failure-path diagnostic drift | Users and contract tests may depend on logical symbol names | Store `symbols_by_id`; diagnostics always map ids back to `BoundarySymbol` |
| Prefix-scan replacement changes guard semantics | Decomposition and PL validation may accept or reject different shapes | Replace scans with explicit family range/count invariants and failure-path tests |
| Compatibility adapters clone maps in hot paths | Performance target missed even if indexed storage exists | Adapters allowed at seams/export only; profiler gate must show hot lifecycle no longer dominated by `BTreeMap::clone_subtree`, `memcmp`, or `format!` |
| Memory blowup from dense optional values | Large symbol universes could increase RSS | Prototype size is small; production stage must record RSS on H2637 and ladder fixtures |
| Floating-point reorder by opportunistic refactor | Bit identity could fail | Storage migration must not change phase order, OFE order, or reductions; anchor gate is `anchor_mismatches = 0` |
| Watershed/hillslope divergence | Two runtime surfaces could evolve incompatible semantics | Registry and indexed surface types belong in shared contract/runtime support, with hillslope and watershed adapters |
| Unclear extension-symbol ownership | Later packages may add ad hoc symbols | Stage 1 must define a pre-freeze extension registration API and reject post-freeze additions |
