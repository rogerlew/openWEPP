# Array-Native Runtime Architecture Specification

Status: **Ratified, Revision 2** - binding design authority for the
performance re-architecture, ratified by
[ADR-0025](../decisions/0025-array-native-hillslope-day-frame.md) on
2026-06-18 and revised after PERFDEEP07 on 2026-06-19.
Audience: all contributors working on runtime, scheduler, kernel, publication,
or performance packages.
Owner: architecture authority; implementation by Codex work packages.
Supersedes: the incremental application of
[ADR-0023](../decisions/0023-array-authoritative-hot-path-state.md), not its
dense-authority principle.
Last updated: 2026-06-19.

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
contracts, process order, and output schemas remain authoritative and must be
validated bit-for-bit or by the existing Arrow/byte identity gates.

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
| PERFDEEP07 retained patch | `685.85 s` | `2907` | `75.20x` | `229004 KB` |
| <=10x budget | `91.2 s` | `386` | `10x` | - |
| <=5x budget | `45.6 s` | `193` | `5x` | - |

H2637 has `235,961` OFE-days. The measured gap is representation-wide:
pointer-heavy maps, dynamic symbol resolution, enum dispatch, allocation,
payload construction, and compatibility refresh/flush dominate over physics.

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

### 4.12 Runtime Modes

The architecture has three modes:

1. **Direct mode**
   The production target. Runs only typed frames and typed projections.

2. **Compatibility mode**
   Maintains the current logical/indexed runtime for replay, comparison, and
   emergency fallback while direct mode is being validated. It is not the
   performance target.

3. **Shadow mode**
   Runs direct and compatibility paths together for selected fixtures or
   packages, then compares field, phase, and output identity. Shadow mode is a
   validation harness, not a shipping hot path.

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

### 7.4 Completion Gates

A direct-frame stage is not complete until:

- focused tests pass;
- H2637 identity passes;
- H2637 endpoint/RSS is recorded;
- `cargo fmt --check` passes;
- `cargo clippy --workspace --all-targets -- -D warnings` passes;
- `cargo test --workspace` passes;
- `cargo deny check` passes;
- scoped docs lint passes;
- review and verification findings are dispositioned.

HOLD is required when known invariant, identity, or performance gates remain
unresolved.

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
| PERFDEEP07 | default-disabled compatibility cleanup, still HOLD |

### 8.2 New Rewrite Sequence

Future work should follow this sequence.

| Stage | Scope | Gate |
|---|---|---|
| **R0 - Runtime schema freeze** | Define typed field schema, ownership, aliases, guards, persistence, publication operands, and replay/diagnostic exposure. | schema review, contract gate, no hot-loop map proof |
| **R1 - Frame constructors and projections** | Build typed run/lane/day/publication frames from existing parsed inputs and project them back to current outputs without replacing execution. | roundtrip and output identity |
| **R2 - Direct executor skeleton** | Introduce direct executor selected once at run setup, with no per-phase compatibility branches. It may execute no-op or shadow-only phases initially. | direct mode constructs no compatibility surfaces |
| **R3 - First complete phase span** | Port a complete phase span with all required upstream/downstream transfer and publication operands. | per-phase and H2637 identity, endpoint/RSS |
| **R4 - Full hydrology direct path** | Port the complete hydrology daily OFE path without requests, payloads, writeback surfaces, symbol lookup, dense refresh, or dirty flush. | material endpoint movement plus identity |
| **R5 - Full OFE-day direct path** | Port all 14 phases to direct-frame execution. | full H2637 identity and endpoint/RSS |
| **R6 - Direct publication cutover** | Make HBP/WAT/PASS/loss/manifest read typed projection only. | byte/Arrow identity and metadata parity |
| **R7 - Remove hot compatibility runtime** | Delete or isolate logical/indexed/dense hot-loop plumbing from production direct mode. | <=10x gate, preferably <=5x trajectory |

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
the hot path.

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
