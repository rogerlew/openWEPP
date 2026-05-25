# Erod15 verification agent a

Status: complete
Evidence mode: ran

## Static
- Verification scope: EROD15-local and directly affected integration suites.

## Ran
- `cargo test --test erod14_wave2_multiofe_enrichment_kernel_contract --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract --test arch22_typed_state_surface_contract --test erod11_alias_boundary_ownership_contract --test cli01_runner_hillslope_integration --test cli03_runner_contract_derived_tests --test erod15_wave3_contract_authority_closure_contract` -> PASS.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract` -> PASS.
