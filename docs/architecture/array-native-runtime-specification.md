# Array-Native Runtime Architecture Specification

Status: **Ratified, Revision 6** - binding design authority for the
performance re-architecture, ratified by
[ADR-0025](../decisions/0025-array-native-hillslope-day-frame.md) on
2026-06-18, revised after PERFDEEP07 on 2026-06-19, and amended by
[ADR-0026](../decisions/0026-stateful-winter-column-sub-solver.md) on
2026-06-23 for the snow/frost winter-column exception. Revision 5
(2026-06-24) reconciles the executed R7G winter-column build (the sub-solver
was implemented and the snow/frost retrofit deleted) and records the
operator-directed opt-in-complete re-sequence: frost-depth fidelity is
decoupled from array-native bit-parity into reopened `GAP-SNOWFREEZE-002`,
and direct mode completes opt-in while default activation defers until frost
is contract-correct. Revision 6 (2026-06-30), amended by
[ADR-0030](../decisions/0030-r7-terminal-contract-and-compatibility-runtime-deletion.md),
records that frost ratification/default activation has closed that deferral:
production direct mode is the normal hillslope execution path, and obsolete
compatibility transition modes may be deleted under no-regression/static-proof
gates while the explicit replay seam remains deprecated and diagnostic-only.
Revision 6 also records the direct-publication RSS arc (the endpoint is now
run-length-flat; §4.11, §8.2) and reframes the final compatibility removal as a
**single-authority re-architecture of the seed/setup layer**, not a clean
deletion — the typed frame is still seeded from a computed symbol-map day-zero
surface, and making it typed-from-parse is the last single-authority step
(§1.3, §8.2).
Audience: all contributors working on runtime, scheduler, kernel, publication,
or performance packages.
Owner: architecture authority; implementation by Codex work packages.
Supersedes: the incremental application of
[ADR-0023](../decisions/0023-array-authoritative-hot-path-state.md), not its
dense-authority principle.
Last updated: 2026-06-30.

---

## 0. Summary

openWEPP's performance target is not reachable by making the current
symbol/logical/writeback runtime cheaper. PERFDEEP07 proved the remaining
default-disabled regression is not just a dense-first lookup bug: removing that
tax and replacing hot `BTreeMap` lookups with `HashMap` improved H2637 from
PERFDEEP05's `701.95 s` to `685.85 s`, but still failed the `<= 676.67 s`
P0 gate. Removing the production indexed runtime outright was worse
(`753.38 s` and `755.48 s`). The compatibility substrate is therefore
load-bearing and costly; it cannot be patched into the architecture we need.

The required architecture is a **complete, validated rewrite of the runtime
representation**:

```text
parsed inputs
  -> typed run schema
  -> typed run/lane/day frames
  -> static direct-frame executor
  -> typed publication projection
  -> legacy-compatible output serialization
```

The normal runtime must not be:

```text
logical maps
  -> symbol registry
  -> indexed mirrors
  -> dense mirrors
  -> writeback payloads
  -> logical maps
```

The target is not another dense island, indexed mirror, or lookup cache. The
target is that the array-native frame is the only authoritative simulation
state during day/OFE/phase execution. Symbol-keyed maps, `BoundarySymbol`,
`BoundaryValue`, `SymbolRegistry`, `HotSymbolTables`,
`HillslopeWritebackSurface`, and `KernelWritebackPayload` survive only as
intake, output, replay, diagnostic, and shadow-validation adapters.

This is a complete rewrite in representation and execution architecture. It is
not a clean-room rewrite of science. Physics, guards, units, conservation
contracts, process order, and output schemas remain authoritative. Existing
byte/Arrow identity gates remain protected-output regression gates, but they
are not sufficient science-contract evidence by themselves; direct-frame work
must also carry contract-derived invariant, guard, unit, closure,
conservation, provenance, and operand-reconstruction evidence where those
obligations apply.

Target: **<=10x and preferably <=5x legacy WEPP on H2637**. The model supports
that target only if the whole per-OFE-day hot path is direct-frame. Partial
compatibility edge work has already failed enough times to be disallowed as the
shipping direction.

---

## 1. Motivation and Evidence

### 1.1 Viability Gate

The H2637 no-UI run is the performance gate used by the PERF work packages.
The current budget is not a nice-to-have optimization target; it decides
whether openWEPP is viable as the Rust simulation engine.

| Anchor | H2637 no-UI | us/OFE-day | x legacy | RSS |
|---|---:|---:|---:|---:|
| Legacy WEPP | `9.12 s` | `38.65` | `1.0x` | about `4.6 MB` |
| openWEPP activation reference | `669.97 s` | `2826` | `73.46x` | about `228 MB` |
| PERFDEEP05 default-disabled | `701.95 s` | `2975` | `76.97x` | about `229 MB` |
| PERFDEEP07 retained HOLD run | `685.85 s` | `2907` | `75.20x` | `229004 KB` |
| <=10x budget | `91.2 s` | `386` | `10x` | - |
| <=5x budget | `45.6 s` | `193` | `5x` | - |

H2637 has `235,961` OFE-days. The PERFDEEP07 row is the best single retained
HOLD run, not an accepted three-run median and not a passing baseline. The
measured gap is representation-wide: pointer-heavy maps, dynamic symbol
resolution, enum dispatch, allocation, payload construction, and compatibility
refresh/flush dominate over physics.

### 1.2 Physics Is Not the Runtime Floor

PERFARCH03 ran one real WB11 warm-rain branch two ways on the same inputs:

| Path | us/OFE-day | Meaning |
|---|---:|---|
| Production logical kernel | `140.83` | symbol-keyed reads and writeback payload |
| Fully array-native branch | `0.96` | direct dense read/compute/write, `to_bits()` identical |
| Arithmetic only | `0.075` | actual branch math |
| One-shot logical materialization seam | `108.07` | dense-to-logical phase edge |

The arithmetic is not the limiting factor. The runtime representation is.

### 1.3 Incremental Rungs Failed as a Class

The program has already tested the major partial strategies:

- **PERFMIG01**: dense writeback for one phase, then immediate
  dense-to-logical materialization. Result: flat/negative.
- **PERFMIG02**: retire a small set of materialized symbols. Result:
  flat/negative because most symbols remained publication/reporting relevant.
- **PERFDEEP02**: full-registry temporary frame. Result: `2417 s`, a large
  regression.
- **PERFDEEP03**: lane-owned compact hydrology dense state. Identity passed,
  endpoint failed at `1147.96 s`.
- **PERFDEEP05**: remove full dense resynchronization. Identity passed,
  endpoint failed at `911.11 s`.
- **PERFDEEP07**: remove dense-first tax in the disabled path and improve hot
  lookup tables. Identity passed, endpoint still failed at `685.85 s`.
- **Seed-layer read burn-down (2026-06-30)**: after the hot path was direct,
  migrating the ~207 day-zero seed reads off the symbol-map surface one at a
  time moved `208 → 207` before holding. The day-zero seed authority is a
  *computed ordered pipeline* (`seed_wb11_runtime_surface_inputs`: WB18/19
  controls, hyetograph, initial layer stores, fine-frost, ET-demand,
  `efflen`/WB16, MOFE03/Wave-2), not independent reads that can be peeled off —
  and a typed carrier that merely *wraps* that symbol-map computation is a
  **false single-authority**. The seed/setup layer, like the hot path, must be
  replaced **wholesale** (a typed re-implementation of the computation), not
  peeled or wrapped.

These are not isolated misses. They show that partial migration keeps paying
the old runtime's boundary costs while adding new representation management.
The program must now move toward a complete validated rewrite, not another
compatibility-edge rung.

### 1.4 PERFDEEP07 Binding Lesson

PERFDEEP07 is the architecture correction point.

Confirmed:

- Dense-first lookup when dense surfaces are absent was real overhead.
- Hot `BTreeMap<String, _>` lookups were avoidable overhead.
- Fixing those issues preserved protected output identity.

Also confirmed:

- The current default path still builds and uses registry/hot-table/indexed
  runtime authority.
- Removing that indexed runtime and falling back to the older plain path is
  slower.
- The architecture cannot be repaired by toggling between old compatibility
  representations.

Therefore, zero-cost-disabled cannot mean "make all compatibility layers cheap
when off." It must mean **compatibility layers are not part of the normal
executor at all**.

---

## 2. Core Architecture Thesis

1. **Array-native is the canonical runtime, not an opt-in island.**
   The shipping executor owns typed frames and runs direct phase functions over
   those frames. Logical/indexed/dense compatibility surfaces are adapters.

2. **Compatibility is edge-only.**
   Symbol names and logical maps belong at intake, output serialization,
   replay, diagnostics, and shadow validation. They do not belong in
   day/OFE/phase execution.

3. **Mode is selected once.**
   The scheduler does not branch per phase between logical, indexed, dense, and
   direct-frame paths. Run initialization selects a direct-frame executor, a
   compatibility executor, or a shadow executor. The hot loop receives a single
   concrete execution plan.

4. **Phases receive typed views, not generic requests.**
   A phase should not ask "which symbol do I need?" during execution. Its API
   exposes the state, forcing, transfer buffers, and outputs it owns.

5. **Publication is a projection edge.**
   HBP, WAT, PASS, loss, manifests, and diagnostic outputs keep their schemas,
   but their source data comes from typed publication projection structures
   rather than runtime symbol surfaces.

6. **Validation is continuous.**
   Complete rewrite does not mean unchecked big-bang replacement. It means the
   destination architecture is complete, while migration is shadowed,
   identity-gated, endpoint-measured, and contract-first where authority
   changes are required.

---

## 3. Current Runtime We Are Replacing

The current scheduler executes:

```text
years
  -> days
  -> OFE lanes
  -> 14 phase DAG
  -> HillslopeKernelRequest
  -> KernelWritebackPayload
  -> HillslopeWritebackSurface
```

The hot runtime representation is a combination of:

- `BTreeMap<BoundarySymbol, BoundaryValue>` state and flux surfaces;
- frozen `SymbolRegistry`;
- `IndexedWritebackSurface`;
- `HotSymbolTables`;
- optional dense slot views and lane dense state;
- `KernelWritebackPayload` and indexed payload application;
- logical refresh/flush at migration boundaries.

That design has useful authority properties: deterministic export order,
symbol-oriented diagnostics, flexible shadow comparisons, and compatibility
with existing publication machinery. It is not acceptable as the normal hot
runtime.

The 14 phases and topology remain:

```text
Normalization
StorageBounds
DecompositionTransition
ResiduePartitionTransition
AnnualGrowthTransition
PerennialGrowthTransition
PercolationDeepSeepage
Evapotranspiration
Drainage
LateralTransfer
PlantRootUptake
RunoffReconciliation
StorageReconciliation
ClosureDiagnostics
```

The process order is preserved. The representation and dispatch mechanism are
replaced.

---

## 4. Target Architecture - Direct Frame Runtime

### 4.1 Runtime Layers

The greenfield runtime has five layers:

1. **Schema layer**
   A frozen typed schema declares every state field, flux field, forcing field,
   transfer buffer, management slot, layer vector, and publication operand used
   by simulation.

2. **Frame layer**
   Run, lane, day, phase, transfer, and publication frames own or borrow typed
   data. These frames are the authoritative state during execution.

3. **Executor layer**
   A static executor runs direct typed phase functions over frame views. It is
   chosen once at run initialization.

4. **Projection layer**
   Typed publication projection accumulates exactly the operands needed for
   HBP/WAT/PASS/loss/manifest output.

5. **Compatibility adapter layer**
   Symbol/logical/indexed surfaces are constructed only for intake, legacy
   replay, diagnostics, and shadow validation. They are not consulted by the
   normal executor.

### 4.2 Static Runtime Schema

The schema is the replacement for hot-loop symbol resolution.

It must define:

- field identity and stable names;
- units and dimensional wrappers;
- finite/domain bounds;
- producer phase and consumer phases;
- persistence lifetime;
- publication/replay/diagnostic exposure;
- legacy symbol aliases where compatibility requires them;
- array shape for hourly, layer, frost, management, and MOFE families;
- default/absence semantics;
- conservation and closure obligations.

The schema may be generated or handwritten, but the hot executor consumes
compiled offsets and typed fields. It does not resolve strings.

Rules:

- Legacy symbol names remain for provenance and diagnostics, but they are not
  runtime keys.
- Deterministic ordering is an export concern, not a storage concern.
- A new hot field requires schema ownership, guard rules, tests, and
  publication/replay disposition.
- A deleted compatibility symbol requires proof that no output, diagnostic,
  replay, or contract obligation still needs it.

### 4.3 Frame Hierarchy

The canonical frame hierarchy is:

```text
HillslopeRunFrame
  static topology, run constants, schema, output identity, static input tables

HillslopeLaneFrame
  OFE geometry, soil/frost/layer state, persistent carryover, management state

HillslopeDayFrame
  daily forcing, mutable phase state, daily fluxes, closure accumulators

PhaseView<'a>
  narrow borrowed view for one phase

TransferBuffers
  typed upstream/downstream OFE transfer channels

PublicationFrame
  typed output operands and provenance rows
```

The frame hierarchy must avoid whole-registry dense mirrors. It stores the hot
working set and the state needed for output projection. It does not store every
possible legacy symbol as an optional slot.

### 4.3.1 Direct-Frame Type Boundary

The existing `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
`HillslopeDayFrame` is a compatibility transition frame: it is registry-backed
and stores `BoundaryValue` slots. That type is useful as shadow evidence and
negative/transition scaffolding, but it is not the direct-mode target described
by this specification.

R0 must make an explicit type-boundary decision before runtime implementation:

- rename or isolate the existing compatibility frame so it cannot be mistaken
  for the direct-mode frame; or
- introduce a distinct direct-frame type.

The R0 gate must prove the direct-mode frame type contains no registry,
`BoundarySymbol`, `BoundaryValue`, `Option<BoundaryValue>`, indexed-surface, or
logical-surface fields in normal execution storage. Any remaining compatibility
frame must be clearly marked as an adapter or shadow type.

### 4.4 HillslopeRunFrame

`HillslopeRunFrame` owns or references data that does not change per day:

- schema metadata and compiled field offsets;
- static OFE topology and ordered phase plan;
- soil profile structure and maximum layer dimensions;
- management schedule descriptors;
- output identity and run provenance;
- configuration flags selected before execution;
- preallocated work buffers reused by lower layers.

No per-day allocation or symbol registry build is allowed inside the run loop.

### 4.5 HillslopeLaneFrame

`HillslopeLaneFrame` is the lane/OFE-owned persistent state. It replaces
`OfeLanePersistentState` as a symbol-surface container.

It owns:

- persistent water, frost, snow, residue, plant, and decomposition state;
- layer/fine-layer arrays as struct-of-arrays where access is columnar;
- management slot state;
- current transfer state;
- reusable phase buffers and dirty/validity bitsets.

Start-of-day creates a `HillslopeDayFrame` by borrowing or moving typed
substructures from the lane frame. End-of-day commits persistent fields by
typed assignment, not by logical surface rebuild.

### 4.6 HillslopeDayFrame

`HillslopeDayFrame` is the authoritative mutable state for one OFE day.

Conceptual shape:

```rust
pub struct HillslopeDayFrame<'run> {
    pub state: HydrologyState,
    pub flux: HydrologyFlux,
    pub soil: SoilLayerColumns,
    pub frost: FrostLayerColumns,
    pub plant: PlantState,
    pub residue: ResidueState,
    pub decomposition: DecompositionState,
    pub transfer: TransferBuffers,
    pub forcing: DayForcingView<'run>,
    pub publication: PublicationOperands,
    pub guards: GuardScratch,
}
```

Binding rules:

- Named typed fields are preferred for scalars.
- Fixed-size families use `[T; N]` where size is fixed by science or file
  format.
- Variable-size families use preallocated slices, `Box<[T]>`, or owned vectors
  allocated at run/lane setup, not rebuilt per phase.
- Climate and management series are borrowed read-only views where possible.
- Absence is represented by typed validity bitsets or contract-authorized
  sentinels, not `Option<BoundaryValue>` in hot storage.
- `BoundaryValue` is transition-only and forbidden in the normal direct-frame
  executor.

### 4.7 Phase Views

Each phase receives a narrow typed view:

```rust
pub fn run_wb11_hydrology(view: HydrologyViewMut<'_>) -> Result<(), HydrologyError>;
pub fn run_wb13_publication(view: PublicationViewMut<'_>) -> Result<(), PublicationError>;
```

The view exposes:

- immutable inputs the phase consumes;
- mutable outputs the phase owns;
- transfer buffers the phase is authorized to read/write;
- guard and diagnostic sinks required by the contract.

The view must not expose generic symbol lookup. The borrow structure should
make most producer/consumer mistakes unrepresentable. Where Rust borrowing
cannot express a dynamic dependency, the schema and executor validate the
dependency before the run starts.

### 4.7.1 Stateful Sub-Solver Exception

The ordinary direct-frame rule is feed-forward typed phase execution. ADR-0026
ratifies one narrow exception: snow/frost is implemented as a stateful
winter-column sub-solver.

The exception is limited to internal solver shape. The winter column owns
persistent lane state, including distinct typed snow and frost sub-states, and
may run hourly loops or staged pre/post hydrology steps over one mutable
`DirectWinterColumnState`. The outer executor still sees a typed day-level
producer that emits typed downstream operands and publication operands.

The exception does not allow hot-loop compatibility authority. Production
winter-column execution must not construct or consult `DirectFrostRunoffSurface`,
`HillslopeKernelRequest`, `HillslopeWritebackSurface`, `BoundarySymbol`,
`BoundaryValue`, compatibility WB13 rows, or map-backed symbol helpers except in
named test/comparator adapters.

### 4.8 Static Executor

The executor is built once:

```rust
let executor = DirectFrameExecutor::new(schema, plan, buffers)?;
executor.run_hillslope(&mut run_frame)?;
```

The executor:

- owns the fixed phase order;
- owns branch mode selection;
- holds reusable work buffers;
- iterates days and lanes;
- calls direct typed phase functions;
- updates transfer buffers;
- records publication projection operands;
- emits typed status and guard diagnostics.

The executor must not:

- construct `HillslopeKernelRequest`;
- construct `KernelWritebackPayload`;
- build or query `SymbolRegistry`;
- build or query `HotSymbolTables`;
- build `HillslopeWritebackSurface`;
- perform dense/logical refresh or dirty flush;
- allocate strings or use `format!` in the normal success path;
- choose runtime representation inside the per-phase loop.

### 4.9 Transfer Channels

MOFE routing is already close to the target. The rewrite keeps the transfer
concept but makes it first-class typed frame state:

- surface carry arrays;
- lateral carry arrays;
- deep drainage and seepage channels;
- sediment and erosion channels;
- snow/frost/hydrology coupling channels;
- upstream/downstream area ratios and closure terms.

Transfer buffers have explicit producers and consumers. They are not inserted
into symbol maps between phases.

### 4.10 Guard and Error Model

Fail-closed semantics are preserved.

Guards move from symbol-read/writeback-field checks to typed field checks:

- static finite/domain checks from schema;
- runtime-derived bounds from phase context;
- conservation/closure checks from canonical `SC-*` contracts;
- diagnostic attribution preserving legacy subject semantics where required;
- typed error enums, not broad boxed errors.

Guard diagnostics may mention legacy symbols, but symbol lookup is performed
by diagnostic projection at the edge, not in the phase arithmetic path.

### 4.11 Memory and Layout Rules

The direct frame runtime is a layout-sensitive design.

Binding rules:

- Arrays and slices must be genuinely contiguous.
- Unit wrappers used in layout-sensitive arrays must have an explicit layout
  policy, normally `#[repr(transparent)]` over the scalar.
- `Option<T>` is allowed only where its layout and semantics are explicit; it
  is not a default absence mechanism for hot arrays.
- Avoid enum-tag dispatch in inner loops.
- Reuse work buffers allocated at run/lane setup.
- Prefer iterator, zipped-slice, and pre-sliced loop forms before considering
  unsafe indexing.
- Any `unsafe` must carry a local invariant proof and be justified by
  profiling after safe forms were tried.
- **Publication and per-run state must not retain whole-run rows.** The
  publication path streams per-day/per-OFE rows to the output sink and drops
  them; it must not accumulate all `DirectPublicationDayRow`/WAT/PASS rows for
  the run, nor clone the publication execution. Parquet writers flush
  incrementally by row group (value/schema/row-count identity is the gate for
  parquet; row-group layout may differ — see §5.2).
- **Setup must not pre-allocate per-day×OFE structures.** Day inputs are
  constructed dynamically; a `Vec<DirectDayConstructorInputs>` (or equivalent)
  sized to `days × OFEs` is forbidden — it was the dominant H2637 RSS cost
  (~909 MiB for 235,961 rows), not the symbol-map carrier.
- **RSS must be run-length-flat.** Endpoint resident memory must not scale with
  `days × OFEs`. RSS is a first-class gate, measured at multiple run lengths;
  the slope must flatten. The PERFARCH03 hot-path working-set floor is ~3 MB; a
  full publishing run adds only bounded per-emit and aggregate buffers, not
  whole-run retention. (Arc: direct H2637 publication went 1.13 GiB → 110 MiB
  full / 51 MiB required-only, byte/value-identical and run-length-flat.)

### 4.12 Runtime Modes

The public runtime architecture has one mode:

1. **Direct mode**
   The production target. Runs only typed frames and typed projections.

Removed historical modes:

- **Compatibility mode** had maintained the logical/indexed runtime for replay,
  comparison, and fallback during validation. ADR-0031 removes the public
  selector; any remaining symbol-keyed code is a held support-boundary deletion,
  not a public execution mode.
- **Shadow mode** ran direct and compatibility paths together for selected
  fixtures/packages. It was a validation harness, not a shipping hot path, and
  public transition selectors are deleted.

Mode selection is outside the hot loop.

---

## 5. Compatibility and I/O Edges

### 5.1 Intake Edge

Input parsing may still use legacy names and flexible intermediate maps. The
edge contract is:

```text
legacy/config input -> parsed input model -> typed run frame
```

After frame construction, the executor uses typed fields only.

### 5.2 Publication Edge

Output schemas do not change.

| Output | Target source |
|---|---|
| HBP | `PublicationFrame` typed scalar operands |
| WAT parquet | typed WAT row builder from `PublicationFrame` |
| PASS parquet | typed PASS row builder from `PublicationFrame` |
| loss JSON | static/run projection plus typed accumulators |
| run manifest | typed provenance and execution trace |
| diagnostic traces | typed diagnostic events projected to legacy names as needed |

Publication projection is a first-class part of the runtime, not an
afterthought. Each publication operand has:

- producer phase;
- units;
- source frame field;
- legacy symbol alias if any;
- output row/column destination;
- identity fixture;
- anti-alias fixture when multiple legacy symbols map to related frame fields.

The PERFDEEP06 publication operand ledger is the current normative seed for
this projection work. R6 and any package that touches publication operands must
either promote that ledger into canonical architecture/contract authority or
update this specification with an equivalent binding ledger before cutover.
The ledger must include anti-alias fixtures, metadata/provenance parity, output
row/column mappings, and independent operand reconstruction for
conservation-sensitive outputs.

#### 5.2.1 R6 Canonical Publication Operand Ledger

This subsection promotes the PERFDEEP06 publication operand ledger into
canonical architecture authority for R6 direct publication cutover. Package-local
ledger artifacts may add evidence, but they do not supersede this table unless
this specification or a canonical `SC-*` output/publication contract is amended.

Every `PublicationFrame` field below is direct-mode authority only after it is
populated from typed direct run/lane/day state. A compatibility WB13 row,
runtime symbol, writeback payload, diagnostic ledger, stale logical surface, or
output row reconstructed from one of those compatibility structures is not a
valid direct source for R6 cutover.

| Output family / field | Units / basis | Canonical direct source | Producer phase | Legacy alias | Destination | Wrong aliases to reject | Current-scope acceptance gate |
|---|---|---|---|---|---|---|---|
| HBP `peakro`; PASS `peakro` | `m^3/s`; diagnostic peak runoff | `publication.erosion.peak_runoff_m3_s` | `RunoffReconciliation` / erosion publication handoff | `peakro` | HBP event header; PASS `peakro` | daily `Q`, `QOFE`, `runvol`, runoff volume | HBP byte identity; PASS Arrow row/schema/metadata parity; fixture with nonzero peak runoff; independent reconstruction from direct runoff/event-duration operands. |
| HBP `watdur` | seconds; event duration | `publication.runoff.runoff_duration_s` | `RunoffReconciliation` | `watdur` | HBP event header | day length, storm duration, irrigation duration, hyetograph duration | HBP byte identity; fixture where runoff duration differs from day/storm durations. |
| HBP/PASS `tdet`, `tdep` | kg; erosion diagnostics | `publication.erosion.total_detachment_kg`, `publication.erosion.total_deposition_kg` | erosion publication handoff | `total_detachment_kg`, `total_deposition_kg` | HBP event header; PASS `tdet`, `tdep` | sediment concentration, particle fractions, class totals, zero default on erosion-active run | HBP byte identity; PASS Arrow parity; erosion-active fixture; independent mass reconstruction when sediment authority is in scope. |
| HBP/PASS `sedcon_1..5` | `kg/m^3`; particle concentration classes | `publication.erosion.sediment_concentration_kg_m3[0..5]` | erosion publication handoff | `sediment_concentration_kg_m3_0001` and future class aliases | HBP class concentration; PASS `sedcon_1..5` | particle flow fraction, detached/deposited mass, all-zero classes when class 1 is nonzero | HBP byte identity; PASS fixture distinguishing class 1 from zero classes before changing PASS behavior; independent concentration reconstruction when sediment authority is in scope. |
| WAT `P` | mm over row area | `publication.climate.precipitation_mm` | `Normalization` | `prcp` | WAT `P` | `RM`, rainfall-only, snowmelt, irrigation | WAT Arrow row/schema/metadata parity; precipitation/rainmelt anti-alias fixture. |
| WAT `RM` | mm liquid input | `publication.liquid_input.rm_mm` | `Normalization` / `RunoffReconciliation` | `RM`, derived from post-winter rain + routed melt + irrigation | WAT `RM` | `P`, irrigation alone, snowmelt alone, raw rainfall | WAT Arrow parity; fixture where rain, snowmelt, and irrigation differ; independent liquid-input reconstruction. |
| WAT `Q` | mm over effective publication length | `publication.runoff.q_mm` | `RunoffReconciliation` | `Q` | WAT `Q` | `QOFE`, `runvol`, physical `Q` in routed per-OFE mode | WAT Arrow parity; per-OFE fixture preserving current geometry formula; independent runoff-depth reconstruction. |
| WAT `QOFE`; PASS `runvol` | WAT mm; PASS `m^3` using outlet row area | `publication.runoff.qofe_mm`; `publication.pass.runvol_m3` | `RunoffReconciliation` / outlet delivery projection | `QOFE` | WAT `QOFE`; PASS `runvol` | `Q * area`, upstream area, publication-area sum, outlet-area shortcut | WAT/PASS parity; existing per-OFE anti-alias fixture; independent volume reconstruction from accepted outlet area and direct `QOFE`. |
| WAT `Ep`, `Es`, `Er` | mm | `publication.evaporation.ep_mm`, `publication.evaporation.es_mm`, `publication.evaporation.er_mm` | `Evapotranspiration` / `PlantRootUptake` | `Ep`, `Es`, `Er` | WAT `Ep`, `Es`, `Er` | ET total, raw negative-tolerance `Es`, seed branch flag, plant demand | WAT parity; fixture with separated Ep/Es/Er; independent ET component reconstruction. |
| WAT `Dp` | mm | `publication.subsurface.dp_mm` | `PercolationDeepSeepage` / `StorageReconciliation` | `D` plus frost bottom overflow | WAT `Dp` | base `D` alone, `watpdg`, `latqcc`, frost storage state | WAT parity; frost-bottom-water fixture; independent deep-percolation reconstruction. |
| WAT `UpStrmQ`, `SubRIn` | mm | `publication.transfer.upstream_surface_mm`, `publication.transfer.upstream_lateral_mm` | `LateralTransfer` | `TransferInput`, `SubRIn` | WAT `UpStrmQ`, `SubRIn` | current lane output arrays, unscaled area ratio, downstream output | MOFE fixture with non-1 area ratio; independent transfer reconstruction. |
| WAT `latqcc`; PASS `sbrunv` | WAT mm; PASS `m^3` using outlet area | `publication.subsurface.latqcc_mm`; `publication.pass.sbrunv_m3` | `Drainage` / `LateralTransfer` | `q`, `latqcc` | WAT `latqcc`; PASS `sbrunv` | `Qd`, `Qdd`, lateral output arrays, runoff volume | WAT/PASS parity; `q` vs state/flux preference fixture; independent lateral-volume reconstruction. |
| WAT `Tile` | mm | `publication.subsurface.tile_mm` | `Drainage` | `Qdd` | WAT `Tile` | `Qd`, `q`, lateral flow | WAT parity; fixture enforcing `Qd = latqcc + Tile`; independent drainage reconstruction. |
| WAT `Total-Soil`, `SoilWaterTotal` | mm | `publication.storage.total_soil_mm` and alias projection | `StorageReconciliation` | `wb11_soil_water`, `SoilWaterTotal` | WAT `Total-Soil`, `SoilWaterTotal` | `watcon` stale logical, frozen water alone, profile depth | WAT parity; hydout-equivalent closure fixture; independent storage reconstruction from layer vector plus accepted frozen storage. |
| WAT `frozwt`, `frdp`, `Snow-Water` | mm | `publication.storage.frozwt_mm`, `publication.storage.frdp_mm`, `publication.storage.snow_water_mm` | `StorageReconciliation` / snow-frost projection | `runtime_frwatc`, `runtime_frdp`, `runtime_swe` | WAT `frozwt`, `frdp`, `Snow-Water` | snow depth, frozen delta, profile depth, SWE before day | WAT parity; active snow/frost fixture with distinct values; independent snow/frost storage reconstruction. |
| WAT profile optional fields | mm | `publication.profile.depth_mm`, `publication.profile.porosity_cap_mm`, `publication.profile.fc_store_mm`, `publication.profile.wp_store_mm` | `StorageBounds` / profile projection | `wb13_profile_depth_mm`, FC/WP layer aliases | WAT `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore` | `solthk(nsl)` alone for FC, porosity for FC, layer 1 only | WAT metadata parity; anti-tautology profile fixture; independent layer-column reconstruction. |
| WAT `Interception` | mm | `publication.interception.interception_mm` | `Normalization` / interception projection | `I` | WAT `Interception` | interception storage, rainfall input, precipitation | WAT parity; fixture with nonzero interception flux and distinct storage. |
| WAT `InterceptionStorage` | mm or absent | `publication.interception.interception_storage_mm` when producer-authoritative; otherwise `None` | future interception-storage producer | none currently authoritative | WAT `InterceptionStorage` | daily interception flux | Preserve `None` until producer-authoritative storage exists; fixture must distinguish storage from flux before enabling. |
| loss JSON climate/run fields | JSON scalar metadata | `publication.loss.run_name`, `publication.loss.first_day`, `publication.loss.last_day`, `publication.loss.climate_day_count`, `publication.loss.executed_day_count`, `publication.loss.precipitation_mm` | run publication projection | runfile/climate span fields | loss JSON keys | optional-output payloads, first-day-only shortcuts except current schema field, executed vs climate day count | Byte-normalized JSON identity; schema/key parity; independent reconstruction from parsed run/climate span and execution counters. |
| loss JSON static sidecar fields | JSON scalar metadata | `publication.loss.ofe_count`, `publication.loss.snow_override_applied`, `publication.loss.frost_wint_red` | run publication projection | soil/snow/frost sidecar fields | loss JSON keys | output row counts, runtime snow/frost state | Byte-normalized JSON identity; sidecar anti-alias fixture; independent reconstruction from parsed static inputs. |
| run manifest input/output checksums | checksum metadata | `publication.manifest.input_checksums`, `publication.manifest.output_checksums` | run publication projection | manifest checksum helpers | manifest JSON maps | stale output paths, optional-output duplicates, checksum order | Manifest schema parity; checksum parity; independent checksum recomputation. |
| run manifest provenance/counters | execution metadata | `publication.manifest.runtime_selection`, `publication.manifest.direct_runtime_counters`, `publication.manifest.warning_ids`, `publication.manifest.output_policy` | run publication projection | existing manifest provenance fields | manifest JSON provenance fields | compatibility row counts as direct authority, diagnostic-only counters, stale warning state | Manifest JSON parity for compatibility mode; direct-mode counter fixture; provenance anti-alias fixture. |

Publication metadata and row identity are also canonical operands, even when
they are not hydrology magnitudes:

| Metadata / identity field | Canonical source | Destination | Acceptance gate |
|---|---|---|---|
| `wepp_id`, `ofe_id` | `publication.identity.wepp_id`, `publication.identity.ofe_id` from typed run/lane/day context | WAT/PASS/HBP identity fields | Arrow parity across multi-OFE fixtures; guards require positive IDs. |
| `sim_day_index` | `publication.identity.sim_day_index` | WAT/PASS day identity | Arrow parity; fail-closed range guard. |
| `julian`, `month`, `day_of_month`, `water_year` | `publication.calendar` | WAT/PASS calendar columns; HBP latest event date | Calendar anti-alias fixture across water-year boundary; Arrow/HBP parity. |
| schema version and dataset metadata | output schema crates, not direct frame state | WAT/PASS parquet metadata; manifest schema ID | Existing schema metadata tests plus R6 metadata parity inspection. |
| field units/descriptions | output schema crates and unit registry | WAT/PASS field metadata | Unit registry validation; direct frame must not duplicate schema authority. |
| producer/provenance metadata | writer metadata plus `publication.manifest.*` | WAT/PASS metadata and run manifest | Metadata parity and independent manifest reconstruction. |

R6 cutover is blocked unless production direct mode can provide this
`PublicationFrame` from typed direct state. Building an object with these names
from compatibility WB13 rows or runtime surfaces is not cutover; it is an
adapter wrapper and must remain in compatibility or shadow mode.

### 5.3 Replay and Diagnostics Edge

Replay, audit, and diagnostics may construct symbol surfaces because their job
is to inspect or compare legacy-shaped data. They must be explicitly requested
and structurally outside direct mode.

Examples:

- symbol registry audit;
- indexed shadow reports;
- frame roundtrip reports;
- legacy comparator dumps;
- contract diagnostic extracts.

Disabled means no object construction and no per-read branch in direct mode.

### 5.4 Compatibility Adapter Ownership

Compatibility adapters are allowed only at declared boundaries. They must not
be hidden behind a convenience API that direct-frame phases can call.

Allowed:

- `TypedFrame::from_legacy_inputs(...)` during initialization;
- `PublicationFrame::to_hbp(...)` at output;
- `ShadowComparator::compare_frame_to_surface(...)` in shadow mode.

Forbidden in direct mode:

- `state_value_for_symbol(...)`;
- `flux_value_for_symbol(...)`;
- `SymbolRegistry::id_of(...)`;
- `HillslopeWritebackSurface` mutation;
- `KernelWritebackPayload` construction;
- dense/logical refresh or dirty flush.

---

## 6. Performance Model and Gates

### 6.1 Model

The model is simple: delete runtime representation overhead as a class.

| Quantity | Value | Basis |
|---|---:|---|
| Current reference OFE-day | `2826 us` | activation reference |
| PERFDEEP07 retained OFE-day | `2907 us` | still compatibility-backed |
| Array-native WB11 branch | `0.96 us` | PERFARCH03 |
| Legacy OFE-day | `38.65 us` | H2637 legacy |
| <=10x OFE-day budget | `386 us` | viability gate |
| <=5x OFE-day budget | `193 us` | aspirational gate |

The architecture assumes meaningful endpoint improvement only when entire
phase spans run without symbol/logical/writeback machinery. Microbenchmarks can
guide implementation, but H2637 endpoint/RSS is the authority.

### 6.2 Required Measurements

Every implementation package must record:

- H2637 endpoint seconds and RSS;
- protected HBP/WAT/PASS/loss/manifest identity;
- focused phase identity fixtures;
- allocation evidence for migrated hot loops;
- frame type-size or layout evidence;
- proof that direct mode does not construct compatibility surfaces;
- skipped gates with explicit blockers.

The no-compatibility proof must be executable, not just prose. Minimum proof:

- a direct-executor entrypoint allowlist and call-graph audit showing it does
  not enter `execute_with_kernel*`, `HillslopeKernelRequest`,
  `KernelWritebackPayload`, `HillslopeWritebackSurface`, symbol-registry,
  hot-table, indexed-surface, dense-refresh, or dirty-flush paths;
- H2637 runtime counters or audit hooks showing zero calls to the named
  forbidden APIs during direct phase execution;
- zero `BoundarySymbol` or owned legacy-symbol construction in direct phase
  execution;
- an explicit allowance list for edge-only compatibility adapters, if any are
  invoked before or after direct execution.

### 6.3 Stop Criteria

Stop and re-architect, not patch around, when:

- a migrated phase cannot be made identity-equivalent;
- a compatibility edge remains in the direct-mode hot loop;
- an opt-in direct path is slower than compatibility mode after edge removal;
- output projection still depends on runtime symbol surfaces;
- a package proposes another lookup/cache layer without deleting a runtime
  compatibility boundary.

---

## 7. Validation Strategy

The rewrite must be validated continuously.

### 7.1 Identity Classes

Required identity levels:

- scalar frame fields: `f64::to_bits()` equality unless a canonical contract
  authorizes bounded normalization;
- fixed arrays: element-by-element bit identity;
- typed rows: Arrow schema/table equality as already used by WAT/PASS gates;
- HBP/WAT where byte identity is currently expected: byte identity;
- metadata/provenance: exact equality except explicitly recorded run-name or
  path differences;
- diagnostics: message-id class and subject attribution parity.

### 7.2 Shadow Execution

Shadow mode runs direct-frame and compatibility execution together for selected
fixtures.

Shadow comparison must cover:

- seed frame fields;
- per-phase outputs;
- transfer buffers;
- end-of-day persistent projection;
- publication operands;
- final outputs.

The direct path becomes authoritative for a stage only after the shadow diff is
clean or every difference has a contract-backed disposition.

### 7.3 Contract Discipline

No physics, guard, unit, output-meaning, or diagnostic-attribution change is
implicit in this architecture. If a direct-frame implementation discovers that
authority must change, the sequence is:

1. amend the canonical `SC-*` contract or relevant ADR;
2. add contract-derived tests;
3. record pre-implementation contract gate evidence;
4. then change production runtime code.

### 7.4 Stage Classes and Completion Gates

Planning/schema stages may complete with static artifacts only when their
package scope explicitly excludes runtime execution, activation, and readiness
claims. They still require required-reading evidence, owned-file manifests,
contract-gate disposition, docs lint, review, and finding disposition.

Shadow-only implementation stages may complete without default activation, but
must run the executable shadow fixtures they introduce and record why any H2637
endpoint gate is not yet applicable. If they touch runtime code, they must run
the relevant Rust gates for that scope.

Activated direct-mode stages are runtime completion claims. They are not
complete until:

- focused tests pass;
- H2637 identity passes;
- H2637 endpoint/RSS is recorded;
- touched `SC-*` invariant and closure checks pass;
- legacy comparator delta review is recorded with confidence tiering;
- conservation-sensitive outputs have independent operand reconstruction
  evidence;
- `cargo fmt --check` passes;
- `cargo clippy --workspace --all-targets -- -D warnings` passes;
- `cargo test --workspace` passes;
- `cargo deny check` passes;
- scoped docs lint passes;
- review and verification findings are dispositioned.

HOLD is required when a stage claims runtime completion without executable
H2637 evidence, when known invariant/identity/performance gates remain
unresolved, or when science-contract closure evidence is missing for touched
physics, guards, publication, conservation, or output operands.

---

## 8. Rewrite Program

This is one architecture program, executed through bounded packages. Packages
may be incremental for validation, but the destination is not incremental.

### 8.1 Completed Negative Evidence

The following packages remain useful as evidence and fixtures, not as target
architecture:

| Package | Architectural disposition |
|---|---|
| PERFDEEP01 | useful scaffold and roundtrip evidence |
| PERFDEEP02 | negative benchmark for full-registry temporary frames |
| PERFDEEP03 | negative benchmark for partial lane dense island endpoint |
| PERFDEEP04 | profile evidence for dense-island sync costs |
| PERFDEEP05 | negative benchmark for edge-shaved dense island |
| PERFDEEP06 | inventory and API planning authority |
| PERFDEEP07 | default-disabled compatibility cleanup; superseded by PERFDEEP09 hold-lift evidence |
| PERFDEEP08 | rejected disabled-path hard-isolation candidate |
| PERFDEEP09 | default-disabled P0 hold-lift; R2+ implementation unblocked |

### 8.2 New Rewrite Sequence

Future work should follow this sequence.

#### 8.2.1 PERFDEEP07 Hold-Lift Gate

PERFDEEP07 was the original `HOLD` for default-disabled timing. PERFDEEP09
superseded that hold for the R2+ sequence by recording a passing
default-disabled H2637 median gate and preserving protected output identity.
R2 through R6J were therefore allowed to proceed. This does not mean the
array-native runtime is complete: R6J closed opt-in direct publication cutover,
while the default runner API and CLI still select compatibility mode unless a
direct flag is supplied.

Any future package that claims default activation, production direct-runtime
completion, or R7 compatibility-runtime removal must carry fresh evidence for:

- default-disabled timing and protected output identity;
- direct-mode timing and RSS;
- rollback selection;
- no hot-loop compatibility authority;
- byte/Arrow/metadata parity for every public output family in scope.

| Stage | Scope | Gate |
|---|---|---|
| **R0 - Runtime schema freeze** | Define typed field schema, ownership, aliases, guards, persistence, publication operands, replay/diagnostic exposure, and direct-frame type boundary. | schema review, contract gate, type-boundary proof, no hot-loop map proof |
| **R1 - Frame constructors and projections** | Build typed run/lane/day/publication frame constructor plans and shadow projections from existing parsed inputs without replacing execution. Production-grade parsed-input constructors remain R7B scope. | roundtrip and output identity; no runtime readiness claim |
| **R2 - Direct executor skeleton** | Introduce a separate direct executor entrypoint selected once at run setup, with no per-phase compatibility branches. It may execute no-op or shadow-only phases initially. | direct mode constructs no compatibility surfaces; call-graph proof that it does not enter `execute_with_kernel*` or `HillslopeKernelRequest` paths |
| **R3 - First complete phase span** | Port a complete phase span with all required upstream/downstream transfer and publication operands. | per-phase and H2637 identity, endpoint/RSS |
| **R4 - Full hydrology direct path** | Port the complete hydrology daily OFE path without requests, payloads, writeback surfaces, symbol lookup, dense refresh, or dirty flush. | material endpoint movement plus identity |
| **R5 - Full OFE-day direct path** | Port all 14 phases to direct-frame execution. | full H2637 identity and endpoint/RSS |
| **R6 - Direct publication cutover** | Make HBP/WAT/PASS/loss/manifest read typed projection only, using the promoted PERFDEEP06 publication operand ledger or an equivalent binding ledger. | byte/Arrow identity, metadata parity, anti-alias fixtures, independent operand reconstruction |
| **R7 - Remove hot compatibility runtime** | Delete or isolate logical/indexed/dense hot-loop plumbing from production direct mode. | <=10x gate, preferably <=5x trajectory |

#### 8.2.2 Current Post-R6J State

R7A reconciles the specification with the implementation evidence through
R6J. This section is descriptive authority for planning; it does not by itself
activate direct mode or relax any R7 gate.

| Stage / package family | Current state | Authority / evidence | Not yet complete |
|---|---|---|---|
| R0/R1 | Planning-only complete. | Direct schema envelope, frame-boundary decision, constructor/projection plan, publication-ledger plan, and no-compatibility proof plan were recorded. | Production typed constructors and default runtime execution were explicitly out of scope. |
| PERFDEEP09 | PERFDEEP07 default-disabled hold lifted for R2+. | Passing default-disabled H2637 median and protected output identity. | Does not implement direct runtime. |
| R2A | Direct runtime namespace and skeleton complete. | Distinct direct frame/executor scaffold, default inactivity proof, explicit opt-in skeleton proof. | No phase math, publication cutover, endpoint improvement claim, or default activation. |
| R3A-R3C | First complete direct spans complete. | Typed inputs, direct compute, state mutation, downstream operands, shadow projection, no-compatibility source/counter proof. | Diagnostic and span coverage only; no full hydrology/publication/default cutover. |
| R4A-R4P/Q/Z | Full direct hydrology span coverage complete for the staged R4 scope. | Direct runoff, storage, subsurface, ET/root uptake, transfer, projection, and closure spans with protected output identity while compatibility remained authoritative. | Normal production execution still runs compatibility scheduler/writeback path unless a direct mode is explicitly selected. |
| R5A-R5E | Full canonical OFE-day direct executor lifecycle and 14-phase coverage complete. | One canonical 14-phase entry per OFE-day, direct R4/R5 spans folded under canonical phase entries, protected output identity and endpoint/RSS evidence. | Public outputs remained compatibility-authoritative at R5E; no default activation. |
| R6A-R6I | Direct publication frame and producer-authority blockers iteratively reduced. | Run-bound publication frame, direct projection consumers, typed day-input builder, PMET layer carry correction, and current-fixture parity closures. | Earlier held states are historical; R6J is the terminal R6 publication cutover package. |
| R6J | Historical opt-in direct publication cutover complete. | `DirectPublicationFrameCutover` proved direct public-output publication could write HBP/WAT/PASS/loss/plot/manifest from direct artifacts with H2637 byte/Arrow identity and direct manifest provenance. | Superseded by R7 direct production and ADR-0030 deletion: cutover/shadow transition modes are not retained production surfaces. |
| R7 | Production direct runtime is activated as the normal hillslope execution path. R7A-H, the ADR-0026 winter-column build, frost observed-data ratification, frost default activation, and the follow-up cutover correction are complete. | R7A-H work packages, `docs/work-packages/20260629-frost-ratification-default-activation-001/`, `docs/work-packages/20260629-frost-direct-cutover-correction-001/`, ADR-0026, ADR-0030, ADR-0031, and the compatibility deletion packages. | Obsolete skeleton/shadow/cutover transition modes are deleted. The public `--compatibility-runtime` selector is removed. Full removal of the compiled scheduler/day-frame/carrier support surface is held as a separate support-boundary deletion. |

Current runtime-mode matrix:

| Mode | Selection status | Execution authority | Publication authority | Manifest/provenance | Remaining blocker |
|---|---|---|---|---|---|
| Compatibility mode | Removed from public API/CLI/harness selection. | No production entrypoint may select compatibility execution. Compiled scheduler/day-frame/carrier support remains only as a held deletion boundary and legacy test surface. | N/A for production. | N/A for production. | Delete or replace the compiled symbol-keyed support boundary, preserving only documented I/O serialization adapters. |
| Shadow / cutover transition modes | Deleted. | N/A | N/A | Source guards reject reintroduction of the obsolete skeleton, shadow, and cutover selectors. | Historical evidence only. |
| Production direct mode | No-env default and explicit `--direct-production-executor`. | Parsed typed run/lane/day frames, `DirectFrameExecutor`, `DirectProductionDayInputBuilder`, and winter-column snow/frost state. | Direct publication artifacts retained from direct executor-owned production state. | Direct runtime counters, direct output provenance, and top-level runtime-selection provenance are recorded when direct reaches execution. | Must remain no-silent-fallback; no public compatibility selector remains. |

The terminal architecture remains stricter than the R6J cutover path. A direct
publication object built from compatibility WB13 rows may be valid R6J adapter
evidence, but it is not sufficient for R7 production direct-mode authority.

#### 8.2.3 Post-R6 Burndown Work-Package Sequence

The following sequence is binding guidance for the remaining array-native
runtime refactor. Each package is expected to be a real execution package with
package-local evidence, review disposition, verification, and commit/push
closure when executed. Do not collapse a later package into an earlier package
unless the earlier package's acceptance gates can still be proved without
weakening review, rollback, fixture, and performance evidence.

**R7A - Architecture State Reconciliation**

Objective: reconcile this specification, ADR-0025 references, and the
work-package log with the actual post-R6J state.

Required work:

- Record PERFDEEP09 as the hold-lift authority for PERFDEEP07.
- Record R2 through R5 as direct-runtime scaffold and phase-coverage evidence.
- Record R6J as opt-in direct publication cutover, not default direct runtime
  completion.
- Define the exact R7 terminal contract: production direct mode must execute
  from typed frames and may not use `HillslopeKernelRequest`,
  `KernelWritebackPayload`, `HillslopeWritebackSurface`, symbol registry,
  indexed surfaces, dense refresh, dirty flush, or WB13-row publication
  authority inside normal execution.
- Add a current-state matrix that separates compatibility mode, shadow mode,
  direct publication cutover, and future production direct mode.

Acceptance gates:

- Architecture spec and work-package catalog agree on R0-R6J status.
- No document claims R6J is default activation or full runtime completion.
- R7 package sequence, stop criteria, fallback policy, and validation gates are
  explicit enough for autonomous execution.
- Scoped Markdown lint passes.

**R7B - Parsed-Input Typed Frame Constructors**

Objective: build production-grade typed `DirectRunFrame`, `DirectLaneFrame`,
and `DirectDayFrame` constructors from parsed run, soil, slope, climate,
management, snow, frost, and PMET sidecar inputs.

Required work:

- Introduce or harden constructor APIs that do not accept
  `HillslopeWritebackSurface`, `BoundarySymbol`, `BoundaryValue`,
  `SymbolRegistry`, indexed surfaces, or WB13 rows as normal direct storage.
- Move sidecar and parsed-input seed logic into typed constructor inputs.
- Preserve legacy aliases only as edge metadata for diagnostics, replay, and
  manifest provenance.
- Add typed finite/domain/unit validation before executor entry.
- Record layout/type-size evidence for direct frame state and major array
  families.

Acceptance gates:

- Constructor roundtrip fixtures cover single-OFE, multi-OFE, snow/frost,
  PMET, breakpoint climate, management, and sidecar absence/default cases.
- Static scans prove direct constructor storage contains no forbidden
  compatibility types.
- Constructor output supplies every input currently read by the direct R4/R5
  phase spans.
- Default compatibility path remains identity-clean and zero-cost-disabled.
- Focused tests, relevant integration tests, Rust gates, and scoped docs lint
  pass.

Execution status: complete in
`docs/work-packages/20260622-r7b-parsed-input-typed-frame-constructors-001/`.
R7B added typed run/lane/day constructor input structs, constructor APIs,
pre-executor validation, R7B constructor fixtures, static no-compatibility
constructor scans, and executable type-size/layout evidence. It did not
activate production direct mode, route the executor from parsed inputs, replace
publication producer authority, or change output schemas.

**R7C - Production Direct Executor Path**

Objective: create the production direct executor path that bypasses
compatibility climate-day execution for direct mode.

Required work:

- Add an explicit production direct runtime selection distinct from skeleton,
  shadow, and publication-only cutover modes.
- Route direct mode from parsed typed frame constructors into
  `DirectFrameExecutor` for the full run/lane/day loop.
- Ensure mode is selected once before execution; the hot loop must not branch
  between compatibility and direct representations per phase.
- Preserve compatibility mode and shadow mode as declared fallback/validation
  modes.
- Add run-local direct runtime counters that prove real phase execution, day
  frame construction, day frame commit, publication production, and zero
  compatibility-edge invocations.

Acceptance gates:

- Direct mode does not call `execute_hillslope_climate_days`,
  `execute_with_kernel*`, or construct `HillslopeKernelRequest`.
- H2637 direct mode executes all canonical R5 phases and records nonzero
  direct phase/counter evidence.
- Focused fixtures prove phase-span identity for the direct executor path.
- Default compatibility mode remains unchanged until the default-activation
  package.
- H2637 default and opt-in direct timing/RSS are recorded.

Execution status: complete in
`docs/work-packages/20260622-r7c-production-direct-executor-path-001/`.
R7C added the explicit `DirectProductionExecutor` runtime selection and
`--direct-production-executor` CLI flag, routed that mode through
`DirectFrameExecutor`, skipped compatibility symbol-registry and indexed-shadow
diagnostic setup for the production direct selection, and recorded run-local
direct runtime counters. Same-binary H2637 evidence recorded default
compatibility at `642.77 s / 228804 KB` and opt-in direct production at
`753.76 s / 625132 KB`; direct production is therefore not performance-ready.
The direct-production H2637 manifest proved `scheduler_kernel_executed=false`,
`publication_source=direct-publication-frame`,
`day_frame_constructions=235961`, `day_frame_commits=235961`,
`direct_phase_entries=8494599`, and
`compatibility_edge_invocations=0`. HBP, PASS, and WAT checksums differ from
default compatibility, so R7C does not close direct publication producer
authority, output parity, default activation, compatibility deletion, or
release readiness.

**R7D - Direct Publication Producer Authority**

Objective: remove WB13-row and runtime-surface authority from direct
publication production.

Required work:

- Emit `DirectRunPublicationFrame` from direct executor state and typed
  publication operands, not from `execution.wb13_rows`.
- Replace runtime-surface scalar reads in direct publication with typed direct
  state, forcing, transfer, erosion, storage, ET, subsurface, calendar,
  identity, and manifest producers.
- Keep compatibility WB13 rows only for compatibility mode and test/shadow
  comparison.
- Expand independent reconstruction evidence for HBP, WAT, PASS, loss, and
  manifest operands.
- Add explicit anti-alias fixtures for precipitation versus liquid input,
  `Q` versus `QOFE` versus `runvol`, lateral versus tile drainage, storage
  aliases, calendar identity, sidecar metadata, and erosion operands.

Acceptance gates:

- Static scans prove production direct publication does not read
  `execution.wb13_rows`, compatibility runtime surfaces, compatibility HBP/WAT/
  PASS/loss builders, or stale logical state as direct authority.
- HBP/WAT/PASS/loss/manifest parity passes for current fixtures and H2637.
- Nonzero peak-runoff/event-duration and erosion-active fixtures are covered
  before claiming erosion publication authority.
- WAT/PASS Arrow schema, values, and metadata are parity-clean.
- Manifest provenance reports direct-source publication rows and run-local
  direct counters.

Execution status: executed-held across eight packages, with the next hold-lift
queued:

- `docs/work-packages/20260622-r7d-direct-publication-producer-authority-001/`
  proved the production direct consumer path emits HBP/WAT/PASS/loss/manifest
  artifacts from the retained `DirectRunPublicationFrame`, not
  `execution.wb13_rows`, and held at
  `HOLD-R7D-MULTIOFE-DIRECT-LANE-SEED-AUTHORITY-ABSENT`.
- `docs/work-packages/20260622-r7d2-multiofe-lane-seed-authority-001/` lifted
  that hold by adding lane-indexed constructor and day-input seed/profile
  authority. The focused one-OFE fixture remains parity-clean, H2637 direct
  production improved to `182.83 s / 627436 KiB`, and direct manifest counters
  still report `compatibility_edge_invocations=0`.
- `docs/work-packages/20260622-r7d3-direct-wb14-r4k-infiltration-producer-001/`
  lifted the direct WB14/R4K producer hold by adding typed hyetograph
  infiltration/depression inputs, direct same-pass infiltration wiring into
  R4A/WB18/ET, direct R4C liquid-aligned storage input, and R4L direct
  saturation addback from R4O hourly carry arrays. H2637 direct production now
  exits 0 at `192.90 s / 643724 KiB`, `H2637.loss.json` is byte-identical to
  default, and direct counters still report `compatibility_edge_invocations=0`.
  Full R7D parity remains executed-held at
  `HOLD-R7D3-DIRECT-MOFE-DYNAMIC-CARRY-TRANSFER-ABSENT`.
- `docs/work-packages/20260622-r7d4-direct-mofe-dynamic-carry-transfer-001/`
  lifted the dynamic MOFE same-day carry blocker by copying current-lane R4O/R4L
  `ui_LfCrf`/`ui_SCrunf` arrays forward into downstream typed transfer buffers
  and making R3A/R4J consume them with area-ratio provenance. Focused H2637
  evidence then had byte-identical WAT and PASS, and loss/plot differed only by
  `run_name`. R7D4 held at
  `HOLD-R7D4-HBP-EROD14-SEDIMENT-PRODUCER-ABSENT` because direct HBP still
  emitted zero sediment concentration, total detachment, and total deposition.
- `docs/work-packages/20260623-r7d5-direct-erod14-sediment-publication-001/`
  proved that the production direct executor has no direct EROD13/EROD14/EROD15
  sediment producer. It replaced silent zero active-sediment publication with a
  fail-closed direct guard when `erod14_wave2_enabled` is active. Focused H2637
  direct production now exits `1` with `R7D5 direct EROD14/EROD15 sediment
  producer must execute before this span`. R7D5 held at
  `HOLD-R7D5-DIRECT-EROD13-EROD14-EROD15-TYPED-PRODUCER-ABSENT`.
- `docs/work-packages/20260623-r7d6-direct-erod13-erod14-typed-producer-001/`
  lifted the missing typed sediment producer hold by adding typed direct
  EROD13/EROD14/EROD15 producer authority, direct WB16 peak-duration authority,
  class-count correction, and direct EROD15 HBP/PASS publication operands.
  H2637 direct production exits `0`, `compatibility_edge_invocations=0`, WAT is
  byte-identical, and PASS sediment fields are parity-clean after removing the
  fabricated MOFE03 default `erod14_lddend = 0.3`. R7D6 held at
  `HOLD-R7D6-PASS-HBP-PEAKRO-COMPATIBILITY-ZERO-RESIDUAL`; R7D7 subsequently
  closed the PASS peak residual and narrowed the current hold to HBP EROD15
  sediment export aliases.
- `docs/work-packages/20260623-r7d7-direct-wb16-peak-publication-parity-001/`
  closed the R7D6 PASS `peakro` residual by making compatibility PASS consume
  runtime `peakro` and direct PASS consume direct runoff peak authority before
  the erosion copy. Fresh H2637 5-day evidence has WAT and PASS byte identity,
  HBP peak/duration parity, and direct `compatibility_edge_invocations = 0`.
  R7D7 originally held at
  `HOLD-R7D7-HBP-EROD15-SEDIMENT-EXPORT-ALIASES-DIRECT-PRODUCER-GAP` because
  compatibility HBP publishes `total_detachment_kg = 0.6` and
  `sediment_concentration_kg_m3 = 6.816136920064195` while direct HBP publishes
  `0.0` for both. R7D8 lifted this hold.
- `docs/work-packages/20260623-r7d8-direct-hbp-erod15-export-alias-parity-001/`
  closed the HBP EROD15 sediment-export alias residual for the current H2637
  5-day direct-production gate. Fresh evidence under `/tmp/r7d8ad-h2637-5day`
  has default/direct exits `0`, HBP/loss/PASS/PLOT/WAT byte identity, parsed
  HBP latest-event parity for peak, duration, total detachment, total
  deposition, sediment concentration, and particle flow fraction, and direct
  manifest `compatibility_edge_invocations = 0`.

Current R7D blocker: none for the current H2637 5-day publication-parity gate.
The remaining R7 work is R7E-R7H: default activation candidate, hot
compatibility isolation/deletion, performance closure, fixture hardening, and
release readiness.

**R7E - Default Activation Candidate**

Objective: make production direct mode the default candidate behind an explicit
activation gate and rollback policy.

Required work:

- Add a default-selection policy that can choose direct mode, compatibility
  mode, or shadow mode before the hot loop.
- Preserve an explicit compatibility fallback through API, CLI, run manifest,
  and operational docs until release cutover.
- Add manifest fields that identify runtime selection, fallback reason, direct
  counter evidence, and output policy.
- Run same-binary default compatibility, direct candidate, and rollback
  comparison on H2637.

Acceptance gates:

- Direct default candidate is byte/Arrow/metadata identity-clean against the
  protected compatibility baseline.
- Direct default candidate is faster than compatibility mode and does not
  regress RSS without recorded disposition.
- Rollback mode writes compatibility-provenanced outputs and remains
  identity-clean.
- CLI/API tests prove default, explicit direct, explicit compatibility, and
  shadow selections.
- No package may switch default behavior if direct mode is slower or if output
  identity is unresolved.

Execution status: executed-held in
`docs/work-packages/20260623-r7e-r7h-direct-runtime-completion-001/`. R7E
selection mechanics are implemented: default-candidate resolves to
compatibility unless explicitly activated, explicit compatibility rollback is
available, and manifests record runtime-selection provenance. This did not
activate direct mode by default.

**R7F - Compatibility Runtime Isolation And Deletion**

Objective: remove or isolate the logical/indexed/dense hot-loop runtime from
production direct mode.

Required work:

- Move compatibility `HillslopeDayFrame`, `HillslopeWritebackSurface`,
  symbol-registry, indexed-surface, dense-refresh, dirty-flush, and
  `KernelWritebackPayload` plumbing behind compatibility/shadow modules.
- Rename compatibility transition types so they cannot be mistaken for direct
  runtime frames.
- Delete unused compatibility hot-loop entrypoints after direct default and
  rollback coverage are proven.
- Keep replay, diagnostics, and shadow comparison adapters edge-only.
- Add compile-time or source-scan guards that fail if direct production code
  imports forbidden compatibility types.

Acceptance gates:

- Production direct-mode call graph excludes compatibility scheduler and kernel
  request/writeback paths.
- Compatibility code remains reachable only through explicit compatibility,
  replay, diagnostic, or shadow modes.
- Static anti-regression scans cover runner, orchestrator, direct runtime, and
  publication modules.
- H2637 direct default remains identity-clean after isolation/deletion.
- Rust gates, cargo-deny, docs lint, and line-count governance pass.

Execution status: complete in
`docs/work-packages/20260623-r7f-direct-day-input-hot-loop-isolation-001/`.
R7F replaced the production direct interleaved day-input builder hot-loop
dependency with typed direct day-input/state projection. Focused R7/R6 tests,
source scans, and manifests now prove production direct reports
`compatibility_edge_invocations = 0` because the hot-loop edge is removed, not
because accounting suppressed it. Static process-control authority still comes
from setup-time seeded surfaces and remains future migration scope.

**R7G - Performance Closure And Fixture Hardening**

Objective: close the array-native runtime against the architecture viability
target and broaden validation beyond the current protected fixture.

Required work:

- Benchmark same-binary H2637 compatibility, direct default, direct explicit,
  and rollback runs with seconds, us/OFE-day, legacy multiplier, and RSS.
- Profile direct mode if it misses the `<=10x` gate; record hot functions,
  allocation sources, string formatting, map/registry calls, and layout costs.
- Remediate measured blockers iteratively until the `<=10x` gate passes or a
  named architecture blocker is proven.
- Add or refresh fixtures for snow/frost active days, breakpoint climate,
  PMET branches, irrigation when enabled, multi-OFE transfer ratios, nonzero
  erosion, sidecar absence/presence, and management transitions.
- Record confidence-tiered legacy comparator delta review and contract-derived
  closure/conservation evidence for touched outputs and process families.

Acceptance gates:

- H2637 direct default reaches `<=10x` legacy or records a named architecture
  hold with profile evidence and a next package.
- Protected public outputs remain byte/Arrow/metadata identity-clean.
- Independent operand reconstruction passes for conservation-sensitive output
  families.
- No compatibility authority appears in direct-mode hot-loop profiles or
  source scans.
- Fixture matrix is documented with pass/fail and residual risk.

Execution status: executed-held in
`docs/work-packages/20260623-r7g-performance-closure-fixture-hardening-001/`,
then continued in
`docs/work-packages/20260623-r7g-iterative-completion-001/`. The marker
was `HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED`, since superseded: the
ADR-0026 winter-column route was executed (see the 2026-06-24 update under the
R7H execution status below).

R7G closed the initial sidecar-only active snow failure by adding typed snow
controls, hourly forcing, state carry, active partition compute, state mutation,
downstream operands, and publication projection. It also reached full-H2637
active frost endpoint execution with `compatibility_edge_invocations=0`.

The remaining failure is architectural, not another scalar input gap. The
request/symbol-surface frost retrofit cannot simultaneously preserve persistent
fine/shadow frost state, avoid unsafe coarse-layer projection, meet protected
HBP/WAT/PASS parity, and keep H2637 within the `<=10x` performance gate.
ADR-0026 ratifies the follow-up route: replace the retrofit with a stateful
winter-column sub-solver, cut direct consumers to typed winter operands, remove
the current direct snow/frost bridges, then rerun the R7G timing, parity,
no-compatibility, fixture, and reconstruction gates.

**R7H - Release Cutover Readiness**

Objective: prepare the direct runtime for release as the normal hillslope
execution path.

Required work:

- Freeze the direct-mode runtime contract and rollback window.
- Update operator-facing CLI/API docs and manifest expectations.
- Add release anti-evasion checks for direct-mode no-compatibility imports,
  runtime counters, output provenance, and fallback behavior.
- Confirm all R7A-R7G package evidence is linked from the work-package catalog.

Acceptance gates:

- Release checklist passes with direct mode as the declared normal path.
- No public compatibility/fallback selector remains; any retained symbol-keyed
  code is documented as a held support-boundary deletion or real I/O adapter.
- Required anti-evasion guards run and pass.
- Workspace Rust gates, `cargo deny check`, protected-output comparison,
  benchmark evidence, and scoped docs lint pass.

Execution status (2026-06-24): the ADR-0026 winter-column sub-solver was built
across the R7G winter sequence (mechanical containment, snow lane migration,
frost state skeleton/comparator seam, typed frost solver extraction, consumer
cutover and retrofit deletion); `winter_column.rs` owns `DirectWinterColumnState`
and the snow/frost retrofit bridges are deleted from production.
`20260624-r7h-closure-activation-gates-001` reran the gates and held at
`HOLD-R7H-H2637-DIRECT-PERFORMANCE-AND-PROTECTED-PARITY` (`113.53 s`, parity
red). `20260624-r7h-iterative-completion-001` then cleared the performance gate:
removing hot-path frost-guard symbol formatting moved the direct
default-candidate endpoint to `61.40 s` (~6.7x legacy, within `<=10x`), with
`compatibility_edge_invocations=0` retained. The remaining frost-freeze
public-output divergence (first material WAT divergence at Julian day 6) was the
sole blocker.

The subsequent frost validation arc ratified the observed-data frost invariants,
bounded the remaining `GAP-SNOWFREEZE-002` residuals, activated direct production
as the no-env default, and corrected the temporary multi-OFE/Wave-2 and legacy
sidecar-discovery compatibility fallback. ADR-0030 therefore amends the R7
terminal contract: compatibility frost bit-parity is no longer the acceptance
target, production direct mode must not silently fall back to compatibility, and
obsolete skeleton/shadow/cutover transition modes are deletion targets. ADR-0031
supersedes the retained replay seam and removes the public
`--compatibility-runtime` selector; rollback is release/git history.

Execution status (2026-06-30): the obsolete skeleton/shadow/cutover **transition
modes are deleted**, and the **direct-publication RSS arc is complete** — the
direct endpoint is run-length-flat (1.13 GiB → 110/51 MiB, byte/value-identical;
the dominant cost was the per-day×OFE setup pre-alloc of §4.11, not the
symbol-map carrier). The typed day-zero seed computation has since cut production
setup to `DirectProductionSeedAuthority`, and ADR-0031 removed the public
compatibility selector. The **remaining single-authority work is support-boundary
deletion**: `scheduler.rs`, `day_frame.rs`, symbol-keyed carriers, scheduler
lifecycle helpers, WB13 scheduler publication, HPHYS trace, audit/shadow support,
and their legacy tests remain compiled as a unit even though no public runtime
selector may invoke compatibility execution.

Closing this now requires a coherent support-boundary deletion:

1. Delete legacy scheduler tests that only validate the removed runtime.
2. Preserve direct-native typed publication/seed tests.
3. Move any genuinely still-needed symbol-keyed serialization helpers out of the
   executable scheduler namespace and document them as I/O adapters.
4. Delete `scheduler.rs`, `day_frame.rs`, and carrier exports that no longer have
   a real adapter role.

Acceptance: no public compatibility selector, no production symbol-map execution
path, direct identity preserved, full gates clean, and only documented
serialization adapters surviving. This is the final step that makes the typed
frame the sole authority
from parse to output.

### 8.3 Package Rules

Every package in this program must state whether it:

- advances direct-frame runtime;
- maintains compatibility validation;
- removes a compatibility boundary;
- only records evidence.

Packages that only make maps, registries, indexed surfaces, dense mirrors, or
payloads cheaper are out of direction unless they are required to preserve a
validation adapter while a direct-frame replacement is being introduced.

### 8.4 Activation Rule

Default activation requires all of:

- direct mode is identity-clean for the activated scope;
- direct mode endpoint improves over compatibility mode;
- direct mode constructs no hot-loop compatibility surfaces;
- output projection is typed for the activated scope;
- rollback path remains available until the next release boundary.

No package may activate a slower direct path by default.

---

## 9. Non-Goals and Preserved Invariants

This architecture changes representation and execution plumbing. It does not
change:

- process physics;
- numerical formulas;
- canonical `SC-*` invariants;
- fail-closed guard posture;
- unit semantics;
- process order;
- MOFE topology;
- output schemas;
- subprocess-per-hillslope orchestration;
- deterministic output requirements;
- legacy provenance obligations.

Out of scope for this document:

- watershed CLI internal redesign;
- wepppy orchestration and GIS concerns;
- legacy input grammar changes;
- output schema redesign;
- default activation of unproven direct mode.

---

## 10. Risks and Mitigations

| Risk | Mitigation |
|---|---|
| Rewrite blast radius hides physics drift | shadow execution, phase fixtures, contract gates, H2637 identity |
| Direct-frame schema misses a publication operand | publication operand ledger before cutover, anti-alias fixtures |
| Layout becomes another dense compatibility layer | typed fields, explicit validity bitsets, no `BoundaryValue` slots in direct mode |
| Runtime mode branches leak into hot loops | executor selected once at run setup; direct-mode no-compatibility-surface proof |
| Guard attribution changes silently | diagnostic subject parity fixtures and contract-first amendment if semantics change |
| Memory grows toward full registry mirrors | frame stores hot working set and publication operands only; type-size/RSS gates |
| Unsafe indexing introduced prematurely | safe iterator/slice/range forms first; unsafe requires proof and profiling |
| Partial direct path repeats PERFDEEP03/05 | complete phase-span gates and stop criteria before expansion |
| Compatibility fallback becomes permanent | packages must identify which boundary they remove or why they are validation-only |

---

## 11. Relationship to ADRs and Existing Authority

- **ADR-0025** remains the ratifying ADR for the array-native
  `HillslopeDayFrame` direction. This revision clarifies that the target is a
  complete direct-frame runtime, not a partial dense island.
- **ADR-0023** remains correct in principle: state must become
  array-authoritative. Its symbol-by-symbol incremental application is
  superseded.
- **ADR-0022** indexed runtime surfaces remain useful for compatibility,
  replay, diagnostics, and shadow checks. They are not part of direct-mode hot
  execution.
- **ADR-0004** subprocess hillslope orchestration is unchanged.
- **ADR-0012**, **ADR-0019**, and **ADR-0020** output/schema authority is
  unchanged.
- **Science contracts** remain the canonical authority for process behavior,
  guards, conservation, and closure. Runtime representation cannot override
  them.

If this document and a package-local artifact conflict, this document controls
architecture direction. Canonical `SC-*` contracts control science behavior.

---

## 12. Open Design Decisions

These are implementation decisions, not permission to keep compatibility in
the hot path. ADR-0026 closes the winter-column stateful sub-solver question for
snow/frost as a narrow accepted exception; it is not general permission to add
stateful sub-solvers elsewhere without a new ADR.

1. **Schema representation**
   Generated field tables, handwritten structs, or a hybrid.

2. **Frame layout**
   Struct-of-arrays versus array-of-structs per family, decided by access
   pattern and endpoint measurement.

3. **Borrowed view granularity**
   One broad `&mut HillslopeDayFrame` versus narrow phase-specific borrow
   splits.

4. **Unit wrappers**
   Which existing wrappers require `#[repr(transparent)]` and where raw `f64`
   remains acceptable.

5. **Validity model**
   Bitsets, typed sentinels, separate optional edge structures, or
   contract-authorized default values.

6. **Shadow mechanism**
   Test-only differential, diagnostic runtime mode, or compile-time feature.

7. **Publication projection storage**
   Per-day row accumulation, streaming projection buffers, or hybrid
   accumulators.

8. **Compatibility fallback lifetime**
   How long logical/indexed execution remains available after direct mode
   passes activation gates.

---

## Appendix A - Reference Map

| Concern | Current location |
|---|---|
| Daily/OFE/phase loop | `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` |
| Phase order | `crates/openwepp-hillslope-orchestrator/src/phase.rs` |
| Kernel request/response/payload | `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs` |
| Symbol registry and indexed surfaces | `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs` |
| Hydrology symbol access | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs` |
| Current frame/transition adapter | `crates/openwepp-hillslope-orchestrator/src/day_frame.rs` |
| Runner setup and indexed authority | `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` |
| Persistent scheduler lifecycle | `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs` |
| HBP/WAT/PASS output helpers | `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` and `crates/openwepp-hillslope-output/src/` |
| Work-package evidence | `docs/work-packages/20260619-perfdeep0*/` |

## Appendix B - Implementation References

These references are guidance inputs only. Local PERF evidence and openWEPP
contracts remain the authority for acceptance.

- Rust `BTreeMap` background:
  <https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#background>.
- Rust `Vec` guarantees:
  <https://doc.rust-lang.org/std/vec/struct.Vec.html>.
- Rust Reference, type layout:
  <https://doc.rust-lang.org/reference/type-layout.html>.
- Rust `Option` representation:
  <https://doc.rust-lang.org/std/option/index.html#representation>.
- Rust Performance Book, heap allocations:
  <https://nnethercote.github.io/perf-book/heap-allocations.html>.
- Rust Performance Book, type sizes:
  <https://nnethercote.github.io/perf-book/type-sizes.html>.
- Rust Performance Book, bounds checks:
  <https://nnethercote.github.io/perf-book/bounds-checks.html>.
- Rust Performance Book, profiling:
  <https://nnethercote.github.io/perf-book/profiling.html>.
- Rust Performance Book, benchmarking:
  <https://nnethercote.github.io/perf-book/benchmarking.html>.
