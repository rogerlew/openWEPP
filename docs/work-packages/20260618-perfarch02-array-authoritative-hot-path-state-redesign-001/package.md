# PERFARCH02 - Array-Authoritative Hot-Path State Redesign (Scoping + Floor Prototype)

Status: executed 2026-06-18 (CONDITIONAL GO to an integrated WB11
array-authoritative pilot; successor to PERFARCH01; follows the PERFIDX01-06
indexed-surface arc that hit its ceiling at 73.12x)

Package type: **Architecture design / feasibility scoping — measures and decides; lands no
production migration.** Like PERFARCH01, the output is a prototype-backed decision + (if GO)
a staged plan and a proposed ADR — **not** the migration itself. The migration, if approved,
is downstream packages gated on this one's GO verdict.

## Why this package exists

PERFIDX06 measured the PERFIDX04 endpoint at **73.12×** legacy (666.82 s / 9.12 s,
same-machine, same-fixture). The whole PERFIDX01–05 arc kept the
`BTreeMap<BoundarySymbol, BoundaryValue>` (`HillslopeWritebackSurface`) as **both** the
kernel I/O contract **and** the authoritative hot-path store, and optimized *around* it
(registry, read-mirror, id-tables). That design has a ceiling:

- **PERFIDX04** won on reads (−24%) — a read can hit the indexed mirror.
- **PERFIDX05** lost on writes/guards (−5.7%) — every writeback must **dual-write** the
  logical map *and* the mirror; the dual-write cost exceeds the id saving.
- **PERFIDX03** showed making the indexed store authoritative regressed via the
  per-lane/day **full-`BTreeMap` export seam** at the kernel boundary.

The PERFIDX06 profile is **spread across the symbol-keyed map machinery as a class**
(`BTreeMap` insert/remove, `__memcmp_sse2`, malloc/free, symbol-table access, writeback,
decomposition guards, residual formatting) with **no single excisable lever**. The cost is
the *representation*, not a hot function. More narrow id-table work is a known losing class.

## The question this package answers

**Can an array-authoritative hot-path state representation reach ≤10×, and is 5× physically
on the table — and what is openWEPP's intrinsic floor?**

The arithmetic: ≤10× requires removing ~86% of the 667 s; **5× requires removing ~93%**.
PERFARCH01 estimated ~89–90% of elapsed is string-keyed surface machinery (removable), but
openWEPP also does **legitimate work legacy does not** — conservation gates every step,
typed bounds, producer/consumer trajectory ownership, parquet + HBP output. That *intrinsic*
work sets the floor, and **it has never been measured**. Whether it is ~2–4× legacy (5×
reachable) or ~6–8× legacy (≤10× is the real target) is the single unknown that decides the
answer. **This package measures it with a minimal prototype BEFORE proposing a full
migration.**

## The redesign shape (what would be built, if GO)

Promote the dense indexed representation from *mirror* to **authority** on the hot path:

- Hot-path state = `Vec<BoundaryValue>` indexed by `SymbolId` (or struct-of-arrays) — the
  legacy fixed-array model that does this physics in ~9 s.
- Kernels read/write that array **directly** by `SymbolId` (resolved once).
- `BoundarySymbol` / `BTreeMap` survive **only at the I/O boundaries**: seed the array once
  at input parse; materialize the contract-visible surface once per shard at publish
  (HBP / parquet) — *outside* the per-OFE-day loop.

This **changes the kernel writeback payload shape** — the exact thing every PERFIDX package
held out of scope. That is the unlock *and* the risk. It **builds on**, not discards, the
PERFIDX01–04 foundation (frozen `SymbolRegistry`, sorted `SymbolId`, `IndexedSurface`).

### The three problems it must solve (the program already paid to find these)

1. **Export seam (PERFIDX03):** the kernel must consume the array directly; no per-day
   `BTreeMap` reconstruction at the boundary. Materialize only at publish.
2. **Dual-write (PERFIDX05):** one authoritative representation, not two. No logical mirror
   maintained in the loop. The `BTreeMap` is rebuilt once at publish, not dual-written.
3. **Typed guards without string cost:** guards become index/range/finite/bounds checks on
   the array slot; the logical `BoundarySymbol` name is resolved **only on the failure
   path** (lazy — PERFOPT01 proved this preserves fail-closed errors + diagnostics at
   ~zero success-path cost).

## The floor prototype (the first and gating deliverable)

Build a **minimal array-authoritative prototype of one representative hot-path flow** — e.g.
one OFE's daily WB11 hydrology + writeback + the active guards — on array state, and measure
the achievable per-OFE-day cost. Extrapolate the H2637 floor. **Then** decide feasibility.

**Honest-measurement constraint (the apples-to-apples trap, again):** the prototype must do
openWEPP's **real** per-step work — the actual conservation gates, typed bounds, fail-closed
guards, value semantics — just on array state. A prototype that strips guards/conservation
to look fast measures a fictional floor and is worse than no measurement (cf. the comparator
surface-artifact lesson). The prototype's outputs on the prototyped flow must be
**bit-identity-validated** against the current path, so the measured cost is for a *correct*
array path. The prototype is a measurement instrument (flag-gated alternate path or separate
harness), **not** production code.

## Decision (the deliverable)

A GO / NO-GO / CONDITIONAL verdict, evidence-backed:

- **GO** — the floor extrapolates to ≤10× (or 5×); here is the staged migration plan
  (per-kernel-family increments, each bit-identity-gated, shadow-then-flip; the order; the
  contract-change blast radius across scheduler / kernels / writeback / guards / publication)
  and a proposed ADR (superseding/amending ADR-0022 for the kernel-contract change).
- **NO-GO** — the floor is too high (e.g. ≤Nx is the architectural ceiling); state N and why,
  and whether ≤10× / 5× is simply not reachable for openWEPP's contract surface. This is a
  **valid, successful** outcome — it stops a large, risky migration that could not pay.
- **CONDITIONAL** — ≤10× reachable but 5× not (or vice-versa); state the realistic target
  and the cost to reach it.

## Scope

In scope:

- The floor prototype (measurement instrument; flag-gated or separate harness; no production
  migration).
- Bit-identity validation of the prototype flow vs the current path.
- Floor measurement + extrapolation; the three-problem design; the contract-change blast
  radius; the decision + (if GO) staged plan + proposed ADR.

Out of scope:

- **No production hot-path migration** — that is downstream packages gated on a GO verdict.
- **Irrigation** — deferred/inert (`docs/backlog/20260617-irrigation-management-gated-activation.md`).
- No `SC-*` science-contract change (this is a *representation/interface* redesign, not a
  physics change; outputs must stay bit-identical).
- No change to the parquet/HBP output surface (ADR-0019/0020) beyond where/when it is
  materialized.

## Acceptance Criteria

- **Floor prototype runs and is bit-identical** on its prototyped flow vs the current path
  (a stripped/incorrect prototype is a fail — measure real work).
- **Floor measured + extrapolated:** achievable per-OFE-day cost → H2637 floor → implied
  legacy ratio, with method + variance, on the same machine as PERFIDX06.
- **Three-problem design:** explicit answers for export-seam, dual-write, and
  typed-guards-without-string-cost, each backed by the prototype where possible.
- **Contract-change blast radius:** enumerated (scheduler, kernel I/O, writeback, guards,
  publication) with the bit-identity strategy for the migration.
- **Decision:** GO / NO-GO / CONDITIONAL with the realistic target and evidence; if GO, the
  staged migration plan + proposed ADR (superseding/amending ADR-0022).
- Markdown lint clean. (Rust gates only on whatever prototype code is produced; no production
  migration to gate.)

## Deliverables

- `artifacts/perfarch02-redesign-shape.md` (array-authoritative design; the three problems)
- `artifacts/perfarch02-floor-prototype.md` (what was built; bit-identity validation; method)
- `artifacts/perfarch02-floor-measurement.md` (per-OFE-day cost → H2637 floor → ratio)
- `artifacts/perfarch02-contract-blast-radius.md` (what the kernel-contract change touches)
- `artifacts/perfarch02-staged-migration-plan.md` (if GO; per-family increments + gates)
- `artifacts/perfarch02-proposed-adr.md` (if GO; supersede/amend ADR-0022)
- `artifacts/perfarch02_disposition.md` (GO / NO-GO / CONDITIONAL + realistic target)

## Dependencies

- `docs/work-packages/20260618-perfidx06-high-ofe-target-assessment-001/artifacts/{perfidx06_disposition,perfidx06-bottleneck-analysis,perfidx06-legacy-ratio,review-claude-independent}.md`
- `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/staged-implementation-plan.md` (the prior design + its ~89–90% estimate)
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1 — the design this supersedes/amends)
- `docs/work-packages/20260617-perfidx05-writeback-guards-by-id-001/artifacts/{perfidx05_disposition,review-claude-independent}.md` (dual-write ceiling)
- `docs/work-packages/20260616-perfidx03-indexed-surface-authority-001/artifacts/perfidx03_disposition.md` (export seam)
- `docs/work-packages/20260616-perf-high-ofe-hillslope-characterization-001/artifacts/perfho01-verdict.md` (the 80–110× characterization + determinism constraint)
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`, `docs/decisions/0003-parity-semantic-not-bit.md`
- FARPOINT01 legacy baseline + H2637 fixture; `docs/numerics/README.md`
- `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`

## Subagent Requirement

None required. If the operator authorizes subagents, the contract-blast-radius enumeration
(read-only call-site mapping of `HillslopeWritebackSurface` producers/consumers across crates)
is the parallelizable step. The prototype + floor measurement are local.

## Autonomy

Execute end-to-end through redesign-shape design, floor prototype, bit-identity validation,
floor measurement, blast-radius enumeration, and the GO/NO-GO/CONDITIONAL decision. A candid
NO-GO ("≤Nx is the floor; 5× / ≤10× is not reachable for this contract surface") is a valid,
successful closure — it must stop a large migration honestly rather than start one
optimistically. If GO, produce the staged plan + proposed ADR; do **not** begin the
production migration in this package.
