# PL10 Active Slot Authority Contract

Status: `complete`
Evidence mode: `Static + Ran`
Disposition: `implemented`

Static:
- PL growth/decomposition dispatch authority no longer binds to fixed
  `slot_0001/crop_0001` symbols in production dispatch logic.
- Active slot/crop selection is resolved per runtime day/year, schedule
  rotation, and OFE target using typed state-symbol guards.

Ran:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Contract

1. Dispatch entry for PL phases requires schedule sentinel presence:
   - `pl_schedule_slot_count`.
2. Active slot selection is computed from runtime symbols:
   - `pl_schedule_slot_count`
   - `pl_schedule_rotation_years`
   - `pl_schedule_rotation_repeats`
   - `year`
   - `day`
3. Active slot candidate must match:
   - `ofe_index == 1`
   - `rotation_index == ((year - 1) / rotation_years) + 1`
   - `year_in_rotation == ((year - 1) % rotation_years) + 1`
4. Active crop selection for the chosen slot is day-aware:
   - annual/fallow window via `jdplt..jdharv`
   - perennial window via `jdplt..jdstop` (or `jdharv` fallback when
     `jdstop == 0`)
5. Missing, non-finite, non-integral, out-of-range, missing-active, and
   ambiguous-active conditions are hard failures via typed statuses
   (`HS-PLDISP-E-001..009`), not silent fallbacks.

## Code Anchors

- `crates/openwepp-hillslope-orchestrator/src/lib.rs:194`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:860`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:1025`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:1133`
