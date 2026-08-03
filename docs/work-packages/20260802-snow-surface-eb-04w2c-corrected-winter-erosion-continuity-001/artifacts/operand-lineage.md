# Operand Lineage

Status: frozen before production edits

| Operand | Units / basis | Producer | Role |
|---|---|---|---|
| `runoff_depth_m` | m water over OFE/day | real fixture pass parquet | event activation and `effdrn` |
| `peakro_m_s` | m s^-1 | real fixture pass parquet | rill discharge and event activation |
| `detinr_kg_s_m2` | kg s^-1 m^-2 | production erosion producers | interrill source |
| normalized `load` delta | nondimensional `G` per 0.01 OFE | Wave-1 RK4/analytic route | accepted numerical solution |
| endpoint `detach + theta` | nondimensional rate | Wave-1 constitutive rate | matched-order diagnostic operand |
| `flux_closure_residual` | nondimensional absolute sum | independent Simpson `1/3`/`3/8` block quadrature minus committed load deltas; trapezoid only for a single-interval region | discretization-consistency diagnostic, not mass law |
| `flux_closure_scale` | nondimensional total `abs(dG)` | accepted grid increments | relative diagnostic denominator |
| exported/inflow/detachment/deposition | kg m^-1 or kg after width projection | committed grid + denormalization | hard telescoping conservation operands |

Rejected closure aliases: refusal percentage is not sediment mass closure;
quadrature-versus-solution agreement is not the exact telescoping identity; accepted
solver output cannot be declared conserving merely because its own residual
field is below a one-sided threshold. Terminal acceptance requires independent
reconstruction of boundary loads and signed per-cell changes.
