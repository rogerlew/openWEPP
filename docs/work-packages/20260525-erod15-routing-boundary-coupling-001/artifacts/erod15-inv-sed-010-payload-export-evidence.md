# Erod15 inv sed 010 payload export evidence

Status: complete
Evidence mode: mixed

## Static
- Wave-3 producer payload family exported by hillslope runtime:
  - `total_detachment_kg`
  - `total_deposition_kg`
  - `particle_class_count`
  - `sediment_concentration_kg_m3_{class:04}`
  - `particle_flow_fraction_{class:04}`
- Wave-3 consumer payload family required by watershed routing intake:
  - `hs{ID}_total_detachment_kg`
  - `hs{ID}_total_deposition_kg`
  - `hs{ID}_particle_class_count`
  - `hs{ID}_sediment_concentration_kg_m3_{class:04}`
  - `hs{ID}_particle_flow_fraction_{class:04}`
- Guard continuity maintained via existing families:
  - `HKERNEL-EROD14-WAVE2-E-001..003`
  - `WKERNEL-WS10-CHANNEL-E-001..003`

## Ran
- `cargo test --test erod14_wave2_multiofe_enrichment_kernel_contract` -> PASS (nominal + zero-outflow + failure vectors).
- `cargo test --test ws10_watershed_kernel_contract` -> PASS (includes missing Wave-3 symbol rejection vector).
