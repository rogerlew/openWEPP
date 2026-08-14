# Advected Energy Convention And Receiver Custody

Evidence class: `Static`; exact terminal authority confirmed by dual review.

The liquid reference state is liquid water at `T_ref=273.15 K` with zero
sensible enthalpy. For every source-resolved liquid parcel,

```text
h_l(T) = C_w * (T - T_ref)       J kg^-1
Q_adv  = m * h_l(T)              J m^-2
C_w    = 4218                    J kg^-1 K^-1.
```

Positive-mass mixing conserves enthalpy exactly:

```text
T_mix = T_ref + sum(m_i*h_i)/(C_w*sum(m_i)).
```

Zero mass has no temperature and no energy record. Nonzero energy with zero
mass, positive mass without temperature, and a mass/energy source mismatch are
typed failures.

## Source Temperature And Receiver Table

| Crossing | Required temperature | Surface sign | Exact receiver/custody |
|---|---|---:|---|
| liquid precipitation | `hydrometeor_temperature_c+273.15` from `openwepp_meteorology::phase::hydrometeor_temperature_from_relative_humidity` | `+m*h(T)` | receiving tile surface-liquid thermal node paired to hydrology ingress |
| routed runon | upstream accepted routed-liquid temperature/enthalpy | `+m*h(T)` | downstream OFE/tile thermal node paired to the same hydrology route |
| canopy throughfall | accepted source occupancy `wet_surface_temperature_k` | `+m*h(T)` | same tile pond/litter receiving node |
| canopy first/second drainage | accepted source occupancy `wet_surface_temperature_k` | `+m*h(T)` | next occupancy when present, otherwise same tile ground receiver |
| stemflow | accepted source occupancy `wet_surface_temperature_k` | `+m*h(T)` | same tile ground receiver, bypassing lower occupancy |
| infiltration | final mixed source-store temperature immediately before partition | `-m*h(T_mix)` | identified target soil-layer thermal owner receives `+m*h(T_mix)` |
| routed runoff | final mixed source-store temperature immediately before partition | `-m*h(T_mix)` | downstream routing envelope receives the same mass/enthalpy pair as runon |
| terminal runoff | final mixed source-store temperature | `-m*h(T_mix)` | explicit hillslope-boundary export; no downstream state receipt |

Canopy liquid is isothermal with its accepted occupancy wet-surface node;
throughfall, both drainage releases, and stemflow therefore carry that exact
`wet_surface_temperature_k`. They are not air, soil, or freezing-temperature
aliases.

## Ordering

Beginning surface liquid supplies same-interval ET; precipitation, runon, and
canopy releases do not. Those current ingress parcels retain separate source
records during potential evaluation. After the one beginning-store
authorization, the final canopy-ground solve rebuilds from beginning state and
produces final release temperatures. Hydrology first applies finalized ET
debits to beginning stores, then mixes current ingress and partitions final
storage/infiltration/runoff. Infiltration and runoff carry the final mixed
surface-node temperature at the instant they leave. No potential-pass
temperature or advected-energy record is committed.

Current ingress is a post-solve enthalpy operator: it changes ending
surface-liquid/thermal state and routed receipts only. It does not feed back
into the accepted interval's surface temperature, sensible heat, latent heat,
ground heat, or ET request/final-use calculation.

## Multi-OFE Routing And Area Conversion

Tile fractions are local to one OFE. Tile-ground mass/energy first becomes
OFE-ground amount by multiplying by that tile's local fraction once. Routing
then conserves extensive mass and energy:

```text
mass_kg = m_upstream_kg_m2_ofe * A_upstream
heat_J  = Q_upstream_J_m2_ofe  * A_upstream
m_downstream_kg_m2_ofe = mass_kg / A_downstream
Q_downstream_J_m2_ofe  = heat_J  / A_downstream.
```

The downstream temperature is reconstructed from `heat_J/mass_kg`, not copied
after an area-basis change. OFE area is `fwidth*slplen` at the routing boundary.
No upstream tile fraction is reused downstream, and no stand-wide fraction is
applied across OFEs.

Every mass record and energy record shares transaction, source, upstream and
downstream OFE/tile identity, interval, area basis, and route sequence exactly
once.
