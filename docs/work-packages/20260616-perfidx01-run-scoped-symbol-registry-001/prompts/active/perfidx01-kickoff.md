# PERFIDX01 Kickoff — Run-Scoped Symbol Registry (Indexed Surface Stage 1)

Execution mode: package-end-to-end (behavior-preserving infrastructure addition).

Autonomy: execute end-to-end (add `SymbolId`/`SymbolRegistry` → build it from
projection → prove sorted-id + completeness + equality → gates) without asking for
direction on intermediate steps.

## Stage 1 only — no storage-authority flip

Per ratified **ADR-0022**, add the frozen run-scoped symbol registry and **prove
its invariants on real runs**. The `BTreeMap<BoundarySymbol, BoundaryValue>`
runtime surface stays authoritative this stage — the registry is a shadow you
validate. **Do not** make the indexed store authoritative (that is Stage 2,
`PERFIDX02`). Outputs must stay bit-identical.

## Steps

1. **Add types.** `SymbolId` (newtype over `u32`) + `SymbolRegistry` (sorted
   `BoundarySymbol` vec → `id` = index; reverse `BoundarySymbol`→`SymbolId`
   lookup), near `BoundarySymbol` in `openwepp-kernel-contract`.
2. **Build the registry** deterministically at projection from parsed dimensions —
   static constants + every dynamic family (climate `point_count`; `wb18/wb19`
   per `nsl`; frost layer/fine; PL schedule/growth/decomp slots×crops; MOFE hourly;
   irrigation events). Assign ids as the index in the **sorted** symbol list. No
   lazy interning after freeze.
3. **Prove sorted-id** — test: `registry.id_of(s)` == `s`'s sorted position for the
   full universe; lexicographic ≡ numeric order for the zero-padded `{:04}` indexed
   families.
4. **Prove completeness (the hard gate)** — a test/validation mode (flagged, so
   production output is untouched) asserting **every** `BoundarySymbol`
   inserted/looked-up during H2637 (both `wepp_ui` variants) + the 1–5-OFE ladder
   is in the frozen registry. **Zero post-freeze unknowns.** Record observed symbol
   count vs registry size. If a family can't be pre-enumerated → **stop and report
   a design-blocking finding** for ADR-0022 revision; do **not** weaken fail-closed.
5. **Prove equality** — id-ordered registry export round-trips to the BTreeMap
   surface (same keys/order/values).
6. **Gates** — if runtime code is touched: `anchor_mismatches = 0` on H2637 + ladder
   vs a pre-change baseline (the PERFOPT01/PERFARCH01 anchor method) + determinism.
   Then `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
   `cargo test --workspace`; `cargo deny check`; line-count governance.

## Hard constraints

- No storage-authority flip; no `BoundarySymbol` API change; no `SC-*` change.
- Fail-closed on unknown post-freeze symbols (typed error) — never silent lazy
  intern.
- Bit-identical outputs if any runtime code is touched; determinism per
  `docs/numerics/` (no FP/phase/OFE reorder; stable id assignment run-to-run).
- Truthfulness: completeness + bit-identity are empirical — label `Ran:`.

## Required reading

- `docs/work-packages/20260616-perfidx01-run-scoped-symbol-registry-001/package.md`
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (the authority)
- PERFARCH01 `artifacts/{indexed-runtime-surface-design,staged-implementation-plan,risk-register}.md`
  + the prototype microbench.
- `AGENTS.md`, `docs/codex_exec_plans.md`, `docs/numerics/README.md`,
  `docs/standards/rust-scientific-coding-standard.md`.
- The symbol-generator code in `package.md` Dependencies.
