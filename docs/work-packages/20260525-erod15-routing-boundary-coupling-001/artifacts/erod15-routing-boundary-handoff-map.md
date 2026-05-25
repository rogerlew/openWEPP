# Erod15 routing boundary handoff map

Status: complete
Evidence mode: static

## Static
- Hillslope producer -> watershed contributor alias mapping:
  - `total_detachment_kg` -> `hs{ID}_total_detachment_kg`
  - `total_deposition_kg` -> `hs{ID}_total_deposition_kg`
  - `particle_class_count` -> `hs{ID}_particle_class_count`
  - `sediment_concentration_kg_m3_{class:04}` -> `hs{ID}_sediment_concentration_kg_m3_{class:04}`
  - `particle_flow_fraction_{class:04}` -> `hs{ID}_particle_flow_fraction_{class:04}`
- Producer runtime location:
  - `crates/openwepp-hillslope-orchestrator/src/lib.rs` (`run_erod14_wave2` Wave-3 export writes)
- Consumer runtime location:
  - `crates/openwepp-watershed-orchestrator/src/lib.rs` (`read_hillslope_sediment_payload` intake validation)
- Typed symbol authority location:
  - `crates/openwepp-kernel-contract/src/lib.rs` (`WatershedProductionStateSymbol::*HillslopeContributor*`)

## Ran
- Not run (mapping evidence is structural/code-level).
