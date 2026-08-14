# Model Stack Selection

The selected stack is named `OPENWEPP_SNOW_FREE_LSE_V1` and requires successor
vegetation identity `OPENWEPP_C3_WOODY_V8`.

- V7 shortwave, interception, FvCB--Medlyn, hydraulics, C/N, phenology and
  ownership remain unchanged.
- FSM2/ORCHIDEE supply arbitrary-rank reciprocal longwave.
- ISBA-MEB supplies the shared canopy-air/ground turbulent network and
  fully coupled energy-budget topology.
- CLM5 supplies mineral-soil vapor and thermal-column equations.
- ISBA-MEB Part 2 supplies the explicit forest-litter heat/water equations.
- Harder--Pomeroy supplies precipitation temperature.
- First-law liquid enthalpy transport supplies runon/infiltration/runoff joins.

Rejected alternatives are agricultural PMET partitioning, one-way V7 forcing
replacement, bulk single-temperature canopy longwave, stale ground
temperature, diagnostic proportional water, a stateless ground ledger, and a
bare-soil-only endpoint.
