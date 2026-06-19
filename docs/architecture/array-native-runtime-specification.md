# Array-Native Hot-Path Runtime Architecture — Specification

Status: **Ratified** — binding design authority for the perf re-architecture, ratified by [ADR-0025](../decisions/0025-array-native-hillslope-day-frame.md) on 2026-06-18
Audience: all contributors; binding design authority for the perf re-architecture program
Owner: Claude Code (architecture authoring) — implementation by Codex
Supersedes: the *incremental application* of [ADR-0023](../decisions/0023-array-authoritative-hot-path-state.md)
(dense authority by symbol/phase) — **not** its dense-authority principle, which this specification fulfils completely
Last updated: 2026-06-18

---

## 0. Summary

openWEPP's per-OFE-day hot path threads all inter-kernel state through symbol-keyed maps
(`BTreeMap<BoundarySymbol, BoundaryValue>`). Profiling proves this representation — not the physics — is
~99% of the runtime: the engine is **73.12× slower than legacy** on H2637, and the physics floor is
sub-microsecond. Two incremental migration rungs (PERFMIG01/02) failed because partial migration pays
**dual-representation bookkeeping** that dominates the win.

This specification defines a **comprehensive array-native re-architecture**: replace the symbol-keyed maps
with a single **typed, dense, cache-resident daily working set** (the *HillslopeDayFrame*) that all 14
phases mutate in place. Symbol/logical surfaces survive **only at the I/O edge** — the HBP scalars and the
WB13 publication-operand projection at end-of-run (parquet rows are typed structs, but are currently
*assembled* from runtime-surface symbol reads that must be lifted to typed frame/projection access; see §5).
Because the frame *is* the state, there are **no phase-to-phase materialization seams to retire** — the
boundary cost that sank the incremental rungs does not exist by construction. This is the fulfilment of openWEPP's own declared kernel boundary — *"kernels
are pure functions over typed state"* ([architecture/README.md](README.md)) — which the string-keyed maps
silently violated.

Target: **≤10× (ideally ≤5×) vs legacy on H2637** — the viability gate. The arithmetic below provides a
strong model hypothesis; staged H2637 endpoint measurements remain the closure authority.

---

## 1. Motivation & Evidence

### 1.1 The viability gate
≤10× (ideally ≤5×) slower than legacy WEPP on H2637 is an **imperative viability gate**, not an
optimization target. Current state and budgets (PERFIDX06, same machine/fixture):

| Anchor | H2637 no-UI | µs/OFE-day | ×legacy | RSS |
|---|---:|---:|---:|---:|
| Legacy WEPP | 9.12 s | 38.65 | 1.0× | ~4.6 MB |
| openWEPP (PERFMIG01 baseline) | 669.97 s | 2826 | 73.46× | ~228 MB |
| **≤10× budget** | 91.2 s | 386 | 10× | — |
| **≤5× budget** | 45.6 s | 193 | 5× | — |

H2637 = 235,961 OFE-days. RSS gap is ~50× — the maps pointer-chase and cache-thrash.

### 1.2 The physics is not the floor — the representation is
PERFARCH03 ran one real WB11 warm-rain runoff branch two ways on the same inputs:

| Path | µs/OFE-day | Note |
|---|---:|---|
| Production logical kernel (same branch) | 140.83 | symbol-keyed reads + writeback payload |
| Fully array-native (read+compute+write dense) | **0.96** | `to_bits()`-identical on 543 state + 8 flux outputs |
| — array physics only | 0.075 | the actual arithmetic |
| One-shot logical materialize (the seam) | 108.07 | dense→logical at a phase edge |

**The branch is 146.8× faster array-native, byte-identical.** Physics is ~1 µs; the other ~139 µs is
symbol machinery. RSS array-native: ~3 MB (cache-resident).

### 1.3 The cost is spread as a class, not localized (PERFIDX06 call-tree)
Children %: Wb11 hydrology **41.6%**, runoff/frost coupling **21.1% + 20.6%**, writeback **17.0%**,
decomposition dispatch **11.7%**, residual **`format!` 9.5%**, overflow guards **8.2% / 7.7%** — all
`BTreeMap` insert/remove, `__memcmp_sse2`, alloc/free, symbol-table access. The PERFIDX06 conclusion:
*"remove symbol-keyed map work, allocation churn, formatting, and dual publication from the hot path **as a
class**, not by shaving one site."* This is why no incremental seam-shaving converges, and why hydrology
alone (~42%) only gets ~73× → ~43×: **≤10× requires the whole OFE-day hot path array-native.**

### 1.4 Why incrementalism failed (the two dead rungs)
- **PERFMIG01** (writeback-only, CONTINUE-but-negative): flipped one phase's *output* to dense, but the
  scheduler immediately materialized dense→logical for downstream (the 108 µs seam). Net **+0.47%** —
  a round-trip added without removing a logical read.
- **PERFMIG02** (reader/materialization retirement, REDIRECT): of 543+8 symbols only **6** were
  internal-only and retireable (the rest feed publication/reporting); retiring those six cost **more**
  (stale-logical removal 105.46 µs > materialize-all 104.75 µs). Final endpoint **flat/negative**
  (672–675 s). This is the PERFIDX05 dual-write ceiling again: maintaining two representations during a
  partial migration is dominated by bookkeeping.

**Lesson, decisive:** the win requires migrating a **complete unit** so the internal seams *vanish*, not
retiring seams one at a time. The complete unit, taken to its conclusion, is the whole per-OFE-day hot
path. That is this specification.

---

## 2. The Core Thesis

1. **The frame eliminates seams by construction.** When all 14 phases mutate one in-memory typed frame,
   there is no dense↔logical conversion *between* phases — the 108 µs/seam tax (and the stale-removal tax)
   simply does not exist. Boundaries collapse to the run's true I/O edge.
2. **Typed state is the declared kernel boundary, not a new direction.** [architecture/README.md](README.md)
   already specifies *"kernels are pure functions over typed state; orchestrators own time-stepping and
   topology; producer/consumer trajectory-ownership maps onto Rust lifetimes."* The `BTreeMap<BoundarySymbol,
   BoundaryValue>` surface is a scaffolding compromise that became the hot-path authority. The frame makes
   the code match its own architecture.
3. **Logical/symbol surfaces are an I/O serialization concern, not a runtime-state concern.** The I/O map
    (§5) shows the minimal HBP edge is ~5 scalars, but current WB13 publication assembly still reads a
    broader runtime-surface symbol set before parquet emission. Stage 0 must map each publication operand to
    typed frame/projection fields before logical hot-path deletion.

---

## 3. The Current Hot Path (grounded baseline)

The re-architecture replaces a concrete, mapped structure. (Refs from the orchestrator/contract crates.)

**Loop nesting:** Years → Days → **OFE lanes 1..N (sequential)** → **14 phases (topological DAG)** per
OFE-day. Day-to-day carryover in `OfeLanePersistentState`; phase-to-phase state in
`HillslopeWritebackSurface`. Inter-OFE routing between lanes via 24-hour carry arrays.

**The 14 phases** (`phase.rs` `HillslopePhase::ORDERED`, fixed topo order):
`Normalization → StorageBounds → DecompositionTransition → ResiduePartitionTransition →
AnnualGrowthTransition → PerennialGrowthTransition → PercolationDeepSeepage → Evapotranspiration →
Drainage → LateralTransfer → PlantRootUptake → RunoffReconciliation → StorageReconciliation →
ClosureDiagnostics` (+ erosion EROD13/14/19 via PeakRunoff/ClosureDiagnostics). ~10.8k lines in
`hydrology/kernel_phases_mod/`; `runoff_reconciliation` (1956 lines) is the largest.

**Kernel interface:** `HillslopeKernel::run_hillslope_phase(&HillslopeKernelRequest) -> KernelRunResponse`.
Request borrows `state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>` + `flux_surface` (+ optional
indexed mirrors + hot tables). Response returns `KernelWritebackPayload` (symbol-keyed `Vec<WritebackField>`)
+ optional `IndexedKernelWritebackPayload`. The scheduler evaluates guards, applies the payload, and
(PERFMIG01) optionally materializes a dense payload back to logical.

**State types:**
- `BoundarySymbol` = newtype `String`, ~200–400 symbols.
- `BoundaryValue` = enum of ~16 **scalar** unit types (`Scalar(f64)`, `WaterDepthMeters`,
  `TemperatureCelsius`, `FlowRateCubicMetersPerSecond`, …). **No array/series members** — arrays are
  flattened to indexed symbols (`mofe_hs_carry_0001..0024`, climate `timem_0001..1500`, per-layer
  `wb14_*_0001..`).
- `HillslopeWritebackSurface { state_surface: BTreeMap<…>, flux_surface: BTreeMap<…> }`.
- `SymbolRegistry` (frozen at run start) + `IndexedSurface` (sorted `Vec<(SymbolId, BoundaryValue)>`) +
  `HotSymbolTables` — the ADR-0022/PERFIDX read-acceleration layer.
- Guards: finite + `[min,max]` domain checks on every read (`require_state_scalar`) and every writeback
  field (`evaluate_kernel_writeback`).

**MOFE routing:** already typed — `TransferInput`/`TransferOutput { surface_carry: [f64;24],
lateral_carry: [f64;24], … }`. The 24-hour arrays only become symbols when inserted into the maps.

---

## 4. Target Architecture — the HillslopeDayFrame

### 4.1 The frame: one typed dense working set
Replace the two `BTreeMap`s with a single owned struct — the authoritative per-OFE mutable state for a
simulation day. Conceptual shape (final field set is an implementation decision; this fixes the
*structure*):

```rust
/// Authoritative dense per-OFE state for one simulation day.
/// Owns every value the 14-phase pipeline reads or writes. No symbol maps.
pub struct HillslopeDayFrame {
    // ---- scalar state, named typed fields (unit newtypes are zero-cost) ----
    pub wb12_infiltration: WaterDepthMeters,
    pub wb12_runoff_reconciled: WaterDepthMeters,
    pub wb14_effective_conductivity: LinearRateMetersPerSecond,
    // … every scalar that is currently a BoundarySymbol becomes a field …

    // ---- fixed-width arrays (was: index-suffixed symbol families) ----
    pub mofe_surface_carry: [f64; 24],
    pub mofe_lateral_carry: [f64; 24],
    pub soil_layer: SoilLayerColumns,          // struct-of-arrays over N layers
    pub frost_fine_layer: FrostFineColumns,

    // ---- variable-length forcing (borrowed, read-only for the day) ----
    pub hyetograph: &'day Hyetograph,          // climate series, not copied per phase

    // ---- flux outputs the pipeline produces ----
    pub flux: HillslopeDayFlux,
}
```

Design rules:
- **Scalars are named typed fields.** Keep the `BoundaryValue` unit newtypes (`WaterDepthMeters`, …) as
  field types — they are zero-cost (compile to `f64`) and preserve dimensional safety, satisfying *"pure
  functions over typed state."* Raw `f64` is the fallback only where a unit type does not exist.
- **Array families become fixed arrays / struct-of-arrays**, not 24/N separate symbols. SoA layout for the
  per-layer and per-hour data keeps the hot inner loops cache-linear (the RSS lever — target a few-MB,
  L2/L3-resident working set, like legacy's COMMON blocks).
- **Forcing series are borrowed read-only** for the day, never copied into the frame per phase.
- **Representation baseline (ratified by PERFDEEP01, 2026-06-18):** the dense store is
  `state_slots`/`flux_slots: Vec<Option<BoundaryValue>>` indexed by frozen `SymbolRegistry` id — the
  **representation PERFARCH03 actually measured** (0.96 µs/OFE-day = 146×, well inside the ≤5× budget) —
  plus the fixed-width array families and typed I/O-edge fields above. This is the authoritative Stage-0+
  frame. Promoting the bulk scalars further to **named unit-typed `f64` fields** (removing `Option`, the
  enum match, and id indirection) is a **second-order micro-optimization** — the conceptual struct above is
  the aspirational end-state, not a Stage-1 requirement — tracked as an open fork (§12 fork 1). The slot
  baseline already clears the viability gate; typed-field promotion is pursued only if a later endpoint
  shows the per-access enum/`Option` cost matters. (The original "no `Option`, no id indirection" rule was
  an over-specification corrected here.)

### 4.2 Kernels as pure functions over the frame
The kernel signature loses the symbol surfaces and the writeback payload:

```rust
// before: fn run_hillslope_phase(&HillslopeKernelRequest) -> KernelRunResponse  (symbol maps + payload)
// after:  fn run_<phase>(frame: &mut HillslopeDayFrame, ctx: &PhaseInputs) -> Result<(), GuardError>
```

- A phase **reads and writes the frame in place**; no `WritebackField` construction, no symbol resolution,
  no `apply_indexed_kernel_writeback`. Producer/consumer ownership is enforced by the borrow checker
  (`&mut` for the fields a phase owns; `&` for what it consumes), fulfilling the trajectory-ownership rule.
- The 14-phase pipeline is a fixed in-order sequence of such calls over one `&mut frame`. The
  topological-DAG scheduler is preserved as the *ordering authority* but dispatches direct typed calls, not
  trait-object kernel invocations with map requests.

### 4.3 Guards become typed frame checks with static and dynamic bounds
The finite/domain guards (currently per-symbol-read and per-writeback-field) become **typed field checks at
the point of write** with a two-tier schema: (a) compile-time field invariants for static bounds and
finiteness, and (b) runtime-derived bounds for branch/state-dependent checks that today flow through
per-update minimum/maximum metadata. Fail-closed semantics, `SimulationStatus` message-id classes, and the
conservation/closure invariants (`SC-*`) are **preserved exactly**. Migration must also preserve boundary
diagnostic attribution policy (field/symbol subject semantics) as an explicit parity gate.

### 4.4 MOFE / OFE-lane routing
Unchanged in shape — `TransferInput`/`TransferOutput` are already typed (`[f64;24]`). The lanes carry
`HillslopeDayFrame` (or its persistent projection) instead of `HillslopeWritebackSurface`. The inter-OFE
transfer reads/writes the frame's `mofe_*_carry` arrays directly — no symbol insertion.

### 4.5 Day-to-day persistence
`OfeLanePersistentState` carries the **typed persistent projection** of the frame (the subset of fields
that survive to the next day) instead of a `HillslopeWritebackSurface`. Start-of-day seeds the frame from
the persistent projection; end-of-day flushes the surviving fields back. Both are typed struct moves, not
map rebuilds.

---

## 5. The I/O Materialization Edge (the key enabling finding)

A fully array-native run must still produce the existing outputs **byte-for-byte**. The I/O map shows the
logical surface is mostly unnecessary once publication operands are lifted to typed frame/projection access:

| Output | Source today | Array-native disposition |
|---|---|---|
| `wat.parquet` | typed `Vec<HillslopeWatRow>`, but rows are currently assembled from runtime-surface symbol/flux reads in runner helpers | lift WB13/WAT row assembly to typed frame/projection operands; no symbol lookups on the hot path |
| `pass.parquet` | typed `Vec<HillslopePassRow>`, derived from WB13/publication operands sourced from runtime-surface reads today | same typed publication projection as WAT; preserve Arrow/semantic identity |
| `loss.json` | static input config | no runtime read |
| `*.hbp` shard | typed binary, **but** constructed by reading ~5 logical runtime scalars at end-of-run (`peakro`, `watdur`, `total_detachment_kg`, `total_deposition_kg`, `sediment_concentration_kg_m3_0001`) | **capture these as typed scalars during the run**; then HBP construction reads typed fields, no logical |
| WB13 daily publication assembly | currently computes publication terms from many runtime-surface symbols/fluxes (`prcp`, `wb11_soil_water`, `frost.*`, `snow.*`, `Irr`, `Q`, `q`, `Qd`, etc.) | define a typed publication projection with operand-lineage parity fixtures before deleting the logical hot path |
| `run_manifest.json` provenance | a few runtime-surface execution fields | capture via typed execution trace |

**Consequences:**
- Parquet rows accumulate in memory per day and flush once at end-of-run; no mid-run logical flush is
  required once WB13/publication operand reads are lifted to typed frame/projection sources.
- Remaining logical dependencies are HBP scalar capture, WB13/publication operand extraction, and manifest
  provenance until those paths are migrated; retire all three before deleting logical hot-path plumbing.
- The HBP binary format and the watershed CLI's typed parser are **unchanged** — the inter-binary contract
  is already typed; only the *construction* path changes from symbol-read to typed-field-read.

So the end state: **no `BoundarySymbol` / `BTreeMap` / writeback-payload on the per-OFE-day hot path or
in daily publication operand assembly.** The registry/`HotSymbolTables`/`IndexedSurface` machinery
(ADR-0022) is retained only where a symbol surface is still genuinely needed (e.g. explicit
legacy-compat diagnostics, replay, and bounded serialization adapters), not on the simulation hot path.

---

## 6. Performance Model

| Quantity | Value | Basis |
|---|---:|---|
| Current OFE-day | 2826 µs | PERFIDX06 |
| Array-native branch (measured) | 0.96 µs | PERFARCH03 |
| Per-phase seam cost (eliminated) | 108 µs × 0 | no seams inside the frame |
| Projected array-native OFE-day | **~14–20 µs** | 14 phases × ~1 µs, no seams |
| Projected ×legacy | **~0.4–0.5×** | 14–20 µs / 38.65 µs |

This is the *floor* (the whole OFE-day array-native, I/O-only logical). Even with conservative discounts
for heavier phases and residual overhead, the model suggests significant headroom vs ≤10× and a plausible
≤5× trajectory; stage-by-stage endpoint measurements remain the only closure evidence. The two compounding
levers:
1. **Instruction count:** symbol resolution / map ops / payload construction / `format!` deleted as a class.
2. **Cache:** working set 228 MB → ~3 MB (PERFARCH03), L2/L3-resident — fewer misses multiply the
   instruction-count win (why PERFARCH03 got 146×, not the ~20× instruction math alone).

**Honesty bound:** the floor is a projection until measured end-to-end. The staged plan (§8) measures the
real H2637 endpoint after every stage; the model is the hypothesis, the endpoint is the authority.

---

## 7. Identity-Gating Strategy (non-negotiable)

A comprehensive rewrite of a scientific engine cannot be a big-bang — it must stay **byte-identical** at
every step. The discipline:

1. **Shadow / parallel-run differential.** During migration, run the legacy logical path **and** the frame
   path for the same OFE-day and assert `to_bits()` equality on every shared output (seeded by PERFARCH03's
   543+8 fixture, extended per phase). The frame path becomes authoritative only when its stage's diff is
   clean. (This is the FDHP01 shadow-state template from the MOFE port — proven.)
2. **Per-phase identity fixtures.** Every migrated phase ships a focused fixture proving its frame
   implementation matches the logical kernel bit-for-bit, including the snow/frost/irrigation/MOFE branches
   (not just warm-rain).
3. **H2637 output identity per stage:** `.hbp` + `wat.parquet` byte-identical, `pass.parquet` Arrow-equal —
   the PERFMIG01/02 gate, kept.
4. **H2637 endpoint + RSS per stage**, same machine/fixture as PERFIDX06. The endpoint is the perf
   authority; the model (§6) is only the hypothesis.
5. **Determinism + conservation gates** (`SC-*` closure invariants) green throughout.

**Kill-criteria (when to stop and re-think, stated up front):**
- Any stage that **cannot** be made bit-identical → stop; the divergence is a real defect or a
  contract-gap, adjudicate before proceeding.
- The **first full hydrology island** (the §8 Stage 2 endpoint) does **not** beat the current endpoint
  meaningfully → the map-seam hypothesis is wrong for production scale and the remaining gap is elsewhere
  (re-profile before continuing). This is the honest falsification point; it is only valid on a *large*
  island (a mini-island is boundary-noise, per PERFMIG01/02).

---

## 8. Staged Execution Plan (comprehensive, identity-gated)

The full rewrite, sequenced so each stage is independently identity-gated and endpoint-measured. Stages are
*committed scope of one program*, not "let's see if it's worth it" — PERFARCH03 already proved the floor.

| Stage | Scope | Gate | Expected endpoint |
|---|---|---|---|
| **0 — Frame scaffold** | Define `HillslopeDayFrame` + typed field schema + start/end-of-day seed/flush + typed I/O capture (HBP `peakro`/`watdur`/sediment, manifest provenance, WB13/publication operands). No phase migrated yet; frame runs *beside* the maps (shadow). | Frame round-trips to/from the logical surface bit-identically; publication projection fixtures and output parity stay green. | ~flat |
| **1 — Hydrology island core** | Migrate the contiguous hydrology cluster (RunoffReconciliation, StorageReconciliation, Evapotranspiration, Percolation, Lateral, Drainage, PlantRootUptake, PeakRunoff) to run over the frame in place. Delete the inter-phase materialization for these. Logical only at island edges (shadow-diffed). | Per-phase bit-identity (all branches) + H2637 identity. | first **measured win** — island ≈ 42% of cost; ~73× → ~43–50× |
| **2 — Close the hydrology edges** | Remove the island's edge materialization; the frame is authoritative across the whole hydrology span; capture I/O-edge scalars typed. | H2637 identity + endpoint; **the §7 falsification check**. | ~43× → solidify |
| **3 — Erosion island** | EROD13/14/19 + PeakRunoff over the frame (sediment columns SoA). | identity + endpoint. | removes erosion machinery share |
| **4 — Growth/decomposition + transitions** | Decomposition/Residue/Growth phases + Normalization/StorageBounds over the frame. | identity + endpoint. | removes remaining phase share |
| **5 — Delete the logical hot path** | Remove `HillslopeWritebackSurface` from the per-OFE-day loop, the writeback payloads, the indexed mirror, the per-day registry build. Logical/symbol surfaces survive only in I/O serialization + replay/diagnostics. | full H2637 identity + endpoint + RSS; **≤10× / ≤5× check.** | **the viability-gate measurement** |

Each stage is one work-package (`PERFDEEP0N`), identity-gated, endpoint-timed, committed only when green.
A stage that regresses without a retireable cause is reverted (the PERFMIG02 precedent), but unlike the
incremental rungs, the frame stages have **no seam tax** — a regression would indicate a real problem, not
an expected boundary offset.

---

## 9. Non-Goals & Preserved Invariants

**This is a representation change. It does not change:**
- **Physics / numerics.** Every kernel computes the same arithmetic; results are **byte-identical**
  (`to_bits()`), not "close." No clean-room, no algorithm change.
- **Science contracts (`SC-*`).** All invariants, closure/conservation laws, guard semantics, and
  message-id classes preserved — they move representation, not meaning.
- **Output schemas.** HBP binary format (ADR-0012 authority), `wat`/`pass`/`loss` parquet/JSON schemas
  (ADR-0019/0020) unchanged. The watershed inter-binary contract is untouched.
- **Process model.** Subprocess-per-hillslope (ADR-0004), the daily loop, the 14-phase DAG ordering, MOFE
  routing topology — all preserved.
- **Determinism.** The numerics determinism policy (`docs/numerics/`) holds; field-order evaluation is
  fixed and reproducible.
- **Irrigation** stays management-gated (no activation change).

**Out of scope:** watershed-CLI internals; parser/intake; legacy ASCII compat (wepppy's concern); the
replay binary (may keep a symbol surface for diffing).

---

## 10. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| **Blast radius** (10.8k kernel lines + scheduler + contract) | Staged islands (§8), each independently identity-gated; shadow-run keeps the logical path live until a stage is proven. |
| **Identity drift** on a subtle branch (snow/frost/irrigation) | Per-branch bit-fixtures required before a phase is authoritative; shadow-diff on H2637 catches any divergence. |
| **The model is wrong at scale** (phases heavier than runoff) | The §7 falsification check at Stage 1/2: a large island that doesn't move the endpoint stops the program for re-profiling. |
| **Conditional-phase frame fields** (phases that don't always run) | The frame carries all fields; unrun phases leave their fields at the seeded/identity value, exactly as the maps do today (absence → default). Validated by identity. |
| **Variable-length forcing** (climate series up to 1500 pts) | Borrowed read-only slice on the frame, not copied; indexed by integer, not symbol. |
| **Frame field churn** during migration | The field schema is the contract; add fields additively per stage; the start/end seed-flush is the single touch-point. |
| **Dynamic guard-bound drift** (current checks use runtime-derived min/max) | Two-tier guard schema (static invariants + runtime-derived bounds), plus accept/reject parity fixtures for message-id class and diagnostic attribution policy. |
| **Output-publication dependency undercount** (WB13 helpers still symbol-read many operands) | Stage-0 publication operand ledger + typed projection adapter + byte/Arrow identity fixtures before logical hot-path deletion. |

---

## 11. Relationship to ADRs & Ratification

- **ADR-0022** (indexed runtime surface / `SymbolId`): retained for I/O serialization + replay; **removed
  from the simulation hot path** (the frame replaces the read mirror). Not contradicted — its scope
  narrows.
- **ADR-0023** (array-authoritative hot-path state): this specification is its **completion**. ADR-0023's
  dense-authority principle stands; its *incremental, symbol-by-symbol application* (PERFMIG01/02) is
  superseded by the whole-frame approach. The new ADR should record this supersession explicitly.
- **Authority (ratified 2026-06-18):** ADR-0025 is the accepted hot-path runtime authority and this
  specification is its binding design authority. ADR-0023's incremental application is superseded — no
  further writeback-only or materialization-retirement rungs.
- **ADR-0019/0020** (output schemas), **ADR-0012** (HBP authority), **ADR-0004** (subprocess model):
  unaffected.
- **Kernel boundary** ([architecture/README.md](README.md)): this specification **fulfils** the declared
  "pure functions over typed state" boundary.

**Ratification path:** this specification requires a new ADR (next free number, **ADR-0025**) — *"Adopt the
array-native HillslopeDayFrame as the hot-path runtime architecture"* — citing this document as the design
authority, recording the supersession of ADR-0023's incremental application, and gating execution on the
§7 identity discipline. Execution proceeds as the `PERFDEEP0N` work-package series (§8) only after
ratification.

---

## 12. Open Design Decisions (for the ratifying ADR / implementation)

These are genuine forks left to the implementing ADR + Codex, not dictated here:

1. **Frame layout:** array-of-structs vs struct-of-arrays for the per-layer/per-hour data. SoA favours the
   cache-linear inner loops (recommended); AoS may be simpler for sparse access. Decide per measured hot loop.
2. **Unit-typed fields vs raw `f64`:** recommended typed (zero-cost, preserves the kernel-boundary
   contract); raw `f64` only where no unit newtype exists. The ADR fixes the policy.
3. **Frame ownership model:** one `&mut HillslopeDayFrame` threaded through the pipeline vs a
   producer/consumer borrow-split per phase. The borrow-split better enforces trajectory ownership but is
   more invasive; decide against the real phase data-dependencies.
4. **Guard-schema representation:** const field-bound table vs per-field validating newtype constructors.
5. **Shadow-run mechanism:** a compile-time feature flag running both paths, vs a test-harness-only
   differential. Affects how long the logical path stays compiled into the hot binary.
6. **Diagnostic attribution policy:** preserve symbol-oriented violation subjects at boundaries, or ratify a
  field-oriented subject contract with explicit compatibility notes and fixtures.

---

## Appendix A — Grounded reference map

| Concern | Current location |
|---|---|
| Daily/OFE/phase loop | `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` (`execute_*_ofe_sequence*`, ~1858–1993); phase order `src/phase.rs:23-38` |
| Phase DAG / dispatch | `scheduler.rs:145-201` (deps), `:1210-1369` (topo exec), `hydrology/01_phase_routing.rs` (phase→class) |
| Kernel trait / request / response | `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs` (`HillslopeKernel` ~940, request ~723, response ~364, payloads ~314/339) |
| State surfaces / symbols / registry | `…/core_types/00_symbol_registry_and_indexed_surfaces.rs`; `HillslopeWritebackSurface` `scheduler.rs:252-256` |
| Guards | `…/writeback.rs` (`evaluate_kernel_writeback`); `hydrology/support_helpers_mod/state_access.rs`; `hydrology/02_guard_errors.rs` |
| Kernel phases (~10.8k lines) | `hydrology/kernel_phases_mod/` (runoff_reconciliation 1956, plant_percolation 1542, infiltration_evap 1332, lateral 1253, erod19 1143, …) |
| I/O writers | `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` (HBP `build_hbp_output` ~153); `crates/openwepp-hillslope-output/src/{hillslope_wat,hillslope_pass}.rs` (typed parquet) |
| MOFE transfers | `scheduler.rs:264-370` (`TransferInput`/`TransferOutput`, `[f64;24]`) |
