# Contract and Test Evidence

Evidence class: `Static + Ran`

`SC-SNOWENERGY-001` version 4 binds `INV-SNOWENERGY-026` to two exact source
branches. Total `m_s <= 1 kg m^-2` suspends before partition. For resolved
total mass, lower `m_l < 1 kg m^-2` collapses to a whole-pack thermal volume;
lower equality remains two-volume.

Contract-derived tests cover:

- total `<`, `=`, and `>` threshold behavior;
- strict lower `<`, exact `=`, and adjacent floating-point branch sides;
- preservation of layer mass and cold content;
- zero surface, conductive, vapor, and sublimation exchange while suspended;
- continued whole-pack exchange after lower-volume collapse;
- resumption of the unchanged Stage 3 solver above the boundary; and
- real runner-trace field binding.

Ran: `cargo nextest run --test snow_surface_eb03_contract --test snow_surface_eb03_runtime`

Result: PASS, 23/23. Final focused run ID:
`071389f0-e888-4f20-8bdc-796f0423908e`.

The private native-SWE threshold-side unit vector also passes 1/1 under run ID
`2f3e6d24-d65b-47cf-a0ea-f6c835e92a8c`.
