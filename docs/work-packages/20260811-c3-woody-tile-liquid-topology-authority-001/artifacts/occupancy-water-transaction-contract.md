# Occupancy Water Transaction Contract

Status: `selected`

Evidence mode: `Static`

Water identities include transaction, stratum owner, occupancy, soil layer,
resource type, unit, and stand-ground amount basis. For positive `f_t`:

```text
D_W,s,t,l = f_t * D_tile,s,t,l
A_tile,s,t,l = A_W,s,t,l / f_t
F_W,s,t,l = f_t * F_tile,s,t,l
0 <= F_W,s,t,l <= A_W,s,t,l <= D_W,s,t,l
```

Hydrology arbitrates all stand-ground requests against one same-layer snapshot.
Duplicate identities fail before summation. Occupancy, layer, transaction, and
amount-basis swaps fail. Final stratum transpiration is the sum of finalized
occupancy/layer uses; unused authorization is not debited.
