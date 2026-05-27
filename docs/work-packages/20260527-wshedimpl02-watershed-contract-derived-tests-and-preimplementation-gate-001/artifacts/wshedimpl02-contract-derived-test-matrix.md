# WSHEDIMPL02 Contract-Derived Test Matrix

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
WSHED03 vectors mapped to normalized unresolved rows:

| Vector | Gap coverage | File + test |
|---|---|---|
| KW/MC wave-state publication vector | `GAP-ROUTE-008`, `GAP-SYSTEM-005` | `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs` -> `wshed03_contract_kw_mc_vector_requires_wave_routing_state_family_publication` (`#[ignore]`) |
| Channel sediment publication vector | `GAP-ROUTE-009`, `GAP-SED-006`, `GAP-SYSTEM-008` | `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs` -> `wshed03_contract_channel_sediment_vector_requires_channel_sediment_publication_family` (`#[ignore]`) |
| Parser-projected impoundment coefficients vector | `GAP-IMPOUND-006`, `GAP-SYSTEM-007` | `tests/integration/ws12_impoundment_physics_equivalence_contract.rs` -> `wshed03_contract_ws12_vector_requires_parser_projected_coefficients_without_manual_seed` (`#[ignore]`) |
| RK4/regime-transition timestep-stability vector | `GAP-IMPOUND-005`, `GAP-SYSTEM-005` | `tests/integration/ws12_impoundment_physics_equivalence_contract.rs` -> `wshed03_contract_ws12_vector_requires_regime_transition_timestep_stability` (`#[ignore]`) |
| Watershed CLI non-stub parquet emission vector | `GAP-SYSTEM-005`, `GAP-SYSTEM-006` | `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` -> `wshed03_watershed_cli_end_to_end_vector_requires_non_stub_parquet_emission` (`#[ignore]`) |

## Ran
- `cargo test -p openwepp --test ws11_channel_routing_physics_equivalence_contract`
- `cargo test -p openwepp --test ws12_impoundment_physics_equivalence_contract`
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract`
