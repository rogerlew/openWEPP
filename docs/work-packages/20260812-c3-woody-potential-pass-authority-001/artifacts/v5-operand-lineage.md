# V5 Capped-Pass Operand Lineage

Status: `frozen before authority edits`

Evidence mode: `Static`

| Operand | Units | Basis | Authoritative source | Role |
| --- | --- | --- | --- | --- |
| `A_W_i` | `kg H2O m^-2 stand-ground` | stand-ground interval amount | hydrology maximum authorization with exact typed key | immutable owner cap |
| `f_t` | `1` | stand area fraction occupied by tile | validated V4 topology configuration | stand/tile conversion |
| `dt` | `s` | immutable transaction interval | validated forcing/configuration identity | amount/rate conversion |
| `A_tile_i` | `kg H2O m^-2 tile-ground` | tile-ground interval amount | `A_W_i/f_t` | intermediate cap amount |
| `cap_rate_i` | `kg H2O m^-2 tile-ground s^-1` | tile-ground rate | `A_W_i/(f_t*dt)` | fixed complementarity operand |
| `q_law_i` | `kg H2O m^-2 tile-ground s^-1` | tile-ground rate | independently evaluated E14 soil-root hydraulic law | constitutive operand |
| `q_i` | `kg H2O m^-2 tile-ground s^-1` | tile-ground rate | `min(q_law_i,cap_rate_i)` | accepted local layer flux |
| active flag | `bool` | exact layer identity | `cap_rate_i<=q_law_i` | derivative and diagnostics branch |
| generalized derivative | residual per coupled unknown | exact layer branch | zero on cap/equality branch; law derivative otherwise | semismooth Newton Jacobian |
| root residual | `kg H2O m^-2 tile-ground s^-1` | occupancy tile-ground rate | `q2-sum_i(q_i)` | coupled continuity equation |
| residual scale | same as root residual | occupancy tile-ground rate | V3 terms plus all `q_law`, cap-rate, and selected-flux magnitudes | normalization only |
| `F_W_i` | `kg H2O m^-2 stand-ground` | stand-ground interval amount | `f_t*q_i*dt` | finalized use, debit, and receipt |

## Anti-Tautology Requirements

The independent validator reconstructs every conversion and minimum from
published operands. It must not consume a producer-supplied zero residual or
active flag without checking it. Fixture values must discriminate amount/rate,
stand/tile, omitted/double weighting, authorization/finalized use, branch tie,
and wrong-layer aliases.
