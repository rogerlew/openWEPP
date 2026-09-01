# V39 soil-energy transaction separation pre-implementation red

Status: `EXPECTED_RED_CAPTURED`

Authority green:

- `cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract v39_contract_binds_distinct_soil_energy_source_and_target_transactions`
- run `b8899cf5-e0db-494c-988e-dfa4ba9ed8a1`: 1 passed.

Expected source red:

- `cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract v39_soil_energy_transaction_separation_production_seams_are_required`
- run `1de603a9-d185-45e0-8453-8733d486004e`: 1 failed as expected.
- The failure reported absent `PhysicalSoilEnergyTransactionAuthorityV2`,
  absent source/soil transaction fields and candidate validation, all three
  required behavior vectors, and the retained one-transaction ingress/soil
  comparison in `physical_soil_energy_operands_v2`.

Retained causal evidence is canonical r102,
`/tmp/wghl_001d_v38_64m_r102.log`, SHA-256
`962716679a499b4e3bd23f87ed2e6ceabda1435081cc590fd9c06f21ed52548b`:
the valid composed second child has outer surface-ingress/source transaction
42 and authenticated soil-target transaction 43, then refuses `V2 soil
support identity` before acceptance. Production implementation begins only
after this authority-green/source-red checkpoint.
