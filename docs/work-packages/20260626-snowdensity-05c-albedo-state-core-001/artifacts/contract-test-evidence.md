# Contract Test Evidence

Ran:

- Initial focused test failed before implementation because the public
  albedo-state API did not exist.
- Focused post-implementation tests prove contract markers, albedo bounds,
  positive-temperature age decay, fresh-snow reset, missing-state fail-closed
  behavior, and no state requirement for `legacy_coe`.

Test target:

- `cargo test --test snowdensity05c_albedo_state_core`

Companion guards:

- `cargo test --test snowdensity05a_melt_contract_guard`
- `cargo test --test snowdensity05b_shortwave_source_contract`
