# PERFDEEP01 Frame Schema

Evidence: Static + Ran.

## Implemented Frame Type

Stage-0 scaffold introduces `HillslopeDayFrame` in `openwepp-hillslope-orchestrator` as a shadow-only
representation of per-day OFE runtime surfaces.

Source: `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`.

### Core Dense Storage

- `state_slots: Vec<Option<BoundaryValue>>`
- `flux_slots: Vec<Option<BoundaryValue>>`
- Slot index is the frozen `SymbolRegistry` id (`SymbolId::as_usize()`), so every registered runtime symbol
  has one dense slot home.
- `BoundaryValue` is preserved in-slot (unit-tagged scalar variants retained; no lossy conversion).

### Array Family Scaffold (Stage-0)

Implemented fixed-width MOFE hourly carry arrays (24 slots each), captured from state surface symbols:

- `mofe_hourly_upstream_saturation_runoff` (`ui_SUrunf_0001..0024`)
- `mofe_hourly_current_saturation_runoff` (`ui_SCrunf_0001..0024`)
- `mofe_hourly_upstream_lateral_runoff` (`ui_LfUrf_0001..0024`)
- `mofe_hourly_current_lateral_runoff` (`ui_LfCrf_0001..0024`)

Each element is `Option<BoundaryValue>` so absent symbols remain explicit `None` without synthetic defaults.

### I/O-Edge Typed Capture Scaffold

`HillslopeDayFrameIoEdgeScalars` captures Stage-0 publication/HBP edge fields:

- `peakro`
- `watdur`
- `total_detachment_kg`
- `total_deposition_kg`
- `sediment_concentration_kg_m3_0001`
- `runtime_day`
- `runtime_year`

### Borrowed Climate Forcing

Frame stores climate forcing as borrowed slice:

- `climate_forcing_series: Option<&[f64]>`

No cloning/allocation of forcing data in Stage-0.

## Seed/Flush Contract

### Seed

`HillslopeDayFrame::seed_from_surfaces` and `seed_from_writeback_surface`:

1. Resolve every symbol in logical surfaces through the frozen registry.
2. Write values into dense slot vectors by id.
3. Populate MOFE array-family fields from dense state slots.
4. Capture I/O-edge scalar fields.

### Flush

`flush_to_surfaces` and `flush_to_writeback_surface`:

1. Iterate registry ids in canonical order.
2. Rebuild logical state/flux maps from `Some` slots only.
3. Preserve exact `BoundaryValue` variants and `f64` bits.

## Shadow Harness Interface

Stage-0 differential harness is test-harness based (no production authority flip):

- `shadow_roundtrip_report`
- `assert_shadow_roundtrip_bits`

Report fields:

- state/flux symbol counts
- state/flux mismatch counts
- first mismatch payload (symbol, expected/observed bits, expected/observed unit labels)

This is the Stage-0 shadow mechanism selected from spec fork 5 (test-harness differential).
