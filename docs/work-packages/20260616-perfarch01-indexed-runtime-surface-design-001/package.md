# PERFARCH01 — Indexed Runtime-Surface Representation (Design + Feasibility)

Status: complete 2026-06-16 (operator-directed target ≤10× vs legacy, ≤5× if
possible; architectural path chosen over incremental PERFOPT passes)

Package type: **Architecture design + feasibility** (no production or contract
change in *this* package — it produces a design, a proposed ADR, a staged
bit-identity-gated implementation plan, and a quantified feasibility verdict;
implementation is staged follow-on packages).

## Objective

Design the replacement of openWEPP's string-keyed runtime surface
(`BTreeMap<BoundarySymbol, BoundaryValue>`) with an **indexed/array-backed**
representation that preserves the `BoundarySymbol` logical interface, bit-identical
outputs, and within-config determinism — the architectural change required to take
the single-hillslope wall-clock from **~85× vs legacy toward ≤10× (≤5× if
feasible)**. Three profiling rounds (PERFHO01, PERFOPT01 re-check, PERFHO02 —
the last with 9,586 `perf` samples) put **~95 %** of openWEPP's time in per-OFE-day
symbol-surface machinery and **0** in physics output: legacy does the same physics
in ~10 s with fixed arrays. Incremental passes (PERFOPT01 = 1.15×) are Amdahl-capped
well above 10× because the cost is *distributed across every state access*, not a
few excisable functions.

**This package de-risks before committing implementation effort.** It validates
the design is feasible and bit-identity-preserving, quantifies the projected
speedup, and stages the work — it does not land the change.

## Root cause (PERFHO0x + audit)

- `BoundarySymbol(String)` (`openwepp-kernel-contract/src/lib_mod/core_types.rs:25`):
  every key is a heap `String`. Every lookup (`runtime_surface_symbol_value`)
  allocates a `String` from `&str` and does a `memcmp` `BTreeMap` walk; every
  per-OFE-day surface **clone** (scheduler `to_execution_input` / `update_from_report`,
  ~14 phases × N OFEs/day) deep-copies all `(String, BoundaryValue)` pairs; hot
  hydrology paths **`format!`-construct** indexed symbols (`frost_layer_symbol`,
  `wb18_perc_*_{layer:04}`, PL dispatch, climate `timem/intsty_{i}`) on every
  OFE-day.
- `perf`: `execute_persistent_scheduler_kernel_lifecycle` 96.24 % children;
  `run_hillslope_phase` 41.14 %; `run_runoff_reconciliation` 22.40 %;
  `apply_kernel_writeback` 12.46 %; `compute_active_frost_coupling` 12.35 %;
  decomposition guard scan 7.48 %.
- Symbol universe ~4–6K/run (climate ≤3K, soil 50–100, frost 100–500, PL 50–500,
  static ~100) — fits a `u32`/`u16` index. **No central symbol registry exists.**

## Proposed design (architecture guidance — validate + refine in this package)

Separate the **logical** symbol interface (`BoundarySymbol`, unchanged at the API)
from the **physical** storage (indexed). Three pieces:

1. **Symbol registry / interner.** A run-scoped, immutable `SymbolRegistry`
   mapping each `BoundarySymbol` → a dense `SymbolId`. Built once at parse/projection
   (where the symbol universe — `nsl`, climate `point_count`, PL slots — is known).
   **Assign ids in sorted-symbol order** so `id` order ≡ sorted-`as_str()` order
   (this is the key trick that preserves the sorted-iteration semantics cheaply).
   Dynamic indexed symbols are pre-registered from their known counts, not
   lazily-interned mid-run (lazy interning would break sorted-id order — an open
   question to resolve).
2. **Indexed store.** Back the surface with a `Vec<Option<BoundaryValue>>` indexed
   by `SymbolId` (dense; ~4–6K × ~16 B ≈ ~100 KB) — O(1) get/set, and **clone =
   `Vec` memcpy, no `String` dup**. (Sparse alternative: sorted `Vec<(SymbolId,
   BoundaryValue)>` + binary search — evaluate memory vs speed.)
3. **Resolve-once access.** Hot paths that rebuild indexed symbols per OFE-day
   resolve their `SymbolId`s once (via a typed `(root, layer/slot/index)` → id
   table) instead of `format!`+hash every time — removing the construction cost
   too.

The `BoundarySymbol` API (`get`/`insert`/`iter`/the writeback payload) stays; only
the storage + hot-path resolution change.

## Bit-identity hazards the design must preserve (from the audit)

| Hazard | Site | Preservation strategy to validate |
|---|---|---|
| `apply_kernel_writeback` sorts `state/flux_updates` by `symbol.as_str()` and exposes `applied_*_symbols` in that order | `writeback.rs:116-156` | sorted-order ids ⇒ sort-by-id ≡ sort-by-string; map id→`BoundarySymbol` for the exposed vectors |
| Decomposition overflow guard scans `state_surface.keys()` for a string prefix | `07_decomposition_equations.rs:580` | replace prefix-scan with a typed id-range check; must produce identical accept/reject |
| HBP directory key order asserted deterministic/strict | `tests/integration/infile_hbp_parser_contract.rs:1028` | emit in the same (sorted) order |
| Dynamic indexed symbols (frost/PL/WB18-19/climate) | `state_access.rs`, `05_projection_helpers.rs`, `core_types.rs` | pre-register deterministically; ids stable run-to-run |
| Determinism (`docs/numerics/`) | — | no FP-reduction reorder, no per-OFE sequencing change, pinned-seed bit-reproducibility |

## Milestones

1. **Audit completion** — confirm the symbol-universe cardinality + the per-OFE-day
   clone/lookup/construction cost *share* (so the projected speedup is grounded),
   and read **ARCH16** (`20260522-arch16-scheduler-hot-path-surface-optimization-001/`)
   for prior surface-clone work it may already cover or have rejected.
2. **Registry + id-assignment design** — sorted-order id assignment; how dynamic
   symbols are pre-registered deterministically; the `(root, index)`→id tables.
3. **Indexed-store prototype** — a throwaway prototype (dense `Vec<Option<…>>`) on
   one hot path (e.g. the per-OFE-day clone + `runtime_surface_symbol_value` +
   the frost layer-state lookups) to **measure** the clone/lookup savings.
4. **Bit-identity + hazard validation** — prove the sorted-iteration hazards above
   are preserved by the design (esp. `apply_kernel_writeback` ordering and the
   decomposition guard).
5. **Feasibility verdict** — a quantified projected speedup (can we credibly reach
   ≤10×? ≤5×?), or an honest "the floor is X×" with the reason.
6. **Staged implementation plan** — incremental, each stage a behavior-preserving
   optimization package with the `anchor_mismatches = 0` gate (suggested: registry
   build → indexed backing under the unchanged API → hot dynamic-symbol resolve-once
   → sorted-iteration-op migration).
7. **Proposed ADR** — draft `docs/decisions/0022-indexed-runtime-surface-representation.md`
   (the decision, rationale, the `BoundarySymbol`-API-preserved + bit-identity
   constraints, and the staging), for ratification before Stage-1 implementation.

## Acceptance criteria

- A design doc + the proposed ADR + the staged plan + the **feasibility verdict
  with a quantified projected speedup** (prototype-measured, `Ran:`), + the risk
  register covering every hazard above.
- No production or contract change in this package; the eventual implementation's
  gate is `anchor_mismatches = 0` (bit-identity) + determinism.

## Deliverables

- `artifacts/indexed-runtime-surface-design.md`
- `artifacts/feasibility-and-projected-speedup.md` (prototype measurements)
- `artifacts/staged-implementation-plan.md`
- proposed `docs/decisions/0022-indexed-runtime-surface-representation.md` (draft)
- `artifacts/perfarch01_disposition.md` + handoff (naming Stage-1).

## Dependencies

- PERFHO01/PERFHO02 + PERFOPT01 (the attribution, perf data, the audit).
- ARCH16 (`20260522-arch16-scheduler-hot-path-surface-optimization-001/`) — prior
  scheduler-surface optimization.
- `docs/numerics/README.md` (determinism); ADR-0003 (within-config bit-repro);
  ADR-0011 (architecture-first); `AGENTS.md`, `docs/codex_exec_plans.md`.
- The runtime-surface code: `openwepp-kernel-contract/src/lib_mod/{core_types,writeback}.rs`,
  `openwepp-hillslope-orchestrator/src/scheduler.rs`,
  `.../hydrology/support_helpers_mod/state_access.rs`,
  `.../runtime_inputs/05_projection_helpers.rs`.

## Autonomy

Execute the design + feasibility end-to-end (audit → registry/id design →
indexed-store prototype + measurement → hazard validation → feasibility verdict →
staged plan → draft ADR) without asking for direction on intermediate steps. This
package **lands no production or contract change**. If the feasibility verdict is
that ≤10× is not reachable even with the indexed surface, that is an honest
finding — say so with the floor and the reason, do not force the target.
