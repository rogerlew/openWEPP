# PERFIDX05 Kickoff — Writeback And Guard Migration (Stage 5)

Execution mode: package-end-to-end (behavior-preserving optimization; bit-identity AND
failure-path tests are load-bearing).

Autonomy: execute end-to-end (inventory → per-change incremental migration → bit-identity
→ failure-path tests → determinism → prefix-range proofs → realized speedup → gates →
dual review/verification → disposition) without asking for direction on intermediate
steps. Stop only on a declared per-change blocker.

## The work

PERFIDX04 did the **read** seam. PERFIDX05 does the **writeback + guard** side — four
mechanical changes from the PERFARCH01 Stage-5 definition:

1. Apply writeback by sorted `SymbolId` (`apply_writeback_payload` / the
   `apply_kernel_writeback` path). Bit-identical **only because** sorted-`SymbolId` order
   == sorted-`BoundarySymbol` order (ADR-0022). Do not change apply order.
2. Replace decomposition prefix scans with registry id-range checks (PL slot/phase
   dispatch). PERFIDX04's `require_integral_pl_dispatch_symbol_ref_in_range` is the
   groundwork — extend it.
3. Replace consumer-boundary + transfer validation scans with required-`SymbolId` sets.
4. Keep applied-symbol vectors in **logical string order** (determinism guard).

Plus the PERFIDX04 residual the handoff flagged: logical-export + layer-symbol `format!`
still in `format_inner` (`schedule_export.rs`, layer-symbol builders).

**Do not assert the ≤10× verdict** — that is Stage 6 (`PERFIDX06`).

## Two hard stops (non-negotiable)

1. **Failure-path tests are the load-bearing gate — bit-identity alone is NOT enough.**
   A guard you silently weaken (accepts what it should reject, or stops checking a
   symbol) **passes the happy-path anchor**, because a passing run never trips the guard.
   You MUST prove the migrated guards still reject **missing / non-finite / out-of-range
   / unknown-symbol** with the same error type and the same logical symbol name
   (`hydrology/02_guard_errors.rs`). Add negative tests wherever coverage is missing.
   This is the difference between PERFIDX05 and PERFIDX03B/04.
2. **Irrigation is OUT.** No pre-resolution, wiring, or activation (deferred:
   `backlog/20260617-irrigation-management-gated-activation.md`).

## The correctness trap (prove it, don't hand-wave)

Prefix scan → registry range check is equivalent **only if** no non-prefix symbol's id
falls inside `[first_id, last_id]` for that prefix. Lexicographic ordering does not
guarantee this for every prefix (a separator/extension char can sort a non-member
between two members). For each prefix→range conversion, **prove** the range is exactly
the prefix set on the production registry, and record it in
`perfidx05-prefix-range-proofs.md`. A range with an interloper or a missing boundary
member is a silent bug.

## Steps

1. **Inventory** — the four changes × their call sites + the `format_inner` residual.
   Irrigation excluded and noted.
2. **Per-change increments** — migrate one change at a time; after each, run the
   bit-identity anchor **and** the relevant failure-path tests so a happy-path regression
   *or* a weakened guard localizes to one change.
3. **Bit-identity (HARD)** — full anchor vs pre-PERFIDX05 baseline. Any mismatch → STOP.
4. **Failure-path (HARD)** — missing / non-finite / out-of-range / unknown reject with
   unchanged error type + logical symbol name.
5. **Prefix-range proofs** — each range == exactly the prefix set.
6. **Determinism** — `docs/numerics/`: applied-symbol vectors in logical string order; no
   FP/phase/OFE reorder; pinned-seed reproducible.
7. **Realized speedup** — H2637 + OFE1-OFE5 before/after; report honestly.
8. **Gates** — fmt; clippy `-D warnings`; `test --workspace`; deny; line-count.

## Hard constraints

- No `SC-*` change; no `BoundarySymbol` public API removal (logical names in errors +
  logical-order vectors preserved); writeback payload shape held.
- No irrigation pre-resolution/wiring/activation.
- Bit-identical outputs + failure-path parity + determinism (`docs/numerics/`).
- Truthfulness: bit-identity, failure-path, determinism, timing are empirical — label
  `Ran:`; do not assert the speedup or the ≤10× verdict.

## Required reading

- `docs/work-packages/20260617-perfidx05-writeback-guards-by-id-001/package.md`
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1)
- `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/staged-implementation-plan.md` (Stage 5)
- `docs/work-packages/20260617-perfidx04-hot-symbol-id-tables-001/artifacts/{perfidx04-worker-handoff,perfidx04-id-table-design,review-claude-independent,perfidx04-bit-identity-evidence}.md`
- `docs/backlog/20260617-irrigation-management-gated-activation.md` (irrigation carve-out)
- `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`,
  `docs/standards/mechanical-refactor-authoring-guide.md`, `docs/numerics/README.md`
- The `apply_writeback_payload` + guard-error code in scope.
