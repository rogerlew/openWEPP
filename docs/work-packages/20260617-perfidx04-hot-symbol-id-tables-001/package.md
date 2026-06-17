# PERFIDX04 - Resolve-Once Hot Families (Stage 4)

Status: scaffolded 2026-06-17 (Stage 4 of the PERFARCH01 staged plan; follow-on to
PERFIDX03B, which closed the clone/export blocker)

Package type: **Behavior-preserving performance optimization — mechanical-refactor
shape**. Bit-identity is **load-bearing** (this changes how hot values are *fetched*,
so identical outputs are the proof of correctness), exactly as in PERFOPT01/PERFIDX03B.

## Objective

Realize the **second dominant lever** after PERFIDX03B's clone elimination: the
per-access cost of resolving a hot symbol. Today the hot read paths build a
`BoundarySymbol` with `format!` per access (heap `String` alloc) and look it up in a
`BTreeMap` (String `memcmp` tree-walk). PERFIDX04 **resolves the `SymbolId` once** for
each hot family and replaces those hot paths with **id-table lookup** over the indexed
representation — eliminating the dynamic `format!` + String alloc + map walk from the
named hot paths.

This is where the *lookup/format!* half of the ~95% symbol-machinery overhead is meant
to come off. PERFIDX02 proved the clone economics; PERFIDX03B realized the clone win;
PERFIDX04 realizes the lookup win. The **≤10× (≤5×) verdict is NOT decided here** — that
is Stage 6 (`PERFIDX06`). Report the realized speedup honestly and do not assert the
target.

## Hot families in scope

Per the PERFARCH01 Stage-4 definition, anchored to the **PERFHO02 profiler evidence**
for which paths are actually hot (resolve the families the profiler named, not a
guessed list):

- climate forcing
- frost (incl. fine-layer)
- WB18 / WB19 water-balance phases
- PL (plant/runtime activation) symbols
- MOFE hourly forcing

**Explicitly OUT of scope: irrigation.** The PERFARCH01 plan listed irrigation among
the hot families, but irrigation is **deferred, unwired, and inert** (see
`docs/backlog/20260617-irrigation-management-gated-activation.md`). No irrigation
symbols are present in any production surface, so "pre-resolving irrigation ids" is
dead work that would re-touch the deferred pipeline. **Do not pre-resolve, wire, or
activate irrigation.** Irrigation gets its id-table treatment if and when its own
management-gated activation package lands.

## Central design question (ground the approach here)

PERFIDX03B's indexed mirror lives on the **persistent lane state** and is refreshed
**after** writeback. But the hot reads happen **during kernel execution**, on the
surface that `take_execution_input()` **moves** into the scheduler — a
`HillslopeWritebackSurface` (`BTreeMap`), not the mirror. So the maintained mirror is
**not** directly the thing the hot paths read.

PERFIDX04 must therefore decide **how the hot read paths obtain an id-indexed view at
execution time** — e.g. resolve ids once at setup and index the execution surface, or
carry an indexed execution surface alongside the logical one, or another option. This
is the load-bearing architecture choice; it is **Codex's to make**. The constraint is
only: realize id-table reads on the hot paths **without** reintroducing a per-lane/day
full-`BTreeMap` export (the PERFIDX03 trap the handoff warns against), and without
changing the writeback payload shape.

## Scope

In scope (confirm against the PERFHO02 profiler-named hot paths — these are the likely
surface, not an exhaustive list):

- `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs` (id-table / read API)
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/` climate / projection helpers
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
- Focused tests and package artifacts.

Out of scope:

- **Irrigation** — no pre-resolution, wiring, or activation (see above).
- No `SC-*` contract changes.
- No `BoundarySymbol` public API removal — the logical name stays the compatibility
  surface and **must still appear in errors** (fail-closed unknown-symbol preserved).
- No writeback payload shape change.
- Stage 5 (writeback/guards by id) and Stage 6 (re-measure) are separate packages.

## Required approach (per-family increments)

1. Start from the PERFIDX03B-complete tree.
2. From PERFHO02 profiler evidence, enumerate the hot symbols per family and the call
   sites that `format!`-build + map-lookup them.
3. Migrate **one family at a time**, with a bit-identity check **per increment** (the
   staged-increment / shadow-then-flip discipline), so any divergence localizes to a
   single family rather than the whole batch.
4. Resolve each family's ids once (against the frozen run-scoped registry) and replace
   the hot `format!` + map lookup with id-table lookup; keep the logical
   `BoundarySymbol` for errors and for any cold path.
5. Capture profiler evidence that dynamic symbol formatting is gone from the named hot
   paths — this is the Stage-4-specific gate, not just wall-clock.

If a family cannot be migrated bit-identically without a payload/API change, **stop and
record the blocker** for that family; partial migration of the other families is an
acceptable, declared outcome.

## Acceptance Criteria

- **Bit-identity (load-bearing):** value-by-id equals value-by-`BoundarySymbol`. The
  full PERFIDX03B anchor holds:
  - `H1.hbp`, loss JSON, `wat.parquet`, `plot.parquet` byte-identical vs a pre-PERFIDX04
    baseline on H2637 both `wepp_ui` variants + the OFE1-OFE5 ladder;
  - `H1.pass.parquet` logical rows compare equal (container bytes are known to churn —
    see PERFIDX03B review; this is not a failure).
- **Determinism** (`docs/numerics/`): no FP-reduction reorder, no per-OFE sequencing
  change, pinned-seed reproducible.
- **Profiler evidence** (Stage-4 specific): `perf` / profiler output showing dynamic
  symbol `format!` + map-lookup removed from the named hot paths.
- **Realized speedup:** H2637 + OFE1-OFE5 before/after wall-clock, reported honestly
  (net + the families that moved the needle). Do **not** assert the ≤10× verdict.
- **Logical names preserved in errors;** fail-closed unknown-symbol behavior intact.
- **Rust gates:** `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D
  warnings`; `cargo test --workspace`; `cargo deny check`.
- `git diff --check` and line-count governance recorded.

## Deliverables

- `artifacts/perfidx04-hot-family-inventory.md` (families × hot symbols × call sites,
  from PERFHO02 evidence; irrigation explicitly excluded)
- `artifacts/perfidx04-id-table-design.md` (the execution-time id-indexed read approach)
- `artifacts/perfidx04-bit-identity-evidence.md`
- `artifacts/perfidx04-determinism-evidence.md`
- `artifacts/perfidx04-profiler-evidence.md` (format! removed from named hot paths)
- `artifacts/perfidx04-realized-speedup.md`
- `artifacts/perfidx04-gate-results.md`
- `artifacts/perfidx04-line-count-governance.md`
- `artifacts/perfidx04-review-a.md`
- `artifacts/perfidx04-review-b.md`
- `artifacts/perfidx04-verification-a.md`
- `artifacts/perfidx04-verification-b.md`
- `artifacts/perfidx04-worker-handoff.md`
- `artifacts/perfidx04_disposition.md`

## Dependencies

- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1)
- `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/staged-implementation-plan.md` (Stage 4)
- `docs/work-packages/20260617-perfidx03b-indexed-kernel-seam-or-export-cache-001/artifacts/perfidx03b-worker-handoff.md` (do not reintroduce full-map export)
- `docs/work-packages/20260617-perfidx03b-indexed-kernel-seam-or-export-cache-001/artifacts/review-claude-independent.md` (mirror-location coupling; pass.parquet container non-determinism)
- `docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/artifacts/perfho02-profiler-evidence.md` (the authority for which paths are hot) + its `perfho02_disposition.md`
- `docs/backlog/20260617-irrigation-management-gated-activation.md` (irrigation carve-out)
- `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/numerics/README.md`

## Subagent Requirement

None required. If the operator authorizes subagents, the hot-family inventory (read-only
mapping of call sites across crates) is the parallelizable step; otherwise enumerate
locally. Run closure gates locally and record command evidence.

## Autonomy

Execute end-to-end through inventory, per-family incremental migration, bit-identity +
determinism + profiler evidence, realized-speedup measurement, gates, dual review, dual
verification, line-count governance, and disposition. Stop only on a declared per-family
blocker and record the first actionable follow-on.
