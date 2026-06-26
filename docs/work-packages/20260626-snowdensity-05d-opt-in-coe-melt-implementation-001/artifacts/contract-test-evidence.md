# Contract Test Evidence

Evidence class: Ran.

## Focused 05D Tests

Command:

```sh
cargo test --test snowdensity05d_opt_in_coe_melt -- --nocapture
```

Result: passed.

Coverage:

- v79 contract markers for `INV-SNOWFREEZE-055`,
  `OBL-SNOWFREEZE-P-030`, `snow_melt_shortwave_absorbed_fraction`, and the
  ratified `amelt` formula.
- Legacy and opt-in shortwave `amelt` reconstruction.
- Missing active opt-in albedo state fails closed.
- Direct runtime storage and shadow projection carry routed melt, lineage, and
  albedo state.

## SNOWDENSITY Regression Set

Command:

```sh
cargo test --test snowdensity02_contract_adr_guard --test snowdensity03_physics_bulk_offline_contract --test snowdensity05a_melt_contract_guard --test snowdensity05b_shortwave_source_contract --test snowdensity05c_albedo_state_core --test snowdensity05d_opt_in_coe_melt
```

Result: passed.

The earlier contract-version guard tests were updated from v78 to v79 where they
assert the current `SC-SNOWFREEZE-001` version.
