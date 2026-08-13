# V3 Area and Unit Ledger

Status: `approved authority complete`

Evidence mode: `Static`

| Operand | Units | Area/basis | Conversion/normalization | Authority |
| --- | --- | --- | --- | --- |
| `L`, `S`, `P` | `m2 plant m-2 tile-ground` | conditional occupancy | `LAI_s/C_s`, `WAI_s/C_s`, `P=L+S` | V2 + V3 |
| `rho/tau` | fraction | band-specific plant optical pair | area weights `L/P`, `S/P` | CLM5 2.3.11--12 |
| `K_eff` | `m2 ground m-2 plant` | direct extinction per actual plant area | `Omega*K`, once | V3 canonical selection |
| absorbed leaf/stem terms | `W m-2 tile-ground` | occupancy/component/direction/band | area-times-absorptivity partition | V3 canonical selection |
| `u_ref`, `u_star`, surface winds | `m s-1` | reference/canopy surface | neutral momentum logarithm, no floor | CLM5 2.5.117 |
| characteristic dimensions | `m` | leaf/wet/stem surface | no conversion | caller configuration |
| `z2` | `m` | root-to-stem path | equals `height_m` | CLM5 plant hydraulics |
| stem gravity | `mm H2O` | root-to-stem head | named `m -> mm H2O`, factor 1000 | CLM5 plant hydraulics |
| node potentials | `mm H2O` | one root/stem/sun/shade node per occupancy | no MPa alias | V3 state schema |
| `q1/q2/q3` | `kg H2O m-2 tile-ground s-1` equivalent contract rate basis | occupancy local | integrate by `dt`; multiply by `f_t` once for owner request | V3/V2 transaction |
| Atkin intercept/result | `umol CO2 m-2 leaf s-1` | leaf area | source equation is already the `Rd25` rate | immutable CTSM source |
| Atkin leaf N / T10 | `g N m-2 leaf`, `degC` | leaf area | `kg N -> g N` and `K -> degC` once | immutable CTSM source |
| `Rd25`, `Rd(T)` | `umol CO2 m-2 leaf s-1` | leaf class | no carbon/day conversion; only admitted temperature response | V3 + immutable CTSM source |
| leaf Rd carbon debit | `kg C m-2 stand-ground interval-1` | class area, tile fraction, interval | molar C, class area, `f_t`, `dt`, once | carbon ledger |

All identity joins are exact. Numeric tolerances cannot repair a wrong area,
band, direction, owner, layer, transaction, unit, or amount basis.
