# Area and Unit Ledger

Status: `selected`

Evidence mode: `Static`

| Operand | Local units/basis | Stand-owner conversion |
|---|---|---|
| `S_liq,s,t`, rain, wet evaporation, releases | `kg H2O m^-2 tile-ground` | `sum f_t X_s,t` |
| local energy | `J m^-2 tile-ground interval^-1` | `sum f_t Q_s,t` |
| local photosynthesis/transpiration | declared leaf or tile-ground basis | integrate local area, then `f_t` once |
| water demand/final use | local `kg H2O m^-2 tile-ground` | multiply by `f_t` once |
| water authorization | stand-ground amount | divide by `f_t` once for local cap |
| shared C/N pools | `kg C|N m^-2 stand-ground` | update once from weighted flux sum |

`C_s=sum f_t` over occupied tiles. Tile fractions are positive and sum to one;
they are never silently normalized. Local and aggregate stores are not both
mutable.
