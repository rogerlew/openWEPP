# PERFIDX05 - Writeback And Guard Migration (Stage 5)

Status: **HELD 2026-06-18** — executed, bit-identical, but performance-NEGATIVE
(H2637 −5.3–5.8%); **code discarded**, record kept (Stage 5 of the PERFARCH01 staged
plan; follow-on to PERFIDX04, which migrated the read seam). **Post-review closure:**
the write/guard-side by-id migration is structurally net-negative under the current
logical-authoritative + read-mirror design (dual-write cost > id-lookup saving — see
`artifacts/review-claude-independent.md` and `artifacts/perfidx05_disposition.md`). All
PERFIDX05 working-tree code was discarded; `crates/` returns to the PERFIDX04 state. Per
operator decision (A), the program **pivots to `PERFIDX06` re-measure** to get the actual
≤10× verdict before any further write-side work. The PERFARCH01 Stage-5 "this is a win"
premise is undercut by this evidence.

Package type: **Behavior-preserving performance optimization — mechanical-refactor
shape.** Bit-identity is load-bearing **and so are failure-path tests** (see the gate
note below — this is the one package where bit-identity alone is *not* a sufficient
gate).

## Objective

PERFIDX04 migrated the **read** seam to resolve-once `SymbolId`. PERFIDX05 migrates the
**writeback + guard** side — the remaining `BoundarySymbol`/`format!`/scan machinery on
the commit and validation paths. Per the PERFARCH01 Stage-5 definition, four mechanical
changes:

1. **Apply writeback by sorted `SymbolId`** — `apply_writeback_payload`
   (`core_types.rs:454`) / the `apply_kernel_writeback` path
   (`scheduler.rs:1676`,`:1715`) commits by sorted `BoundarySymbol`; migrate to sorted
   `SymbolId`. This is bit-identical **only because** ids are assigned in sorted-`as_str()`
   order (ADR-0022), so sorted-`SymbolId` order == sorted-`BoundarySymbol` order. That
   invariant is load-bearing here; do not change apply order.
2. **Replace decomposition prefix scans with registry range checks** — the PL /
   decomposition code (`hydrology/00_pl_slot_resolution.rs`,
   `hydrology/05_pl_phase_dispatch.rs`) scans by string prefix; replace with registry
   id-range checks. PERFIDX04 already introduced `require_integral_pl_dispatch_symbol_ref_in_range`
   as groundwork — extend that pattern.
3. **Replace consumer-boundary and transfer validation scans with required-id sets** —
   `consumer_boundary.rs` and the same-day transfer validation scan symbol sets; replace
   with precomputed required-`SymbolId` sets.
4. **Keep applied-symbol vectors in logical string order** — the determinism guard: any
   vector exposed in errors / trajectory / export must stay in logical string order even
   though application now keys on `SymbolId`.

Also fold in the **PERFIDX04 residual** the handoff named: logical-export and
layer-symbol construction still showing in `format_inner`
(`schedule_export.rs`, layer-symbol builders) — these are the writeback/export-side
`format!` sites left for Stage 5.

The **≤10× verdict is NOT decided here** — that is Stage 6 (`PERFIDX06`). Report the
realized speedup honestly.

## The gate that makes this package different

For PERFIDX03B/04, bit-identity on the anchor was a near-complete correctness proof.
**For PERFIDX05 it is not.** The guard migration changes *validation* code — a guard
that has been silently **weakened** (now accepts a value it should reject, or no longer
checks a symbol) is **invisible to the happy-path anchor**, because a passing run never
trips the guard. Therefore the load-bearing gate is **failure-path / negative tests**:
prove the migrated guards still reject **missing**, **non-finite**, **out-of-range**, and
**unknown-symbol** cases with the same error type and the same logical symbol name as
before (`hydrology/02_guard_errors.rs`: `MissingRequiredStateSymbol`/`...FluxSymbol`,
`NonFiniteStateSymbol`/`...Flux`, `StateSymbolOutOfRange`/`...Flux`, + the Erod variants).
Bit-identity + determinism remain required, but they do **not** cover this.

## Known correctness trap (surface, don't hand-wave)

**Prefix scan → registry range check is not automatically equivalent.** A prefix's
symbols are contiguous in sorted-id order, but a `[first_id, last_id]` range check equals
a prefix scan *only if no non-prefix symbol's id falls inside that range*. Lexicographic
ordering does not guarantee that for every prefix (a separator/extension can sort a
non-member between two members). Codex must **prove** each migrated range is exactly the
prefix set for the production registry — a range that admits an interloper, or misses a
boundary member, is a silent correctness bug that may or may not show on the anchor.

## Scope

In scope (confirm against the actual writeback/guard call sites):

- `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs` (`apply_writeback_payload`;
  id-range / required-id-set helpers)
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` (writeback apply path)
- `crates/openwepp-hillslope-orchestrator/src/consumer_boundary.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/00_pl_slot_resolution.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/05_pl_phase_dispatch.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs` (error parity)
- `crates/openwepp-hillslope-orchestrator/src/schedule_export.rs` + layer-symbol builders
  (PERFIDX04 `format_inner` residual)
- Focused + failure-path tests and package artifacts.

Out of scope:

- **Irrigation** — no pre-resolution, wiring, or activation (deferred:
  `docs/backlog/20260617-irrigation-management-gated-activation.md`).
- No `SC-*` contract changes.
- No `BoundarySymbol` public API removal — logical names must still appear in errors and
  in the logical-order applied-symbol vectors.
- No writeback payload shape change.
- Stage 6 (re-measure / ≤10× verdict) is a separate package.

## Required approach (per-change increments)

1. Start from the PERFIDX04-complete tree.
2. Migrate one of the four changes at a time; after each, run the bit-identity anchor
   **and** the relevant failure-path tests, so a happy-path regression *or* a weakened
   guard localizes to a single change.
3. For each prefix→range conversion, record the proof that the registry range is exactly
   the prefix set (the trap above).
4. Keep logical `BoundarySymbol` for errors and logical-order vectors throughout.

If a guard cannot be migrated without changing its rejection behavior, **stop and record
the blocker** for that guard; partial migration of the other changes is acceptable and
declared.

## Acceptance Criteria

- **Bit-identity (load-bearing):** full PERFIDX04 anchor holds — `H1.hbp`, loss JSON,
  `wat.parquet`, `plot.parquet` byte-identical on H2637 both `wepp_ui` variants +
  OFE1-OFE5 ladder vs a pre-PERFIDX05 baseline; `pass.parquet` rows equal (container
  churn expected).
- **Failure-path tests (load-bearing, Stage-5 specific):** the migrated guards reject
  **missing / non-finite / out-of-range / unknown-symbol** with unchanged error type and
  logical symbol name. New negative tests required where coverage is absent.
- **Determinism** (`docs/numerics/`): no FP-reduction reorder, no per-OFE sequencing
  change; applied-symbol vectors stay in logical string order; pinned-seed reproducible.
- **Realized speedup:** H2637 + OFE1-OFE5 before/after wall-clock, reported honestly. Do
  **not** assert the ≤10× verdict.
- **Rust gates:** `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D
  warnings`; `cargo test --workspace`; `cargo deny check`.
- `git diff --check` and line-count governance recorded.

## Deliverables

- `artifacts/perfidx05-writeback-guard-inventory.md` (the four changes × call sites;
  PERFIDX04 `format_inner` residual; irrigation excluded)
- `artifacts/perfidx05-prefix-range-proofs.md` (each prefix→range = exactly the prefix
  set on the production registry)
- `artifacts/perfidx05-bit-identity-evidence.md`
- `artifacts/perfidx05-failure-path-evidence.md` (missing/non-finite/out-of-range/unknown)
- `artifacts/perfidx05-determinism-evidence.md`
- `artifacts/perfidx05-realized-speedup.md`
- `artifacts/perfidx05-gate-results.md`
- `artifacts/perfidx05-line-count-governance.md`
- `artifacts/perfidx05-review-a.md`
- `artifacts/perfidx05-review-b.md`
- `artifacts/perfidx05-verification-a.md`
- `artifacts/perfidx05-verification-b.md`
- `artifacts/perfidx05-worker-handoff.md`
- `artifacts/perfidx05_disposition.md`

## Dependencies

- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1; the
  sorted-`SymbolId` == sorted-`BoundarySymbol` invariant that makes apply-by-id identical)
- `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/staged-implementation-plan.md` (Stage 5)
- `docs/work-packages/20260617-perfidx04-hot-symbol-id-tables-001/artifacts/{perfidx04-worker-handoff,perfidx04-id-table-design,review-claude-independent}.md`
- `docs/work-packages/20260617-perfidx04-hot-symbol-id-tables-001/artifacts/perfidx04-bit-identity-evidence.md` (anchor method)
- `docs/backlog/20260617-irrigation-management-gated-activation.md` (irrigation carve-out)
- `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/numerics/README.md`

## Subagent Requirement

None required. If the operator authorizes subagents, the writeback/guard inventory
(read-only call-site mapping across crates) is the parallelizable step. Run closure
gates locally and record command evidence.

## Autonomy

Execute end-to-end through inventory, per-change incremental migration, bit-identity +
failure-path + determinism evidence, prefix-range proofs, realized-speedup measurement,
gates, dual review, dual verification, line-count governance, and disposition. Stop only
on a declared per-change blocker and record the first actionable follow-on.
