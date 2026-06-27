# Stream Water Temperature — Shared Surface Energy Balance + Temperature-Carrying Flux State

## Status

- `state`: **backlog (concept)** — not promotable before stream water
  temperature becomes a prioritized deliverable. **But two design decisions are
  cheap now and expensive to retrofit, and one of them (the
  `openwepp-meteorology` crate scope) should be made before that crate hardens.**
- `date`: 2026-06-27 (created, Claude Code)
- `relates`:
  [ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md)
  (contract-first new physics),
  `crates/openwepp-meteorology` (the shared surface-energy-balance foundation,
  SNOWDENSITY-10.3.5a),
  `SC-SNOWFREEZE-001` / the FDHP01 frost-soil thermal state (lateral-flow source
  temperature), the snow energy balance (melt source temperature), the `cancov`
  canopy machinery (riparian shading),
  `openwepp-watershed-orchestrator` / `openwepp-topology` (in-stream routing).
- `provenance`: design discussion 2026-06-27 during the snow/frost
  energy-balance arc; no code or contract authored yet.

## Summary

Stream water temperature is **not a separate model** — it is the **terminal
energy-balance consumer** of the thermal machinery openWEPP is already building
for snow and frost. The same surface energy balance (net radiation + turbulent
sensible/latent fluxes) drives snow, soil, and water surfaces; the same `cancov`
canopy attenuation that shades snowmelt shades the stream. The leverage is two
**positioning** decisions made early, not a new physics program.

## The two parts of stream temperature

1. **Source temperatures** — the temperature of each water flux entering the
   channel, mostly already produced by current work:
   - snowmelt ≈ 0 °C (snow energy balance);
   - lateral / interflow = soil-layer temperature (FDHP01 frost-soil thermal
     state);
   - baseflow = groundwater temperature (a damped annual-mean model, to add);
   - surface runoff = surface temperature; rain = wet-bulb (the
     `openwepp-meteorology` psychrometrics already compute this).
2. **In-stream energy balance** — net shortwave (with **riparian shading via
   `cancov`**), net longwave, sensible + latent (evaporation), and streambed
   conduction over the reach travel time. This is the *same* surface energy
   balance as the snow surface — i.e. the `openwepp-meteorology` crate.

## Two design decisions (cheap now, expensive to retrofit)

1. **Scope `openwepp-meteorology` as the shared *surface energy balance*, not a
   rain/snow-partition helper.** It already holds the psychrometric primitives and
   the Harder-Pomeroy hydrometeor energy balance. As the snow/partition work needs
   them anyway, add **net radiation, sensible-heat, and latent-heat/evaporation**
   as pure, **surface-agnostic** functions so a snow surface, a soil surface, and a
   water surface all call the same flux code. Make this scoping choice while the
   crate is young.
2. **Carry water temperature as a typed *intensive* state on each water flux**,
   mixed **mass-weighted** at confluence, threaded hillslope → **HBP shard** →
   channel. Declaring now that `wmelt`, lateral, baseflow, and runoff each carry a
   temperature (additive to the HBP boundary) makes stream temperature a natural
   later extension; retrofitting temperature onto a frozen flux architecture is the
   painful path. This is contract-first: a new `SC-*` for **thermal transport**
   (conservation of thermal energy advected with the water mass).

## Architecture landing (fits the subprocess-per-hillslope model)

- **Hillslope subprocess** emits the **source temperatures** in the HBP shard
  (additive per-flux temperature fields), drawing on the snow energy balance, the
  frost-soil thermal layers, and a simple groundwater-temperature model.
- **`openwepp-watershed-orchestrator`** does the **in-stream** step: mix lateral
  inputs + upstream inflow (mass-weighted), then apply the surface energy balance
  over the reach travel time, reusing `openwepp-meteorology` + the
  `cancov`-based riparian shading. Seasonal canopy (the new deciduous/mixed
  managements) → **seasonal riparian shading → seasonal stream temperature** for
  free.

## Minimal abstraction

```
T_reach_out = energy_balance( mix_massweighted(T_inflow, T_lateral_sources),
                              R_n(shade=cancov) + H + L_vE + G_bed,
                              travel_time )
```
Modeling choice (contract-level): **equilibrium-temperature relaxation**
(SNTEMP-style — fits a daily/reach-routed model with modest data) vs a
**heat-budget advection** along the reach (Heat Source / HFLUX-style — more
faithful sub-daily). Decide per the routing timestep.

## Governing constraints

- **Contract-first (ADR-0011):** author an `SC-*` thermal-transport contract
  (intensive-temperature state, mass-weighted mixing, conservation of advected
  thermal energy, the in-stream energy-balance law, shading via `cancov`) before
  production code.
- **Conservation:** thermal energy must be conserved through mixing and routing;
  temperature is an intensive property of a tracked water mass, never created or
  destroyed without an energy term.
- **Reuse, don't fork:** the in-stream energy balance must call the same
  `openwepp-meteorology` flux functions as snow, not a parallel implementation.
- **Validate against observed data** (the partition-work discipline): USGS NWIS
  continuous stream-temperature gauges are the no-calibration validation corpus.

## Falsifiable validation gates

1. **Source-temperature sanity:** snowmelt-dominated reaches run cold (~0–4 °C
   in melt season); baseflow-dominated reaches track the damped annual mean.
2. **Seasonal signature:** modeled reach temperature reproduces the observed
   seasonal cycle and the diurnal range at NWIS gauges, without site calibration.
3. **Shading sensitivity:** removing/adding riparian canopy (`cancov`) moves
   summer maxima in the observed direction (open reaches warmer).
4. **Conservation:** thermal-energy balance closes through confluences and
   routing.

## Promotion criteria

- Stream water temperature is a prioritized deliverable, **and**
- the `openwepp-meteorology` surface-energy-balance scope + the
  temperature-carrying flux-state decision are ratified (these gate everything
  downstream and should be settled early regardless), **and**
- a thermal-transport `SC-*` contract surface is authored.

When these hold, spin up a dated work-package under `docs/work-packages/`.

## Open questions

- Groundwater/baseflow temperature model: damped-annual-mean air temperature vs a
  deep-soil thermal model vs a calibratable offset (keep calibration-free by
  default).
- HBP boundary: the additive per-flux temperature fields and their units/domain
  guards.
- Equilibrium vs heat-budget-advection in-stream formulation (timestep-dependent).
- Streambed/hyporheic conduction fidelity — first-pass simple bed term vs a
  hyporheic exchange model.
- Reservoir/impoundment thermal behavior (the watershed impoundment element).

## References

- **Theurer, F. D., Voos, K. A., Miller, W. J. (1984).** *Instream Water
  Temperature Model* (SNTEMP). USFWS Instream Flow Information Paper 16 — the
  classic stream-network energy-balance / equilibrium model.
- **Boyd, M., Kasper, B. (2003).** *Analytical methods for dynamic open channel
  heat and mass transfer* (Heat Source), Oregon DEQ — riparian shading + energy
  balance.
- **Glose, A., Lautz, L. K., Hare, D. K.** HFLUX stream-temperature solver — clean
  reference for the surface flux set.
- **Caissie, D. (2006).** *The thermal regime of rivers: a review.* Freshwater
  Biology 51:1389–1406.
- **Webb, B. W., et al. (2008).** *Recent advances in stream and river temperature
  research.* Hydrol. Process. 22:902–918.
- **Hannah, D. M., Garner, G. (2015).** *River water temperature in the United
  Kingdom: changes over the 20th century and possible changes over the 21st.*
  Prog. Phys. Geogr. 39:68–92.
- USGS NWIS continuous water-temperature data — the observed validation corpus.
- Internal: `crates/openwepp-meteorology` (shared surface energy balance),
  the FDHP01 frost-soil thermal state, the `cancov` canopy machinery,
  `openwepp-watershed-orchestrator` (in-stream routing).
