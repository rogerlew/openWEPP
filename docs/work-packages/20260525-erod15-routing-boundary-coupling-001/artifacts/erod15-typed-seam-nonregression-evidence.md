# Erod15 typed seam nonregression evidence

Status: complete
Evidence mode: mixed

## Static
- Typed watershed contributor symbols added without removing existing typed projections:
  - `HillslopeContributorTotalDetachmentKg`
  - `HillslopeContributorTotalDepositionKg`
  - `HillslopeContributorParticleClassCount`
  - `HillslopeContributorSedimentConcentrationKgM3`
  - `HillslopeContributorParticleFlowFraction`
- Alias registry extended to include direct + contributor-scoped Wave-3 payload families.

## Ran
- Typed seam validation suites passed:
  - `cargo test --test arch22_typed_state_surface_contract` -> PASS.
  - `cargo test --test erod11_alias_boundary_ownership_contract` -> PASS.
  - `cargo test --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract` -> PASS.
