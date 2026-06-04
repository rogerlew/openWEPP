# Unit Registry Audit

Status: completed
Evidence mode: static + ran

Static: `crates/openwepp-sim-contract/src/units.rs` now includes HPHYS0280 watershed climate aliases, selected snow runtime/trace aliases, `DirectionDegrees` domain class, and `TypedRequired` posture for migrated symbols. `tests/integration/sim_contract_boundary_unit_registry.rs` now asserts the new aliases resolve and the migrated symbols are no longer follow-up.

Ran:
- `cargo test --test sim_contract_boundary_unit_registry -- --nocapture`: pass.
- `tools/release/check_unit_registry.sh`: pass.
