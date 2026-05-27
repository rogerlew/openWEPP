# Worker Handoff

Status: complete
Evidence mode: static
Date: 2026-05-27

## Static
- WSHED03 is complete for vector + pre-implementation gate scope.
- New ignored expected-failure vectors are the authoritative baseline for
  downstream runtime migration closure:
  - `wshed03_contract_kw_mc_vector_requires_wave_routing_state_family_publication`
  - `wshed03_contract_channel_sediment_vector_requires_channel_sediment_publication_family`
  - `wshed03_contract_ws12_vector_requires_parser_projected_coefficients_without_manual_seed`
  - `wshed03_contract_ws12_vector_requires_regime_transition_timestep_stability`
  - `wshed03_watershed_cli_end_to_end_vector_requires_non_stub_parquet_emission`

### Immediate next actions
- Execute `WSHED04`: remove manual WS12 coefficient seeding dependency by
  closing parser-to-runtime projection seams.
- Execute `WSHED05`: migrate WS11 wave-routing lineage (`q1/qin/qlat/c0..c4`).
- Execute `WSHED06`: migrate channel sediment routing process families.
- Execute `WSHED07`: migrate WS12 RK4/adaptive regime-transition continuity.
- Execute `WSHED08`: activate watershed parquet writers and remove
  `OWSOUT-E-004` in valid lanes.

### Watch-items
- Keep ignored vectors failing until the owning runtime package lands closure;
  then promote each vector out of ignored expected-failure posture.
- Preserve fail-closed typed guard behavior in all intermediate migrations.
- Keep `SC-ROUTE-001`, `SC-IMPOUND-001`, `SC-SED-001`, and `SC-SYSTEM-001`
  gap rows synchronized as vectors flip from expected-failure to pass.

## Ran
- none
