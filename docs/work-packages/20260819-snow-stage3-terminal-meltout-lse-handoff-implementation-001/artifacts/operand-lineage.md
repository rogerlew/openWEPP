# Operand lineage and anti-alias plan

Status: frozen before production edits

Evidence mode: Static

All mass depths at the snow boundary are `kg m^-2` horizontal ground area;
receiver tile quantities retain their contract-defined tile/stand basis and
must pass explicit area conversion rather than numerical aliasing.

| Operand | Units/basis | Source and authority | Status |
| --- | --- | --- | --- |
| start/end ice, deposition, sublimation, refreeze, melt | `kg m^-2` ground | terminal Stage 3 event, `SC-SNOWENERGY-001` | evaluation-authoritative inside coupled candidate |
| start/terminal retained liquid, external rain liquid | `kg m^-2` ground | terminal event state/ledger | evaluation-authoritative; rain remains separately tagged |
| terminal liquid parcel | `kg m^-2` ground plus explicit receiver conversion | exact event liquid ending store, debited once | new handoff; never store delta, CoE melt, or runoff |
| parcel temperature/enthalpy | `K`, `J kg^-1` | terminal isothermal liquid at `273.15 K`; precipitation advected heat already closes snow energy | new typed lineage; no terminal energy alias |
| cold-content change, component energy, fusion/refreeze, terminal unallocated energy | `J m^-2` ground | terminal event | closes snow only; unallocated energy is not a receiver credit |
| `t*`, evaluated and remaining support | `s` inside day/hour/half-hour identity | terminal bracket/result plus scheduler calendar | exact, nonoverlapping support |
| receiver SW/LW/sensible/latent/precipitation/soil heat | `W m^-2` and integrated `J m^-2` on contract basis | freshly evaluated LSE-V2 terminal-receiver branch | no snow albedo, temperature, roughness, or flux reuse |
| vegetation/litter/mineral/frost/ponded selector state | typed owner state and fingerprints | immutable beginning V10/LSE/surface/soil/frost owner set plus event-time liquid | receiver authority |
| infiltration, ponding, overflow, runon, runoff | `kg m^-2` or contract depth/area basis | real surface-liquid/WB14/routing owners | exact-once receiver outputs; snow drainage is not runoff |
| transaction/restart/rollback lineage | typed IDs, hashes, phase and consumed bit | new outer coupled owner | mechanics authority only |

Required independent identities:

- solid: `ice_start + refreeze + deposition - sublimation - melt = ice_end`;
- liquid: `liquid_start + external_rain + melt - refreeze = terminal_liquid`;
- time: snow support plus receiver support equals requested support with no
  overlap or gap;
- handoff: snow terminal-liquid debit equals receiver parcel credit after
  explicit area conversion;
- receiver water: incoming terminal/rain/runon equals infiltration plus ending
  storage, evaporation, overflow/routed carry, and runoff under owning signs;
- snow energy and receiver energy close separately. No equation may credit
  terminal unallocated snow energy to the receiver.

Anti-alias fixtures must use unequal start store, store delta, external rain,
melt, refreeze, terminal liquid, CoE melt, runon, ponding, overflow, and runoff;
unequal snow and receiver albedo/temperature/roughness; nonzero terminal
unallocated energy; and non-midpoint `t*`. The actual consumer evidence must be
reconstructed without producer residual fields and checked with two-sided
closure/magnitude tolerances.
