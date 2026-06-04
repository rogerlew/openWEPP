# Unit Registry Audit

Status: completed
Evidence mode: static + ran

Static: Registry changes align typed posture with actual migrated producers:

- Migrated hillslope daily climate aliases are `TypedRequired`.
- Migrated SIMIMPL28 hourly aliases are `TypedRequired`.
- `wind` direction is split from `vwind` and remains `FollowUpRequired`.
- Watershed-prefixed storm timing/intensity aliases are split into follow-up
  rows instead of overclaiming typed coverage.
- `snow.hourly.rain_m_{idx4}` is split from
  `snow.hourly.rain_retained_m_{idx4}` so the migrated input is typed and the
  retained trace remains follow-up.

Ran:

- `cargo test --test sim_contract_boundary_unit_registry` -> pass.
- `tools/release/check_unit_registry.sh` -> pass.
