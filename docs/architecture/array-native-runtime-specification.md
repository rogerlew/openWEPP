# Array-Native Hot-Path Runtime Architecture — Specification

Status: **Ratified** — binding design authority for the perf re-architecture, ratified by [ADR-0025](../decisions/0025-array-native-hillslope-day-frame.md) on 2026-06-18
Audience: all contributors; binding design authority for the perf re-architecture program
Owner: Claude Code (architecture authoring) — implementation by Codex
Supersedes: the *incremental application* of [ADR-0023](../decisions/0023-array-authoritative-hot-path-state.md)
(dense authority by symbol/phase) — **not** its dense-authority principle, which this specification fulfils completely
Last updated: 2026-06-19

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

**Post-PERFDEEP05 binding direction:** the shipping path is no longer another narrow
compatibility-edge optimization. PERFDEEP05 removed the measured full-sync hotspot and
still measured `911.11 s`; that is over 100× the legacy `9.12 s` anchor and
`1.36x` slower than the `669.97 s` openWEPP activation reference. The remaining gap
requires a complete array-native per-OFE-day fast path: no symbol maps, writeback
payloads, registry lookups, or dense/logical refreshes inside phase execution.
Logical/symbol surfaces survive only at true I/O, replay, and diagnostic edges.

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
retiring seams one at a time. PERFDEEP02 then proved a full-registry temporary frame is a verified negative
benchmark (`2417 s`), and PERFDEEP03 proved a lane-owned compact hydrology island is correct but still not
an endpoint win (`1147.96 s` vs `669.97 s`). PERFDEEP04 profiled that no-go and found the dominant
opt-in-only hotspot is daily lane-dense resynchronization from logical/indexed surfaces (`33.49%`
inclusive). PERFDEEP05 removed that full sync and preserved H2637 identity, but the opt-in endpoint still
failed (`911.11 s` vs `669.97 s`). The replacement costs are daily cached-slot refresh, dense logical
writeback apply, `SymbolRegistry::id_of`, and dirty flush. The complete unit, taken to its conclusion, may
still be the whole per-OFE-day hot path; a partial island must first prove those remaining edge costs can be
removed before expanding.

### 1.5 PERFDEEP05 conclusion: stop seam shaving

PERFDEEP05 is the falsification point for partial dense-island seam work. It
proved three things at once:

1. The old `sync_from_writeback_surface` loop was real overhead and should not
   be restored.
2. Removing that loop is not enough; the hot path immediately exposes another
   compatibility boundary.
3. The 70x-class gap is representation-wide, not a localized hotspot.

Therefore, future performance packages must be judged against this rule:
**do not spend another package merely shaving `BTreeMap`, `SymbolRegistry`, or
writeback-payload costs while keeping those mechanisms in the OFE-day phase
loop.** Such work may be useful only if it is part of deleting those mechanisms
from the hot loop.

The next package is an architectural execution package, not another patch:
`PERFDEEP06 - Array-Native Fast-Path Frame Inventory and Execution Plan`.
It must enumerate the H2637 hot-loop working set, publication operands,
persistent lane state, borrowed forcing, and exact direct-frame phase API before
implementation resumes.

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
- **Scalars are named typed fields.** Keep the unit wrapper types (`WaterDepthMeters`, …) as field types
  where they exist; this preserves dimensional safety and satisfies *"pure functions over typed state."*
  Raw `f64` is the fallback only where a unit type does not exist. Layout-sensitive use of these wrappers
  must follow the §4.7 `#[repr(transparent)]` policy.
- **Array families become fixed arrays / struct-of-arrays**, not 24/N separate symbols. SoA layout for the
  per-layer and per-hour data keeps the hot inner loops cache-linear (the RSS lever — target a few-MB,
  L2/L3-resident working set, like legacy's COMMON blocks).
- **Forcing series are borrowed read-only** for the day, never copied into the frame per phase.
- **Historical slot scaffold (PERFDEEP01-05):** the existing
  `state_slots`/`flux_slots: Vec<Option<BoundaryValue>>` plus frozen `SymbolRegistry` ids is a verified
  shadow/compatibility scaffold, not the shipping fast-path representation. It proved identity and allowed
  bounded experiments, but PERFDEEP02/03/05 showed that id-backed slots plus logical-surface refresh,
  `BoundaryValue` enum dispatch, writeback payload application, and dirty flush still form an
  array-shaped compatibility layer. Stage 4+ must therefore use direct frame/view APIs for the migrated
  phase chain: no `BoundarySymbol`, `SymbolRegistry::id_of`, `BoundaryValue`, `Option<BoundaryValue>`,
  `KernelWritebackPayload`, or logical-surface fallback on the normal success path. `HillslopeLaneDenseState`
  remains useful as a transition adapter and negative/identity benchmark; it is not an acceptable production
  end state.

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

### 4.6 Ownership — persistent lane-owned state, NOT a temporary mirror (PERFDEEP02 lesson, binding)
The dense frame must be the **carried, lane-owned runtime authority**, not a snapshot the scheduler rebuilds
around the logical maps. **PERFDEEP02 proved the anti-pattern is fatal:** it kept the logical/indexed
surfaces as the real runtime state and built a `Vec<Option<BoundaryValue>>` frame sized to the **full
registry (~4038 slots)** and **re-seeded/flushed it per OFE-day** (×235,961) — a temporary dense mirror
around old maps. Result: H2637 **2417 s, a 3.6× regression** (commit `fa29c34b`, kept opt-in as a verified
negative benchmark). This is the PERFIDX dual-representation ceiling in frame form.

Binding rules for every migration stage:
- **The lane runtime OWNS one persistent dense frame**; scheduler phases **borrow views** (`&`/`&mut`) into
  it. The scheduler does **not** create or reconcile a per-phase/per-day temporary frame.
- **Create the frame once** at lane-execution start; keep it alive across the full migrated phase chain;
  reads and kernel writebacks update it **in place**.
- **Hold the hot working set, not the full registry.** Dense storage over the whole
  bounded symbol universe is not practical; PERFIDX showed the reachable universe
  can explode far beyond the symbols actually touched by H2637. The frame stores
  lane-persistent scalars, fixed hourly arrays, soil/frost layer arrays, and
  phase-owned scalars. Climate forcing is **borrowed** (§4.1), not slotted;
  publication/diagnostic-only symbols are not hot-frame state.
- **Track dirty slots** with a compact dirty bitset / id list.
- **Materialize to logical/indexed only at true boundaries:** a non-migrated phase edge, output
  serialization, diagnostics/contract evidence, the external API. **No full-frame seed/flush loop inside
  scheduler phase execution.**

A partial island still pays a per-OFE-day *edge* cost (seed the read-set in, flush the dirty write-set out).
PERFDEEP03 bounded that state to a lane-owned compact hot set and preserved identity, but the real H2637
endpoint still measured `1147.96 s`. Ownership is therefore necessary but not sufficient; follow-on work
must profile and remove the remaining edge/fallback costs before expanding the same island shape.
PERFDEEP05 confirmed this: direct dense transfer authority removed the full resync hotspot, but final H2637
still measured `911.11 s`; the remaining measured edge is cached daily refresh plus logical dense
writeback/flush compatibility. This is not a reason to revert; it is the reason
to stop treating the partial island as the shipping architecture.

### 4.7 Rust implementation gotchas and binding mitigations

Web guidance reviewed on 2026-06-19 reinforces the local PERFDEEP evidence: the win comes from deleting
allocation, indirection, enum/tag dispatch, and compatibility lookups from the hot loop, not from merely
renaming maps as arrays.

Post-ratification binding implementation rules (ADR-0025 Amendment 1, 2026-06-19):

- **Contiguity must be real.** Fixed-width hourly/layer families should use `[T; N]`, slices, or
  pre-sized vectors/boxed slices with one owned allocation. Rust guarantees array element contiguity and
  offset arithmetic for `[T; N]`; `Vec<T>` is a contiguous growable array whose initialized elements live in
  order in the allocation. Do not reintroduce per-symbol heap nodes or per-day rebuilt vectors for hot state.
- **Do not assume optional or enum storage is free.** Rust only guarantees `Option<T>` has the same layout as
  `T` for the documented reference, `Box`, function-pointer, `NonNull`, `NonZero*`, and transparent-wrapper
  cases. `Option<BoundaryValue>` is not covered by that guarantee. Production frame fields should be typed
  scalars plus explicit validity/dirty bitsets where absence is semantically required.
- **Unit wrappers need explicit layout policy.** If a unit newtype's layout or ABI equivalence to `f64` is
  relied on for arrays, FFI, SIMD, or reinterpretation, it must be `#[repr(transparent)]` over the scalar
  field and must not be transmuted without an unsafe proof. Rust's default representation only guarantees
  soundness-level layout properties, not field order or ABI.
- **Bounds checks stay safe and visible.** Hot loops should prefer iterator/zipped-slice forms, pre-sliced
  arrays, and explicit range assertions that let LLVM remove redundant checks. `get_unchecked` is a last
  resort and requires the repository's `unsafe` proof discipline plus profiling evidence.
- **No hot-loop allocation helpers.** `format!`, `String` construction, owned-key cloning, collection
  cloning, full logical materialization, and per-phase work-vector allocation are forbidden on the normal
  success path. When a small temporary collection is unavoidable, allocate it once at lane/run scope and
  reuse/clear it.
- **Measure representation, not just arithmetic.** Each stage must record H2637 endpoint/RSS plus at least
  one allocation or type-size check for the new hot-frame state. Microbenchmarks remain useful only as
  hypothesis generators; realistic endpoint timing remains authority.

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
5. **Allocation/type-size regression gates** for migrated hot-frame state: record frame slot/field counts,
   representative `size_of` / `-Zprint-type-sizes` output or equivalent, and evidence that no normal-path
   allocation helpers (`format!`, owned symbol cloning, collection rebuilds) remain in the migrated phase
   loop.
6. **Determinism + conservation gates** (`SC-*` closure invariants) green throughout.

**Kill-criteria (when to stop and re-think, stated up front):**
- Any stage that **cannot** be made bit-identical → stop; the divergence is a real defect or a
  contract-gap, adjudicate before proceeding.
- A lane-owned hydrology island does **not** beat the current endpoint meaningfully -> the current
  partial-island shape is falsified for production scale and the remaining gap is elsewhere. PERFDEEP03
  reached this stop point (`1147.96 s` vs `669.97 s`), so the next action is re-profiling, not default
  activation or blind expansion. PERFDEEP05 removed the identified full-sync hotspot and still measured
  `911.11 s`; the next action is the PERFDEEP06 fast-path inventory/API planning gate, not another
  compatibility-edge optimization or broad island expansion.

---

## 8. Staged Execution Plan (comprehensive, identity-gated)

The full rewrite, sequenced so each stage is independently identity-gated and endpoint-measured. Stages are
*committed scope of one program*, not "let's see if it's worth it" — PERFARCH03 already proved the floor.

| Stage | Scope | Gate | Expected endpoint |
|---|---|---|---|
| **0 — Frame scaffold** ✅ *(PERFDEEP01, conditional GO 2026-06-18)* | Define `HillslopeDayFrame` + slot schema + seed/flush + typed I/O capture (HBP scalars, manifest provenance, WB13/publication operands). No phase migrated yet; frame runs *beside* the maps (shadow). | Frame round-trips bit-identically; output parity green. | ~flat |
| **1 — Hydrology island core** ⚠️ *(PERFDEEP02 NO-GO; PERFDEEP03 lane-owned compact state NO-GO, 1147.96 s)* | PERFDEEP03 migrated the hydrology cluster to a **lane-owned persistent compact frame (§4.6)** with forcing borrowed and dirty boundary flush. | Identity passed; opt-in H2637 endpoint failed the hard `< 669.97 s` gate. No default activation. | **NO-GO** - re-profile before expanding |
| **2 — Hydrology edge closure** ⚠️ *(PERFDEEP05 sync removal NO-GO, 911.11 s)* | PERFDEEP05 removed `sync_from_writeback_surface` from the opt-in daily hot loop and applied transfer directly to dense lane state. Remaining measured costs are cached daily refresh, logical dense writeback apply, symbol lookup, and dirty flush. | H2637 identity passed; endpoint still failed the `< 669.97 s` gate. This closes the edge-shaving experiment as insufficient. | **NO-GO** - stop seam shaving |
| **3 — Fast-path inventory and API** *(PERFDEEP06 next)* | Enumerate the H2637 hot-loop frame: persistent scalars, fixed arrays, layer SoA, borrowed forcing, phase-owned outputs, and publication operands. Define direct-frame phase APIs and prove which logical surfaces remain only at I/O/replay/diagnostic edges. Include a layout/type-size ledger and allocation-risk checklist from §4.7. | Static no-hot-loop-map design proof; publication operand ledger; package sequence for direct-frame ports; no production activation. | planning gate |
| **4 — Direct-frame hydrology fast path** | Port the complete hydrology daily OFE chain over `&mut HillslopeDayFrame`: no `HillslopeKernelRequest`, no `KernelWritebackPayload`, no `HillslopeWritebackSurface`, no `SymbolRegistry` lookup between hydrology phases. | Shadow bit-identity for migrated phases; H2637 HBP/WAT/PASS identity; endpoint/RSS measurement. | must move endpoint materially |
| **5 — Complete OFE-day frame path** | Port erosion, growth/decomposition, transitions, and closure diagnostics so all 14 phases mutate one frame. | full H2637 identity + endpoint + RSS; no logical/symbol surfaces in phase execution. | **the viability-gate measurement** |
| **6 — Delete logical hot-path plumbing** | Remove `HillslopeWritebackSurface`, indexed mirrors, writeback payloads, and registry build from the per-OFE-day loop. Logical/symbol surfaces survive only in intake, I/O serialization, replay, and diagnostics. | full H2637 identity + endpoint + RSS; **≤10× / ≤5× check.** | shipping gate |

Each stage is one work-package (`PERFDEEP0N`), identity-gated, endpoint-timed; default activation only when
the opt-in path beats the baseline endpoint. Unlike the incremental rungs, the target frame's *internal*
phase-to-phase seams vanish. PERFDEEP02, PERFDEEP03, and PERFDEEP05 prove that a
partial island still pays a per-OFE-day edge cost large enough to dominate the
physics. The next direction is therefore not another compatibility-edge rung:
PERFDEEP06 must design the complete fast path, and subsequent packages must port
complete direct-frame execution units.

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
| **Option/enum layout bloat** (`Option<BoundaryValue>` or large enums hiding in arrays) | Treat slot/enum forms as transition-only; require type-size evidence and explicit validity/dirty bitsets for production frame absence semantics. |
| **Bounds-check or unsafe regression** | Prefer iterator/slice/range-assertion forms; any unchecked access requires a local invariant proof, tests, and profiling evidence that the safe form is inadequate. |

---

## 11. Relationship to ADRs & Ratification

- **ADR-0022** (indexed runtime surface / `SymbolId`): retained for I/O serialization + replay; **removed
  from the simulation hot path** (the frame replaces the read mirror). Not contradicted — its scope
  narrows.
- **ADR-0023** (array-authoritative hot-path state): this specification is its **completion**. ADR-0023's
  dense-authority principle stands; its *incremental, symbol-by-symbol application* (PERFMIG01/02) is
  superseded by the whole-frame approach. ADR-0025 records this supersession explicitly.
- **Authority (ratified 2026-06-18):** ADR-0025 is the accepted hot-path runtime authority and this
  specification is its binding design authority. ADR-0023's incremental application is superseded — no
  further writeback-only or materialization-retirement rungs.
- **ADR-0019/0020** (output schemas), **ADR-0012** (HBP authority), **ADR-0004** (subprocess model):
  unaffected.
- **Kernel boundary** ([architecture/README.md](README.md)): this specification **fulfils** the declared
  "pure functions over typed state" boundary.

**Ratification record:** ADR-0025 — *"Adopt the array-native HillslopeDayFrame as the hot-path runtime
architecture"* — cites this document as design authority, records the supersession of ADR-0023's
incremental application, and gates execution on the §7 identity discipline. Execution proceeds as the
`PERFDEEP0N` work-package series (§8) under ADR-0025 authority.

---

## 12. Open Design Decisions (for implementation packages / future authority changes)

These are genuine forks left to the implementing ADR + Codex, not dictated here:

1. **Frame layout:** array-of-structs vs struct-of-arrays for the per-layer/per-hour data. SoA favours the
   cache-linear inner loops (recommended); AoS may be simpler for sparse access. Decide per measured hot loop.
2. **Unit-typed fields vs raw `f64`:** recommended typed (zero-cost, preserves the kernel-boundary
   contract); raw `f64` only where no unit newtype exists. Unit wrappers used in layout-sensitive arrays
   must state whether `#[repr(transparent)]` is required.
3. **Frame ownership model:** one `&mut HillslopeDayFrame` threaded through the pipeline vs a
   producer/consumer borrow-split per phase. The borrow-split better enforces trajectory ownership but is
   more invasive; decide against the real phase data-dependencies.
4. **Guard-schema representation:** const field-bound table vs per-field validating newtype constructors.
5. **Shadow-run mechanism:** a compile-time feature flag running both paths, vs a test-harness-only
   differential. Affects how long the logical path stays compiled into the hot binary.
6. **Diagnostic attribution policy:** preserve symbol-oriented violation subjects at boundaries, or ratify a
   field-oriented subject contract with explicit compatibility notes and fixtures.
7. **Validity representation:** decide per frame family whether absence is impossible, represented by a
   typed sentinel authorized by a contract, or represented by a compact bitset. `Option<BoundaryValue>` is
   transition-only unless a later endpoint and type-size gate explicitly re-authorize it for a non-hot edge.

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
| Current transition frame/adapter | `crates/openwepp-hillslope-orchestrator/src/day_frame.rs` (`HillslopeDayFrame`, `HillslopeLaneDenseState`, dirty-id flush and symbol-registry bridges) |
| Current dense read fallback | `hydrology/support_helpers_mod/state_access.rs` (`indexed -> dense slot -> logical surface` read chain, a compatibility path not the Stage-4+ target) |

## Appendix B — External implementation references

These references are guidance inputs only; local PERF evidence and openWEPP science contracts remain the
authority for acceptance.

- Rust `BTreeMap` background: ordered maps trade cache behavior, comparisons, and indirection; the current
  hot path uses this ordered logical property where deterministic export matters, but not where daily phase
  state needs direct lane-local mutation:
  <https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#background>.
- Rust `Vec` guarantees: `Vec<T>` is a contiguous growable array; capacity planning avoids reallocations,
  but `Vec` is still an owned allocation and should not be rebuilt per phase/day in the hot loop:
  <https://doc.rust-lang.org/std/vec/struct.Vec.html>.
- Rust Reference, type layout: arrays have contiguous element layout; default `repr(Rust)` gives only
  soundness-level layout guarantees; `#[repr(transparent)]` gives a one-field wrapper the same layout/ABI as
  its non-zero-sized field: <https://doc.rust-lang.org/reference/type-layout.html>.
- Rust `Option` representation: null-pointer/niche optimization is guaranteed only for the documented type
  set, not arbitrary enums such as `BoundaryValue`:
  <https://doc.rust-lang.org/std/option/index.html#representation>.
- Rust Performance Book, heap allocations: allocations, `format!`, cloning, and collection rebuilds are
  normal causes of hot-path cost; reusable workhorse collections and allocation checks are recommended
  where profiling shows the site is hot:
  <https://nnethercote.github.io/perf-book/heap-allocations.html>.
- Rust Performance Book, type sizes and bounds checks: measure hot type layout, guard against accidental
  size regressions, and use iterator/slice/range-assertion forms before considering unchecked indexing:
  <https://nnethercote.github.io/perf-book/type-sizes.html> and
  <https://nnethercote.github.io/perf-book/bounds-checks.html>.
- Rust Performance Book, profiling and benchmarking: optimize profiled hot paths, use realistic workloads,
  and treat endpoint timing/RSS as the production signal:
  <https://nnethercote.github.io/perf-book/profiling.html> and
  <https://nnethercote.github.io/perf-book/benchmarking.html>.
