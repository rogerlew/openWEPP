# Source Provenance Ledger

Evidence mode: Static.

## Canonical OpenWEPP Acceptance Point

Canonical openWEPP acceptance point: daily climate `rad`/`radly` in `Ly d^-1`.

This is the engine-owned seam for shortwave radiation. It is already present in
WEPP climate input and runtime daily forcing. Upstream gridded products may feed
this field, but product selection, acquisition, spatialization, and upstream
provenance are orchestration concerns and must not be implemented inside the
openWEPP kernel/runtime.

## Transformation Ledger

| Stage | Symbol/surface | Units | Authority |
|---|---|---|---|
| Climate parser/runtime daily forcing | daily climate `rad`/`radly` | `Ly d^-1` | `SC-CLIMATE-001#INV-CLIMATE-013`; climate file spec |
| Daily metric conversion | `radmj = radly * 0.04184` | `MJ m^-2 d^-1` | `SC-CLIMATE-001#INV-CLIMATE-013` |
| Slope/aspect daily transform | `sunmap` `estrad`/`rpoth` | `MJ m^-2 d^-1` | `SC-CLIMATE-001#INV-CLIMATE-013`; SIMIMPL28 runtime |
| Hourly distribution | `radcur`/`hr_tmp` | `MJ m^-2 h^-1` | `SC-CLIMATE-001#INV-CLIMATE-013` |
| Publication | `winter.hourly.rad_mj_m2_####` | `MJ m^-2 h^-1` | Runtime climate projection |

Near-isothermal hourly branch: `radmj/24`, still after the single conversion
from `radly`.

## ET Shared Authority

ET shared authority: ET and snowmelt consume the same daily radiation source
family, represented as climate `rad`/`radly` and ET `RA`/`radiation_ly`.
`SC-EVAP-001#INV-EVAP-021` governs ET radiation provenance, while
`SC-CLIMATE-001#INV-CLIMATE-013` governs the winter hourly radiation unit seam.

`coe_shortwave_albedo_v1` must not introduce a snow-only radiation scalar. A
future melt implementation must consume the hourly surface produced from the
same daily climate radiation authority.

## Upstream Gridded Product Boundary

OpenWEPP does not own gridded shortwave product selection. If DAYMET or another
source is used upstream, orchestration must normalize it into the daily climate
`rad` field in `Ly d^-1` and carry provenance outside the engine or as a typed
metadata ledger. This package does not choose a provider and does not add a
provider selector.
