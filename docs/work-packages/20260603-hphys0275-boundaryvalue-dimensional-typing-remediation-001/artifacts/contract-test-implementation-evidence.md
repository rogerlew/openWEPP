# Contract Test Implementation Evidence

Status: completed
Evidence mode: static + ran

Static: HPHYS0275 added and extended contract-derived tests:

- `tests/integration/hphys0275_boundary_value_dimensional_typing_contract.rs`
  verifies constructor fail-closed behavior, daily no-breakpoint typed labels,
  breakpoint `stmstr`/series typed labels, all 24 SIMIMPL28 hourly typed labels,
  and exact selected numeric lineage preservation.
- `tests/integration/sim_contract_boundary_unit_registry.rs` verifies migrated
  aliases are `TypedRequired` and non-migrated aliases remain
  `FollowUpRequired`.
- `crates/openwepp-unit-boundary/src/lib.rs` unit tests cover added wrappers,
  above-maximum errors, signed temperature, density, and daily/hourly radiation
  wrappers.

Ran:

- `cargo test -p openwepp-unit-boundary` -> pass.
- `cargo test --test hphys0275_boundary_value_dimensional_typing_contract`
  -> pass.
- `cargo test --test sim_contract_boundary_unit_registry` -> pass.
