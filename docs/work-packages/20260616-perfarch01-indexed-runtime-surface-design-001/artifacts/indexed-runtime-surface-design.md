# PERFARCH01 Indexed Runtime-Surface Design

Status: COMPLETE 2026-06-16
Evidence mode: **Static** (source audit, ARCH16/PERFOPT01/PERFHO02 context) + **Ran** (prototype in this package)

## Design Verdict

Replace the physical runtime-surface representation with a run-scoped indexed
store while preserving the logical `BoundarySymbol` seam. The key invariant is:

```text
SymbolId order == BoundarySymbol string sort order
```

That invariant lets the implementation keep every current sorted-symbol public
effect, including `apply_kernel_writeback` applied-symbol ordering, while making
hot storage operations array indexed.

## Current Surface Audit

Static:

- `BoundarySymbol` is `BoundarySymbol(String)` and derives `Ord`, so every
  `BTreeMap` lookup/insert/clone/drop works through heap strings and string
  comparisons (`crates/openwepp-kernel-contract/src/lib_mod/core_types.rs`).
- `BoundaryValue` is `Copy`; the expensive clone unit is the key and tree node,
  not the value payload.
- `HillslopeWritebackSurface` owns two
  `BTreeMap<BoundarySymbol, BoundaryValue>` maps (`state_surface`,
  `flux_surface`) and is cloned in lane-state and trace paths.
- `HillslopeKernelRequest` borrows the maps, which was the ARCH16 improvement,
  but the borrowed storage is still string-keyed.
- `apply_kernel_writeback` sorts by `field.symbol.as_str()` and then inserts
  cloned `BoundarySymbol`s into both maps.
- Hot symbol families are repeatedly constructed with `format!`: climate
  `timem/intsty`, WB18/WB19 layer keys, frost fine-layer keys, MOFE hourly
  arrays, and PL schedule/growth/decomposition slots.
- Guard paths still scan or probe strings: decomposition overflow prefix checks,
  PL active-slot resolution, frost fine-state guards, consumer-boundary checks,
  and transfer-array validation.

ARCH16 already removed full request-surface clones. PERFARCH01 therefore targets
the remaining string-keyed physical storage, not the borrow boundary ARCH16
already fixed.

## Types

Proposed core shapes:

```rust
#[repr(transparent)]
pub struct SymbolId(u32);

pub struct SymbolRegistry {
    symbols_by_id: Vec<BoundarySymbol>,
    ids_by_symbol: BTreeMap<BoundarySymbol, SymbolId>,
    families: SymbolFamilyRanges,
}

pub struct IndexedSurface {
    values: Vec<Option<BoundaryValue>>,
}

pub struct IndexedWritebackSurface {
    registry: Arc<SymbolRegistry>,
    state: IndexedSurface,
    flux: IndexedSurface,
}
```

`ids_by_symbol` is not the hot path. It is for setup, compatibility seams, error
construction, and debug/export. Hot paths use `SymbolId` or family-specific id
tables after registry freeze.

## Registry Construction

The registry is run-scoped and frozen before scheduler execution:

1. Collect static symbols from typed production enums and current runtime input
   projection.
2. Pre-register dynamic families from known dimensions: climate point count,
   soil layer count, frost fine-layer shape, PL slot/crop counts, irrigation
   event counts, MOFE hourly array width, OFE count, and watershed node ids.
3. Sort symbols by `BoundarySymbol::as_str()`, deduplicate, and assign dense
   zero-based ids.
4. Materialize family ranges and id tables after sorting.
5. Convert current BTreeMap surfaces into indexed surfaces.

No symbol is lazily interned after freeze. Unknown symbols after freeze are
typed failures, not silent map growth. If a package needs extension symbols, it
must register them before freeze.

## Access Model

The API exposes two access modes:

- Compatibility mode: `get_symbol`, `set_symbol`, and sorted iteration by
  `BoundarySymbol`. This keeps current public seams and error surfaces intact.
- Hot mode: `get_id`, `set_id`, and typed family accessors. This is the mode for
  frost, WB18/WB19, PL, MOFE transfer, climate, and writeback apply loops.

Sorted iteration is simply id-order iteration because ids are assigned in string
order. Exporting applied symbols maps ids back through `symbols_by_id`.

## Writeback

Stage implementation should initially keep `KernelWritebackPayload` unchanged:

1. Resolve each `WritebackField.symbol` to `SymbolId`.
2. Sort accepted fields by `SymbolId`.
3. Apply `state[id] = Some(value)` or `flux[id] = Some(value)`.
4. Build `applied_state_symbols` and `applied_flux_symbols` from ids for the
   current public result.

Once kernels can emit id-backed writeback fields, the symbol payload remains
available for diagnostics and compatibility, but the success path need not sort
or clone strings.

## Guard And Prefix Hazards

String prefix scans should be replaced by registry family ranges, not by
unchecked assumptions:

- Decomposition indexed-symbol overflow becomes a range/count invariant over the
  registered PL decomposition family.
- Consumer-boundary validation becomes a set of required ids per phase.
- Frost fine-state domain checks walk pre-resolved layer/fine id arrays.
- MOFE transfer clears walk fixed hourly id arrays.

Failure behavior must remain fail-closed and must still name the logical
`BoundarySymbol` in diagnostics.

## Bit-Identity Constraints

The design does not reorder floating-point reductions, OFE sequencing, phase
order, writeback accept/reject logic, or output publication. It only changes how
scalar boundary values are found and stored. Staged implementation must prove:

- sorted id order equals sorted symbol order,
- BTreeMap export from indexed surfaces equals the previous map,
- applied-symbol order is byte-identical,
- HBP/parquet anchor comparisons have `anchor_mismatches = 0`,
- repeated optimized runs are within-config deterministic.

## Implementation Boundary

PERFARCH01 lands no production change. The proposed ADR and staged plan make the
indexed store the chosen architecture, but every code-bearing stage must still
ship with bit-identity and determinism gates.
