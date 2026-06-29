# Stream Water Temperature — Shared Surface Energy Balance + Temperature-Carrying Flux State

## Status

- `state`: **active program candidate** — the two prerequisites are now met: the
  `openwepp-meteorology` surface energy balance is built (Paradigm 2 **Stage 0**),
  and the snow-side meltwater-temperature **source** is delivered as a typed flux
  (Paradigm 2 **Stage 3-decouple**, the snow-neutral water-temperature arm). Stream
  water temperature is the next watershed-side program. **Lead entry investigation
  (operator, 2026-06-29): determine whether hourly water + temperature can be
  serialized across the HBP shard and consumed by the `openwepp-cli-watershed` CLI
  for in-stream routing** — the concrete form of design decision 2; it gates both
  this program and the Paradigm-2 multilayer promotion (what that promotion
  serializes). See the lead-investigation section below.
- `date`: 2026-06-27 (created); 2026-06-29 (elevated to active program candidate +
  serialization determination, Claude Code)
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

1. **(DONE — Paradigm 2 Stage 0.)** **Scope `openwepp-meteorology` as the shared
   *surface energy balance*, not a rain/snow-partition helper.** The surface-agnostic
   net-radiation / turbulent sensible / latent / conduction primitives now exist in
   the crate (`surface_energy.rs`), alongside the psychrometric primitives and the
   Harder-Pomeroy hydrometeor energy balance — so a snow, soil, or water surface all
   call the same flux code. This decision is settled.
2. **Carry water temperature as a typed *intensive* state on each water flux**,
   mixed **mass-weighted** at confluence, threaded hillslope → **HBP shard** →
   channel. Declaring now that `wmelt`, lateral, baseflow, and runoff each carry a
   temperature (additive to the HBP boundary) makes stream temperature a natural
   later extension; retrofitting temperature onto a frozen flux architecture is the
   painful path. This is contract-first: a new `SC-*` for **thermal transport**
   (conservation of thermal energy advected with the water mass).

## Lead investigation — hourly water + temperature serialization to the watershed CLI

The first concrete step (and the gate for the Paradigm-2 multilayer promotion that
follows): determine the **feasibility and resolution** of carrying water +
temperature across the subprocess boundary to the watershed CLI.

- **What the watershed CLI consumes.** `openwepp-cli-watershed` routes over completed
  per-hillslope HBP shards. In-stream temperature needs each hillslope's exported
  water **fluxes** plus their **temperatures** at the routing timestep.
- **Resolution question (the crux): hourly vs daily.** The HBP boundary today is
  **daily** (per-day rows). Diurnal stream temperature wants **hourly** water +
  temperature — but that is ~24× the per-flux data crossing the HBP, and the hourly
  intensity is **CLIGEN-stochastic** (strategy §10.2 item 6 / paradigm2 spec §1.1),
  so diurnal stream temperature would be **forcing-limited** while daily/seasonal
  aggregates stay forcing-robust. Decide hourly-vs-daily against that
  fidelity-vs-cost-vs-forcing tradeoff.
- **Serialization feasibility.** Can the HBP shard schema (ADR-0019 output surface)
  carry per-flux hourly water + a typed temperature, and can the watershed routing
  timestep actually consume it — or does the watershed route daily, making daily
  serialization sufficient?
- **The typed-flux-temperature boundary (design decision 2, now concrete).** The
  meltwater-temperature source already exists as a typed flux (Stage 3-decouple);
  this settles whether/how it — plus the lateral / baseflow / surface-runoff / rain
  source temperatures — serialize to the HBP and are **mass-weighted-mixed** at
  confluence in the watershed.
- **Output:** a feasibility determination (hourly serializable + watershed-consumable
  — yes/no, at what resolution) + the HBP boundary decision. This **feeds the
  multilayer promotion** (whether it serializes hourly or daily water + temperature)
  and the eventual `SC-*` thermal-transport contract.

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
