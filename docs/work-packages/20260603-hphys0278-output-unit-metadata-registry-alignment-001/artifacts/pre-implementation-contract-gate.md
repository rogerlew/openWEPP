# Pre Implementation Contract Gate

Status: completed
Evidence mode: ran

Static: production output writer code was not modified before the contract
amendment and red test were added.

Ran:

```text
cargo test --test sim_contract_boundary_unit_registry \
  hphys0278_output_unit_registry_covers_output_schema_unit_metadata -- --nocapture
```

Result: failed as expected before implementation.

Failure class:

- unresolved `OutputUnitRegistry`;
- unresolved `watershed_interchange_schemas`;
- `hillslope_wat_schema` was still non-fallible.
