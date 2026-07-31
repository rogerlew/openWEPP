# Contract-Test Evidence

Status: `complete`

Evidence mode: `Ran`

Ran: `cargo nextest run --test snow_surface_eb03_contract --test
snow_surface_eb03_runtime --test
paradigm2_stage0_surface_energy_balance_contract` passed `13/13`.

The tests cover:

- required authority/version/binding markers;
- displaced-sky longwave and typed polar-night failure;
- B/L/S/LS selector orthogonality and default-off behavior;
- independent hourly longwave and vapor/latent reconstruction;
- cold-content export, layer exhaustion/promotion, aggregate SWE/depth/density,
  and no liquid alias;
- missing shared provider and legacy/new sublimation double-debit rejection;
- the exact Stage 3 runtime-source allowlist authorized by EB-03.
