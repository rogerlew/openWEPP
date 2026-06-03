# Contract Test Implementation Evidence

Status: completed
Evidence mode: static-and-run

Static: Added `tests/integration/sim_contract_boundary_unit_registry.rs` and
registered it in the root `Cargo.toml`.

Ran: `cargo test --test sim_contract_boundary_unit_registry` passed before final
artifact disposition.

## Test Coverage

- Confirms WAT publication columns resolve to expected units and dimensions.
- Confirms runtime `prcp` meters are distinct from WAT `P` millimeters.
- Confirms climate, winter hourly, snow runtime, snow hourly, and soil template
  aliases resolve through the registry.
- Confirms the required-alias gate rejects missing dimensional symbols.
- Confirms the HPHYS0274 required-alias manifest resolves all touched-scope
  runtime and publication aliases.
- Confirms WAT schema unit metadata resolves through the registry.
- Confirms dimensional rows cannot use `dimensionless` as a unit label.
- Confirms scalar exceptions require an explicit reason.
- Confirms duplicate and ambiguous aliases are rejected.
- Confirms invalid template tokens, ambiguous concrete template matches, and
  duplicate publication aliases are rejected.

## Gate Wrapper

`tools/release/check_unit_registry.sh` runs the focused registry test and
focused clippy gate and is documented as the mandatory local package gate for
unit-registry-affecting work.
