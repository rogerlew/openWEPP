# PERFARCH02 Kickoff — Array-Authoritative Hot-Path State Redesign (Scoping + Floor Prototype)

Execution mode: architecture design / feasibility scoping — measures and decides, lands no
production migration.

Autonomy: execute end-to-end (redesign-shape → floor prototype → bit-identity validation →
floor measurement → contract blast radius → GO/NO-GO/CONDITIONAL decision). A candid NO-GO is
a **successful** outcome. Do **not** start the production migration in this package.

## Why you're here

PERFIDX06 measured the PERFIDX04 endpoint at **73.12×** legacy (666.82 s / 9.12 s,
same-machine). The PERFIDX01–05 arc kept the `BTreeMap<BoundarySymbol, BoundaryValue>` as
both the kernel I/O contract and the authoritative hot-path store, and hit a ceiling: reads
won (PERFIDX04 −24%), writes/guards lost to **dual-write** (PERFIDX05 −5.7%), and the
authority flip died on the **export seam** (PERFIDX03). The profile is now spread across the
symbol-keyed map machinery **as a class** — no single excisable lever. More id-table work is
a known losing class.

## The question — and the unknown that decides it

**Can array-authoritative hot-path state reach ≤10×, is 5× on the table, and what is
openWEPP's intrinsic floor?** ≤10× needs removing ~86% of 667 s; **5× needs ~93%**.
PERFARCH01 estimated ~89–90% is removable string-keyed machinery — but openWEPP also does
legitimate work legacy does not (conservation gates, typed bounds, trajectory ownership,
parquet/HBP). That intrinsic work is the **floor**, and it has never been measured. **Measure
it with a minimal prototype before proposing any migration.**

## The redesign shape (if GO)

Promote the dense indexed rep from *mirror* to **authority**: hot-path state is a
`Vec<BoundaryValue>` indexed by `SymbolId`; kernels read/write it directly; `BoundarySymbol`/
`BTreeMap` live only at the I/O boundaries (seed once at parse; materialize once per shard at
publish, outside the daily loop). This **changes the kernel writeback payload shape** — the
thing every PERFIDX held out of scope. It builds on the PERFIDX01–04 `SymbolRegistry`/
`SymbolId`/`IndexedSurface` foundation.

Solve the three problems the program already paid to find:
1. **Export seam (PERFIDX03):** kernel consumes the array directly; no per-day `BTreeMap`
   rebuild at the boundary.
2. **Dual-write (PERFIDX05):** one authoritative rep, not two; `BTreeMap` rebuilt once at
   publish.
3. **Typed guards without string cost:** index/range/finite/bounds checks on the slot;
   resolve the logical name **only on the failure path** (lazy; PERFOPT01 proved it).

## The floor prototype — gating deliverable

Build a **minimal array-authoritative prototype of one representative hot-path flow** (e.g.
one OFE's daily WB11 + writeback + active guards) on array state; measure the achievable
per-OFE-day cost; extrapolate the H2637 floor; **then** decide.

**Honest-measurement hard stop:** the prototype must do openWEPP's **real** per-step work —
actual conservation gates, typed bounds, fail-closed guards, value semantics — just on array
state. A stripped prototype measures a fictional floor and is worse than nothing (the
comparator surface-artifact lesson). Its outputs on the prototyped flow must be
**bit-identity-validated** against the current path. The prototype is a measurement instrument
(flag-gated or separate harness), not production code.

## Decision (the deliverable)

- **GO** — floor extrapolates to ≤10× (or 5×): staged migration plan (per-kernel-family,
  bit-identity-gated, shadow-then-flip; order; contract blast radius) + proposed ADR
  (supersede/amend ADR-0022).
- **NO-GO** — floor too high: state the architectural ceiling ≤Nx and why; stop the migration
  honestly. Valid, successful closure.
- **CONDITIONAL** — ≤10× reachable but not 5× (or vice-versa): state the realistic target +
  cost.

## Constraints

- No production hot-path migration (downstream, gated on GO). No `SC-*` change. No
  parquet/HBP output-surface change (ADR-0019/0020) beyond *when/where* it materializes.
- Irrigation stays deferred/inert.
- Determinism (`docs/numerics/`) preserved; bit-identity is the validation gate even though
  the internal contract changes.
- Truthfulness: floor numbers, prototype timings, ratio are empirical — label `Ran:`; the
  GO/NO-GO is a judgment from evidence. Do not deliver an optimistic floor as a measured one.

## Required reading

- `docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/package.md`
- `docs/work-packages/20260618-perfidx06-high-ofe-target-assessment-001/artifacts/{perfidx06_disposition,perfidx06-bottleneck-analysis,perfidx06-legacy-ratio}.md`
- `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/staged-implementation-plan.md`
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1)
- `docs/work-packages/20260617-perfidx05-writeback-guards-by-id-001/artifacts/perfidx05_disposition.md` (dual-write)
- `docs/work-packages/20260616-perfidx03-indexed-surface-authority-001/artifacts/perfidx03_disposition.md` (export seam)
- `docs/work-packages/20260616-perf-high-ofe-hillslope-characterization-001/artifacts/perfho01-verdict.md` (determinism constraint)
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`, `0003-parity-semantic-not-bit.md`
- `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`, `docs/numerics/README.md`
- The `HillslopeWritebackSurface` producers/consumers + `IndexedSurface`/`SymbolRegistry` in
  `crates/openwepp-kernel-contract` and the scheduler.
