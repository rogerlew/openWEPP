# PERFIDX02 — Indexed Shadow Runtime-Surface + Clone-Economics Gate (Stage 2)

Status: complete 2026-06-16 (ADR-0022 + Amendment 1; Stage 2 of the
`PERFARCH01` staged plan; PERFIDX01 complete; storage authority not flipped)

Package type: **Behavior-preserving infrastructure addition + a binding
clone-economics go/no-go** (adds a non-authoritative indexed shadow; the BTreeMap
surface stays authoritative; no output change — but the migration's real go/no-go
is decided here by measurement).

## Objective

Per [ADR-0022 Amendment 1](../../decisions/0022-indexed-runtime-surface-representation.md):
before any later authority flip (Stage 3), **prove at real H2637 scale that a
working-set-sized indexed store keeps the per-OFE-day clone a win** vs the current
BTreeMap, **choose the representation by measurement** (sparse vs compact-dense),
then **add that store as a validated shadow** beside the authoritative BTreeMap.

This is the make-or-break stage. PERFIDX01 proved the registry is complete and
sorted-id-correct but found its capacity is ~1.7M for H2637 (worst-case
over-enumeration; ~3.6K materialize; the cloned surface is ~hundreds–low-thousands).
Amendment 1 therefore **rejected a dense `Vec` over the global `SymbolId`** and
requires the store be sized to the working set, with the clone economics
**measured, not assumed**.

## Decision context (Amendment 1)

- Store backing the per-phase-cloned surface is **sized to its working set**, keyed
  by `SymbolId`. **Primary candidate: sparse sorted `Vec<(SymbolId, BoundaryValue)>`**
  (sized to *present* ~hundreds, memcpy clone, no `String`, naturally sorted-id
  order). **Alternative: compact local-index dense array** (if a hot path needs
  O(1)). A dense `Vec` over the global 1.7M `SymbolId` space is **rejected**.
- The **global sorted `SymbolRegistry` is unchanged** (id assignment, completeness,
  `BoundarySymbol` export; sparse order preserves `apply_kernel_writeback` order).
- The **production registry registers the reachable universe**, not the worst-case
  bound (PERFIDX01's 1.7M was a validation posture).

## Milestones

1. **Clone-economics prototype + measurement (the go/no-go).** Capture a *real*
   H2637 per-OFE-day surface snapshot (actual present symbols + values, multiple
   OFEs/days). Microbench, at that real present count and against the production
   (reachable) registry size: **clone** time of the sparse `Vec<(SymbolId,val)>`
   (and the compact-dense candidate) vs the current `BTreeMap::clone`; lookup time
   on the hot families; **RSS**. **Gate:** the chosen working-set store must keep
   the clone a win at H2637 scale. **If none does, STOP** and report a
   migration-blocking finding for ADR-0022 (the indexed-surface approach's go/no-go)
   — do not proceed to the shadow.
2. **Tighten the production registry to the reachable set** (bounded by parsed
   dimensions), so the shadow + RSS are sane. Re-run the PERFIDX01 completeness
   audit against the tightened registry: still **0 post-freeze unknowns** on H2637
   (both variants) + the 1–5-OFE ladder.
3. **Add the indexed shadow surface** (the measurement-chosen representation) beside
   the authoritative `BTreeMap`. Populate the shadow from the BTreeMap; **validate
   round-trip equality** — id-ordered shadow export ≡ the BTreeMap key order/values
   — on the H2637 + ladder cohort. Reuse/extend the PERFIDX01 env-gated audit hook
   (or a sibling) so the shadow is **dormant in production**.
4. **Keep the BTreeMap authoritative** (no flip — Stage 3). Outputs unchanged.

## Acceptance criteria

- **Go path:** measurement evidence (`Ran:`) that the chosen working-set store keeps
  the clone a win at real H2637 scale (report the factor + RSS); the representation
  choice (sparse vs compact-dense) is justified by the numbers. Shadow round-trip
  equality passes on H2637 (both variants) + the ladder. Registry tightened; 0
  post-freeze unknowns. `anchor_mismatches = 0` + determinism (shadow dormant in
  production). `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D
  warnings`; `cargo test --workspace`; `cargo deny check`; line-count governance.
- **No-go path:** if no working-set representation keeps the clone a win at H2637
  scale, that is a clean, honest **STOP** with the measurements and the
  representation tried — a migration go/no-go finding for ADR-0022, not a forced
  pass. (The analysis predicts go, but the gate is real.)
- **Storage authority is NOT flipped** (Stage 3's job).

## Deliverables

- `artifacts/perfidx02-clone-economics-measurement.md` (the go/no-go: real-scale
  clone/lookup/RSS, sparse vs compact-dense, the representation choice).
- `artifacts/perfidx02-shadow-equality-evidence.md` (round-trip equality on the
  cohort) + `artifacts/perfidx02-bit-identity-evidence.md`.
- `artifacts/perfidx02-gate-results.md`, `artifacts/perfidx02-line-count-governance.md`.
- `artifacts/perfidx02_disposition.md` + worker-handoff (naming Stage 3,
  `PERFIDX03-indexed-surface-authority-001`, only if the go/no-go passed).

## Dependencies

- ADR-0022 + **Amendment 1** (the storage decision); PERFIDX01 (registry, the audit,
  `review-claude-independent.md`, `perfidx01-storage-representation-analysis.md`).
- PERFARCH01 staged plan + the prototype microbench (the measurement template);
  PERFOPT01 (the bit-identity anchor method).
- `docs/numerics/README.md`; `AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/standards/rust-scientific-coding-standard.md` (line-count).
- The surface/clone code: `openwepp-hillslope-orchestrator/src/scheduler.rs`
  (`HillslopeWritebackSurface`, the clone sites), `openwepp-kernel-contract/src/lib_mod/core_types.rs`
  (`SymbolRegistry`, `SymbolId`), `openwepp-runner/.../scheduler_seed_and_runtime.rs`
  (per-OFE-day clone + the daily climate clear/reload), `symbol_registry_audit.rs`.

## Autonomy

Execute end-to-end (clone-economics prototype + measurement → tighten registry →
shadow + equality → bit-identity + gates) without asking for direction on
intermediate steps. **Do not flip storage authority.** The clone-economics gate
(milestone 1) is a hard stop in **both** directions: do not proceed to the shadow
if the working-set clone is not a win at H2637 scale, and do not weaken the gate to
force a pass — report the go/no-go honestly with the measurements.
