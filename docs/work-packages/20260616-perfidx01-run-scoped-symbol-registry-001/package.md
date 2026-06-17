# PERFIDX01 — Run-Scoped Symbol Registry (Indexed Runtime-Surface Stage 1)

Status: complete 2026-06-16 (ADR-0022 ratified; Stage 1 of the
`PERFARCH01` staged plan; operator-directed; storage authority not flipped)

Package type: **Behavior-preserving infrastructure addition** (Stage 1 — adds the
registry + invariant proofs **without flipping storage authority**; the BTreeMap
runtime surface stays authoritative; no output change).

## Objective

Add the `SymbolId` + frozen run-scoped `SymbolRegistry` mandated by
[ADR-0022](../../decisions/0022-indexed-runtime-surface-representation.md), and
**prove its two load-bearing invariants on real runs** before any storage change:

1. **Sorted-id invariant** — `SymbolId` order ≡ sorted `BoundarySymbol::as_str()`
   order (the property that makes id-storage preserve `apply_kernel_writeback`
   ordering and deterministic exports cheaply).
2. **Completeness / no-lazy-interning** — the registry, built deterministically at
   projection from parsed dimensions, captures the **entire** symbol universe a
   real run produces; any symbol absent from the frozen registry **fails closed**
   (ADR-0022). This is the make-or-break feasibility proof for the whole migration:
   if the universe cannot be fully pre-enumerated, Stages 2–6 cannot proceed as
   designed.

This stage de-risks Stage 2 (the indexed shadow) by establishing the registry is
correct and complete first.

## Scope (this stage)

- Add `SymbolId` (newtype; `u32` — ~4–6K symbols/run fit, with headroom) and
  `SymbolRegistry` (sorted `BoundarySymbol` vector → `id` = index; reverse lookup
  `BoundarySymbol`→`SymbolId`), near `BoundarySymbol` in
  `openwepp-kernel-contract/src/lib_mod/core_types.rs` (Codex finalizes placement).
- Build the registry deterministically from the parsed projection dimensions —
  static constants + the dynamic families the audit enumerated: climate
  `timem/intsty_{i}` (`point_count`), `wb18_perc_*` / `wb19_*` (`nsl`), frost
  layer/fine (`frost_layer_symbol`/`frost_fine_layer_symbol`), PL
  schedule/growth/decomp slots/crops, MOFE hourly, irrigation events.
- Add BTreeMap↔registry **export/equality adapters** (id-ordered export ≡ the
  current sorted BTreeMap key order).
- **Do not** make the indexed store authoritative; **do not** change the runtime
  execution path's outputs. The registry is a shadow this stage only validates.

## Behavior-preservation contract

- **No output change.** If runtime code is touched at all (e.g. to build the
  registry at projection), outputs stay **bit-identical**: `anchor_mismatches = 0`
  on H2637 (both `wepp_ui` variants) + the 1–5-OFE ladder vs a pre-change baseline.
- **Fail-closed on unknown post-freeze symbols** (ADR-0022) — a typed error, never
  a silent lazy intern.
- Determinism (`docs/numerics/`): id assignment is deterministic and stable
  run-to-run; no FP/phase/OFE reorder.
- `BoundarySymbol` public API unchanged; no `SC-*` change.

## Key validation (the dispositive deliverables)

1. **Sorted-id test** — for the full real symbol universe (incl. the zero-padded
   `{:04}` dynamic families), assert `registry.id_of(s)` equals `s`'s position in
   the sorted symbol list, and that lexicographic order ≡ numeric order for the
   padded indexed symbols.
2. **Completeness audit on real runs** — instrument a **test/validation mode**
   (behind a flag so production output is untouched) that checks **every**
   `BoundarySymbol` inserted or looked up during H2637 + the ladder is present in
   the frozen registry. **Zero post-freeze unknowns** is required. Record the
   observed symbol count vs the registry size. If unknowns appear, fix the
   enumeration (or report the family that cannot be pre-enumerated as a
   design-blocking finding for ADR-0022 revision).
3. **Equality adapter test** — id-ordered registry export round-trips to the
   BTreeMap surface (same keys, same order, same values).

## Acceptance criteria

- `SymbolId` + `SymbolRegistry` added; the three validations above pass (`Ran:`).
- **Completeness: 0 post-freeze unknown symbols** across H2637 (both variants) +
  the 1–5-OFE ladder.
- If runtime code is touched: `anchor_mismatches = 0` + determinism.
- Gates: `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`; `cargo deny check`. Line-count governance dispositioned.
- The indexed store is **not** authoritative (Stage 2's job).

## Deliverables

- `artifacts/perfidx01-registry-and-invariants.md` (sorted-id + completeness +
  equality evidence, with the observed symbol counts).
- `artifacts/perfidx01-bit-identity-evidence.md` (if runtime touched).
- `artifacts/perfidx01-gate-results.md`, `artifacts/perfidx01-line-count-governance.md`.
- `artifacts/perfidx01_disposition.md` + worker-handoff (naming Stage 2,
  `PERFIDX02-indexed-shadow-runtime-surface-001`).

## Dependencies

- ADR-0022 (the design authority); PERFARCH01 design + staged plan + risk register
  + the audit (`20260616-perfarch01-indexed-runtime-surface-design-001/`).
- `docs/numerics/README.md` (determinism); `AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/standards/mechanical-refactor-authoring-guide.md`,
  `docs/standards/rust-scientific-coding-standard.md` (line-count).
- The runtime-surface + symbol-generator code:
  `openwepp-kernel-contract/src/lib_mod/core_types.rs`,
  `openwepp-hillslope-orchestrator/src/{constants.rs,scheduler.rs}`,
  `.../hydrology/support_helpers_mod/state_access.rs`,
  `.../runtime_inputs/05_projection_helpers.rs`,
  `.../hydrology/07_decomposition_equations.rs`.

## Autonomy

Execute end-to-end (add types → build registry from projection → prove sorted-id
+ completeness + equality → gates) without asking for direction on intermediate
steps. **Do not flip storage authority** (Stage 2). The completeness proof is the
hard gate: if the symbol universe cannot be fully pre-enumerated deterministically,
**stop and report it as a design-blocking finding** for ADR-0022 revision rather
than weakening the fail-closed rule.
