# PL10 Implementation and Test Evidence

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Implemented active slot/crop resolver and wired growth/decomposition
  dispatch preconditions to dynamic symbol families.
- Added/updated scheduler-kernel integration-style tests for multi-slot,
  rotation-boundary, and typed failure behavior.

Ran:
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (with existing unmatched-allowance warnings)

## Code Changes

1. Added active slot selection type and typed resolver error model:
   - `HillslopePlActiveSlotResolutionError` (`HS-PLDISP-E-001..009`)
2. Added slot/crop symbol construction helpers for schedule/growth/decomp.
3. Added typed finite/integral/range guards for dispatch symbols.
4. Added deterministic slot/crop resolution functions:
   - `resolve_active_pl_slot_selection`
   - `select_active_crop_slot_for_day`
5. Updated growth/decomposition boundary errors to carry
   `ActiveSlotResolution` and inherit resolver code/boundary class.
6. Replaced fixed-slot management symbol normalization calls with dynamic
   symbol references.

## Test Evidence

### Active slot routing behavior

- `active_slot_resolution_uses_year_three_perennial_slot`
- `active_slot_resolution_wraps_rotation_boundary_to_year_one`

### Typed failure behavior

- `active_slot_resolution_rejects_ambiguous_slot_candidates`
- `active_slot_resolution_rejects_missing_active_crop_for_day`
- `active_slot_resolution_rejects_ambiguous_active_crops_for_day`

## Code Anchors

- `crates/openwepp-hillslope-orchestrator/src/lib.rs:194`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:860`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:1025`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:1133`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:3075`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:3131`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:3187`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:3234`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs:3313`
