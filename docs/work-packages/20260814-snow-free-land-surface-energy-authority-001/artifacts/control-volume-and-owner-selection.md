# Control Volume And Owner Selection

Evidence class: `Static`; exact terminal authority confirmed by dual review.

## Mutable Owner Table

| Quantity/state | Sole mutable owner | LSE access | Candidate/receipt rule |
|---|---|---|---|
| canopy liquid, canopy thermal/numerical state | vegetation | final component operands | vegetation candidate only |
| ponded/depression water mass | hydrology | immutable keyed mass view; isothermal with tile surface node | hydrology alone debits evaporation or credits dew |
| litter-held liquid mass | hydrology | immutable keyed mass view; isothermal with litter surface node | hydrology enforces capacity, overflow, debit, and credit |
| soil-layer liquid/frozen mass | hydrology | immutable keyed layer view | hydrology alone debits finalized soil evaporation/root uptake |
| ground/litter surface enthalpy | LSE | mutable candidate; accepted temperature is derived | sole physical LSE thermal state; retained temperature is numerical warm start/diagnostic only |
| soil-layer thermal state | soil/frost thermal owner | immutable beginning temperatures | receives the exact opposite of surface conductive transfer plus accepted infiltration enthalpy |
| vegetation C/N/material pools | vegetation and BGC under V8 transaction | no LSE mutation | unchanged V8 owner-set rules |
| routed liquid mass | hydrology routing envelope | paired immutable energy record | upstream outflow and downstream runon are one routed identity |
| routed liquid enthalpy | LSE/soil-thermal routing envelope | mutable candidate paired to hydrology mass | same source, transaction, interval, and route as mass |

There is no LSE `M_l` mass state, persistent per-store liquid temperature, or
second mutable physical surface-temperature state. Surface liquid is
isothermal with the one tile surface node; its hydrology-owned mass contributes
`C_w*W` to the sole surface enthalpy. Accepted temperature is derived from
enthalpy/mass/capacity; any retained temperature is only a numerical warm start
or diagnostic. Thermal state does not authorize, reserve, debit, or credit
water.

## Exact Control Volumes

Each OFE-local tile has one ground/litter surface thermal control volume. A
bare tile couples that surface to the configured soil thermal column. A litter
tile has a litter surface node over the same soil column. A covered tile also
has one shared zero-storage V8 canopy-air heat/vapor node. Tile control volumes
close before OFE weighting.

The soil/frost thermal candidate receives exactly the energy leaving the
surface through conduction. Infiltration enthalpy enters the identified target
soil layer. Routed runoff enthalpy either becomes the matching downstream runon
receipt or leaves the supported hillslope boundary. No ground flux, liquid
enthalpy, or water mass is stored by two owners.

## Signed Owner Aliases

All surface-ledger fluxes are positive **into** the surface. Constitutive
outward fluxes use distinct names:

| Physical crossing | Constitutive sign | Surface-ledger alias | Paired receiver |
|---|---|---|---|
| sensible heat `H_out` | positive surface to air | `H_surface=-H_out` | atmosphere/canopy-air `+H_out` |
| vapor `E_out` | positive evaporation; negative condensation | `LE_surface=-L_v(T_s)*E_out` | atmosphere `+L_v*E_out`; hydrology mass `-E_out*dt` |
| conduction `G_down` | positive surface to soil | `G_surface=-G_down` | soil thermal `+G_down` |
| liquid advection into surface | positive incoming mass enthalpy | `Q_adv_surface=+m*h(T)` | source has matching negative record |
| infiltration/runoff out | nonnegative outgoing mass enthalpy | `Q_adv_surface=-m*h(T_mix)` | soil/downstream routing has matching positive record |

Thus condensation (`E_out<0`) warms the surface (`LE_surface>0`) and produces a
positive hydrology credit `C_cond=-f_tile*E_out*dt`; it is never represented as
a negative withdrawal or netted against an evaporation request.
