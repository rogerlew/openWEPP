# Height And Geometry Reconciliation

Status: `complete diagnostic / no correction authority`.

Evidence mode: `Ran`.

GRIDMET `vs`: nominal `10 m` above its land/model surface. CLI/Stage 3:
unadjusted raw daily value. PMET: locally adjusted to `2 m`. Stage 3:
`z_T=z_q=z_u=5 m` virtual transfer geometry above instantaneous modeled snow,
with `z0=0.005 m`. These datums are not aliases. No retained snow-surface
height translation, source pixel, stability-aware correction, or exposure
authority exists.

Independent neutral diagnostic uses the runtime relation in
`openwepp-meteorology/src/surface_energy.rs:24,1246-1247`:
`PAESCHKE=7.35` and `d=2*PAESCHKE*z0/3`. With contract-bound
`z0=0.005 m`, `d=0.0245 m`:

`L10=ln((10-d)/z0)=7.598449453381016`

`L5=ln((5-d)/z0)=6.902843234621116`

`L10/L5=1.1007709714847784`: `10.0771%` friction-velocity factor.
Squared product ratio `=1.211696731663543`: `21.1697%` turbulent-product
factor. Conversely, a neutral 10-to-5 transfer factor is `0.9084542` for wind
and `0.8252890` for the squared product.

This is a direction/scale sanity bound only. `d` is reconstructed from the
existing runtime formula, not fitted authority; stability, canopy,
ground-to-snow datum, and
exposure are unresolved. No production correction follows.
