# PERFIDX03 — Indexed-Surface Authority Flip (Stage 3)

Status: executed-hold 2026-06-17 (ADR-0022 + Amendment 1; Stage 3 of the
`PERFARCH01` staged plan; PERFIDX02 complete - clone-economics gate cleared;
authority flip attempted and held on realized performance regression).
**Post-review closure:** all PERFIDX03 working-tree code was discarded; the committed
record is docs-only and `crates/` returns to the PERFIDX02 state — see
`artifacts/perfidx03_disposition.md` (Post-review closure). Re-flip happens clean in
`PERFIDX03B`; the inadvertently-wired irrigation is extracted to
`backlog/20260617-irrigation-management-gated-activation.md` (management-gated, out of
scope for perf).

Package type: **Behavior-preserving authority flip — high-risk** (the first stage
where outputs depend on the indexed store; bit-identity is **load-bearing**, no
longer shadow-dormant).

## Objective

Make the **sparse indexed store authoritative** for the per-OFE runtime surface,
replacing the `BTreeMap<BoundarySymbol, BoundaryValue>` backing, while keeping the
`BoundarySymbol` API, the kernel writeback payload shape, and **bit-identical
outputs**. This is the stage where the **clone win materializes** (lane-state
clones become sparse `Vec<(SymbolId, value)>` clones — PERFIDX02 measured 54–70×
on the clone itself).

PERFIDX02 proved the sparse shadow equals the BTreeMap (mismatch 0) and the clone
is a real-scale win; PERFIDX03 turns that shadow into the authority.

## What changes vs what is held

- **Authoritative storage** for the per-phase-cloned surface → the sparse store
  (PERFIDX02's `IndexedWritebackSurface`), keyed by `SymbolId`, sized to the
  working set. Lane clones are sparse-Vec clones.
- **`BoundarySymbol` compatibility layer stays** — `get`/`insert`/`iter`/export by
  `BoundarySymbol` over the sparse store (insert maintains sorted-`SymbolId` order;
  lookup is binary search). The rest of the code keeps using `BoundarySymbol`.
- **Writeback payload shape unchanged**; `apply_kernel_writeback` applied-symbol
  order preserved via sorted-`SymbolId` order.
- **Registry becomes production-active** (built every run, not env-gated).
- **Not in this stage:** resolve-once hot families (Stage 4) and guard-to-id-range
  migration (Stage 5). Hot lookups still go through the compatibility layer
  (`BoundarySymbol`→`SymbolId` per access) until Stage 4 — see Performance note.

## Milestones

1. **Pre-flip gate — reachable-registry across diverse managements (precondition).**
   Because the registry is now production-load-bearing and a miss is **fail-closed
   (a crash)**, validate the tightened reachable enumeration (the `ncut`/`ncycle`
   bounds + `.unwrap_or(0)` fallback from PERFIDX02) produces **0 post-freeze
   unknowns across a *diverse* config cohort** — grazing, multiple cuts/cycles,
   varied soil-layer/crop-rotation (**irrigation is out of scope** — deferred to
   `backlog/20260617-irrigation-management-gated-activation.md`; do **not** wire or
   activate irrigation) — not just H2637 + the ladder. If a
   gap is found, **fix the enumeration before flipping** (or branch a registry-
   completeness fix). Do not flip on an unproven reachable set.
2. **Flip authority.** Swap the surface backing to the sparse store behind the
   `BoundarySymbol` compatibility layer; lane clones → sparse clones; writeback
   payload unchanged.
3. **Bit-identity (HARD, load-bearing).** `anchor_mismatches = 0` on **H2637 both
   `wepp_ui` variants + the 1–5-OFE ladder** vs a pre-flip baseline. This is no
   longer dormant — the indexed store *is* the surface, so any divergence (sorted
   order, value representation, an edge case the shadow-equality cohort missed)
   changes outputs. **Escalate (STOP + diagnose) on any divergence** — a live
   mismatch means a case the shadow missed; find it, do not force the flip.
4. **Determinism.** Sorted-`SymbolId` order is now live (writeback order, exports);
   pinned-seed bit-reproducibility; no FP/phase/OFE reorder.
5. **Measure realized speedup.** H2637 before/after wall-clock + RSS. Report the
   net and, where possible, the clone-vs-lookup split.

## Performance note (set expectations)

Stage 3 realizes the **clone** win (the dominant PERFHO01 cost) but **not** the
lookup/`format!` win — hot lookups still resolve `BoundarySymbol`→`SymbolId` per
access through the compatibility layer, which adds indirection vs the old direct
BTreeMap lookup. **Net wall-clock may be modest** (clone gain partly offset by
compatibility-layer lookup cost) until Stage 4 (resolve-once). Stage 3's success is
the **correct, bit-identical flip with the clone win realized** — not the full
≤10×, which awaits Stages 4–6 and the Stage-6 re-measure. Report the net honestly;
a small or even ~neutral net here is acceptable if bit-identity holds and the clone
cost is demonstrably gone.

## Acceptance criteria

- Pre-flip: **0 post-freeze unknowns across the diverse-management cohort** (not
  only H2637 + ladder).
- **`anchor_mismatches = 0`** (load-bearing) on H2637 both variants + the ladder;
  determinism preserved.
- `BoundarySymbol` API + kernel writeback payload shape unchanged; the indexed
  store is authoritative; lane clones are sparse.
- Realized speedup measured (H2637 before/after + RSS), with honest framing per the
  Performance note.
- Gates: `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`; `cargo deny check`; line-count governance.

## Escalation boundary

- **Any bit-identity divergence → STOP + diagnose** (do not weaken the anchor gate;
  the shadow proved equality on a cohort, so a live mismatch is an uncovered case).
- If the diverse-management pre-flip gate finds a reachable-set gap that can't be
  fixed within this package's write-set, branch a registry-completeness fix and do
  not flip until it closes.

## Deliverables

- `artifacts/perfidx03-preflip-registry-coverage.md` (diverse-management 0-unknowns).
- `artifacts/perfidx03-bit-identity-evidence.md` (load-bearing anchor on the cohort).
- `artifacts/perfidx03-realized-speedup.md` (H2637 before/after + RSS + framing).
- `artifacts/perfidx03-gate-results.md`, `artifacts/perfidx03-line-count-governance.md`.
- `artifacts/perfidx03_disposition.md` + worker-handoff (naming Stage 4,
  `PERFIDX04-hot-symbol-id-tables-001`).

## Dependencies

- ADR-0022 + Amendment 1; PERFIDX02 (`IndexedSurface`/`IndexedWritebackSurface`, the
  shadow + clone measurement, `review-claude-independent.md`); PERFIDX01 (registry,
  the audit); PERFOPT01 (the anchor method).
- `docs/numerics/README.md`; `AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/standards/rust-scientific-coding-standard.md` (line-count).
- The surface/clone + writeback code: `openwepp-hillslope-orchestrator/src/scheduler.rs`,
  `openwepp-kernel-contract/src/lib_mod/{core_types,writeback}.rs`,
  `openwepp-runner/.../scheduler_seed_and_runtime.rs`, `indexed_shadow_surface.rs`,
  `symbol_registry_audit.rs`.

## Autonomy

Execute end-to-end (pre-flip diverse-management gate → flip → load-bearing
bit-identity → determinism → realized-speedup measurement → gates) without asking
for direction on intermediate steps. The two hard stops are non-negotiable: **do
not flip without the diverse-management reachable-registry proof**, and **stop +
diagnose on any bit-identity divergence** rather than weakening the gate.
