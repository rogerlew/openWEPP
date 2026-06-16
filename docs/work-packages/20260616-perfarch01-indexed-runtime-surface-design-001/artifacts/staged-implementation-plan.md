# PERFARCH01 Staged Implementation Plan

Status: COMPLETE 2026-06-16
Evidence mode: **Static** design plan informed by PERFHO02 and prototype evidence

Each stage is a separate work package. No stage closes without bit-identity,
determinism, and normal Rust gates unless explicitly scoped as docs-only.

## Stage 0: ADR Ratification

Package: `PERFADR01-indexed-runtime-surface-ratification`

- Review and ratify proposed ADR-0022.
- Confirm the no-lazy-interning-after-freeze rule.
- Confirm sorted-id order as the compatibility invariant.

Gate: documentation review only.

## Stage 1: Registry Skeleton

Package: `PERFIDX01-run-scoped-symbol-registry-001`

- Add `SymbolId` and `SymbolRegistry`.
- Build a frozen registry from existing runtime surfaces.
- Add tests proving `id` order equals sorted `BoundarySymbol` order.
- Add BTreeMap export/equality checks.
- Do not make the indexed store authoritative yet.

Gates:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- H2637/OFE ladder bit-identity anchors where runtime code is touched.

## Stage 2: Indexed Shadow Surface

Package: `PERFIDX02-indexed-shadow-runtime-surface-001`

- Add `IndexedSurface` and `IndexedWritebackSurface`.
- Populate indexed state/flux shadows from current maps.
- Validate round-trip equality against the BTreeMap surfaces.
- Keep current maps as authority.

Gate: same as Stage 1 plus explicit equality tests for state/flux surfaces.

## Stage 3: Indexed Surface Authority With Compatibility View

Package: `PERFIDX03-indexed-surface-authority-001`

- Make indexed state/flux storage authoritative inside scheduler lifecycle.
- Keep `BoundarySymbol` compatibility accessors and sorted export.
- Replace lane-state clones with dense surface clones.
- Keep current kernel writeback payload shape.

Gate: full Rust gates, bit identity, determinism, and H2637 before/after timing.

## Stage 4: Resolve-Once Hot Families

Package: `PERFIDX04-hot-symbol-id-tables-001`

- Pre-resolve climate, frost, WB18/WB19, PL, MOFE hourly, and irrigation ids.
- Replace hot `format!` + map lookup paths with id-table lookup.
- Preserve logical symbol names in errors.

Gate: full Rust gates, bit identity, determinism, and profiler evidence showing
dynamic symbol formatting removed from the named hot paths.

## Stage 5: Writeback And Guard Migration

Package: `PERFIDX05-writeback-guards-by-id-001`

- Apply writeback by sorted `SymbolId`.
- Replace decomposition prefix scans with registry range checks.
- Replace consumer-boundary and transfer validation scans with required-id sets.
- Keep applied-symbol vectors in logical string order.

Gate: full Rust gates, bit identity, determinism, and failure-path tests for
missing, non-finite, out-of-range, and unknown-symbol cases.

## Stage 6: Target Assessment

Package: `PERFIDX06-high-ofe-target-assessment-001`

- Re-run the PERFHO02 H2637 profiler flow with `perf`.
- Re-run OFE1-OFE5 and H2637 wall-clock ladder.
- Compute actual legacy ratio and remaining top symbols.
- Decide whether <=10x is closed, whether <=5x remains plausible, and what the
  next non-storage bottleneck is.

Gate: profiler artifacts, timings, bit identity, determinism, and disposition.
