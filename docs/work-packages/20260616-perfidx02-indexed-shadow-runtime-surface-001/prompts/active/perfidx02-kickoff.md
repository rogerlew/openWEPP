# PERFIDX02 Kickoff — Indexed Shadow + Clone-Economics Gate (Stage 2)

Execution mode: package-end-to-end (behavior-preserving infrastructure + a binding
clone-economics go/no-go).

Autonomy: execute end-to-end (clone-economics prototype + measurement → tighten
registry → shadow + equality → bit-identity + gates) without asking for direction
on intermediate steps.

## The gate comes first (ADR-0022 Amendment 1)

PERFIDX01 found the registry capacity is ~1.7M for H2637 (worst-case
over-enumeration; ~3.6K materialize; the per-phase-cloned surface is
~hundreds–low-thousands present). Amendment 1 therefore **rejected a dense `Vec`
over the global `SymbolId`** and requires the store be sized to the working set,
with the clone economics **measured at real H2637 scale, not assumed**. **Measure
before you build the shadow.**

## Steps

1. **Clone-economics measurement (HARD go/no-go).** Capture a *real* H2637
   per-OFE-day surface snapshot (the actual present `(BoundarySymbol, BoundaryValue)`
   set, several OFEs/days). Microbench at that real present count + the production
   (reachable) registry size:
   - clone time: **sparse sorted `Vec<(SymbolId, BoundaryValue)>`** (primary) and a
     **compact local-index dense array** vs the current `BTreeMap::clone`;
   - lookup time on the hot families; RSS.
   Choose the representation by the numbers. **Gate:** the chosen store must keep
   the clone a **win** at H2637 scale. **If neither does, STOP** — report a
   migration go/no-go finding for ADR-0022; do not build the shadow, do not weaken
   the gate to force a pass.
2. **Tighten the production registry to the reachable set** (bounded by parsed
   dimensions, not the worst-case combinatorial bound). Re-run the PERFIDX01
   completeness audit against it: still **0 post-freeze unknowns** on H2637 (both
   variants) + the 1–5-OFE ladder.
3. **Add the indexed shadow** (the chosen representation) beside the authoritative
   `BTreeMap`. Populate it from the BTreeMap; **validate round-trip equality**
   (id-ordered shadow export ≡ BTreeMap key order + values) on the cohort. Keep the
   shadow **dormant in production** (reuse/extend the PERFIDX01 env-gated hook).
4. **Do not flip storage authority** (Stage 3). The BTreeMap stays authoritative;
   outputs unchanged.
5. **Gates** — `anchor_mismatches = 0` + determinism (shadow dormant); then
   `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
   `cargo test --workspace`; `cargo deny check`; line-count governance.

## Hard constraints

- No storage-authority flip; no `BoundarySymbol` API change; no `SC-*` change.
- The clone-economics gate is a hard stop **both ways** — proceed only if the
  working-set clone is a win at H2637 scale; STOP honestly if it isn't.
- Sparse-store `SymbolId` order must preserve sorted-string order
  (`apply_kernel_writeback` / exports); prove it on the cohort.
- Bit-identical outputs (shadow dormant in production); determinism per
  `docs/numerics/`.
- Truthfulness: clone/lookup/RSS are empirical — label `Ran:`; the representation
  choice must be justified by the measured numbers, not asserted.

## Required reading

- `docs/work-packages/20260616-perfidx02-indexed-shadow-runtime-surface-001/package.md`
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (esp. **Amendment 1**)
- PERFIDX01 `artifacts/{perfidx01-storage-representation-analysis,review-claude-independent,perfidx01-registry-and-invariants}.md`
- PERFARCH01 `artifacts/prototypes/indexed_surface_microbench.rs` (measurement template)
  + `staged-implementation-plan.md`; PERFOPT01 disposition (the anchor method).
- `AGENTS.md`, `docs/codex_exec_plans.md`, `docs/numerics/README.md`,
  `docs/standards/rust-scientific-coding-standard.md`.
- The surface/clone code in `package.md` Dependencies.
