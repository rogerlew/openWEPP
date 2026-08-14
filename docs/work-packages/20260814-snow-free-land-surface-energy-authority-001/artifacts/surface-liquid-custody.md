# Surface Liquid Custody

Evidence class: `Static`; canonical authority and independent vectors are
terminally confirmed by dual review.

`SurfaceWaterKey` is `(transaction, OFE, tile, surface_class, store, basis)`.
Stores are `PondedSurface`, `LitterHeld`, and `SoilLayer(layer_id)`. Hydrology
owns every mass and publishes one immutable beginning snapshot; LSE owns no
duplicate mass amount.

## Request, Authorization, Final Use, And Credit

Potential evaporation publishes one nonnegative request for each exact source.
One real hydrology arbitration considers all vegetation-root and ground
requests against the same immutable source snapshot. The identity chain is

```text
0 <= finalized withdrawal F <= maximum authorization A <= request D.
```

Condensation is not a request or a negative use. With outward vapor rate
`E_out<0`, its positive stand-ground credit is
`C_cond=-f_tile*E_out*dt`. Hydrology validates capacity and constructs that
credit after the final solve. The ledger retains `F` and `C_cond` separately;
they cannot cancel before both typed records exist.

## Conservation-Safe Fixed Authorization

Authorization is made strictly from immutable beginning source stores and all
competitors on that same snapshot. Current precipitation, runon, throughfall,
drainage, and stemflow do not enlarge same-interval ET availability. A
potential canopy candidate never mutates or becomes the authorization
inventory. The final fixed-cap solve is rebuilt from original beginning
thermal/vegetation state and recomputes final canopy release and hydrology
ingress.

Each finalized withdrawal is constrained by its beginning-store-backed fixed
authorization:

```text
F_source <= A_source
F_source <= A_source <= D_source
F_source <= M_begin_source
M_after_ET = M_begin_source - F_source >= 0
M_end_source = M_after_ET + I_current_source + C_cond
               - infiltration_source - runoff_source - overflow_source >= 0.
```

No authorization is backed by a contingent release, so a smaller final release
cannot invalidate supply. All current ingress is applied after finalized ET
debits; it remains in final hydrology storage or proceeds through final
infiltration/runoff and is not reauthorized for ET in the same interval. No
second authorization or separate final-inventory cap occurs.

## Source Priority And Capacity

Source priority is structural, not scalar donation: ponded evaporation uses
`PondedSurface`; litter evaporation uses `LitterHeld`; bare mineral-soil
evaporation uses its explicit configured `SoilLayer`. Litter blocks direct
mineral-soil evaporation in V1. Root requests never borrow a surface-store
authorization, and unused root authorization never becomes ground credit.

Hydrology applies litter/pond capacity to the final candidate. Condensation
first creates a typed credit to the configured surface store; any excess is a
typed final overflow crossing routed by hydrology to infiltration/runoff. It is
not clipped, donated, or silently discarded. Every branch preserves the exact
mass/thermal lineage described in `advected-energy-convention.md`.
