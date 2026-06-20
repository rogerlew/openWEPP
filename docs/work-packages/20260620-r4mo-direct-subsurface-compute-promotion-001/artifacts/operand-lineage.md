# Operand Lineage

Status: pre-implementation.

| Operand | Units | Basis | Authority | Status |
|---|---|---|---|---|
| `theta[]` | m water | per soil layer | `SC-PERC-001`, `SC-SUBHYD-001` | authoritative direct state |
| `fc[]` | m water | per soil layer | `SC-PERC-001` | authoritative direct input |
| `ul[]` | m water | per soil layer | `SC-PERC-001`, `SC-SUBHYD-001` | authoritative direct input |
| `ssc[]` | m/s | per soil layer | `SC-PERC-001`, `SC-SUBHYD-001` | authoritative direct input |
| `dg[]` | m | per soil layer | `SC-PERC-001`, `SC-SUBHYD-001` | authoritative direct input |
| `thetdr[]` | m/m | per soil layer | `SC-PERC-001`, `SC-SUBHYD-001` | authoritative direct input |
| `thetfc[]` | m/m | per soil layer | `SC-SUBHYD-001` | authoritative direct input |
| `por[]` | m/m | per soil layer | `SC-SUBHYD-001` | authoritative direct input |
| `coca[]` | fraction | per soil layer | `SC-SUBHYD-001` | authoritative direct input |
| `D` | m | OFE-day | `SC-PERC-001` | authoritative direct downstream operand for R4B |
| `Pe` | m | OFE-day | `SC-PERC-001` | authoritative direct downstream operand for WB19 |
| `Pe_i[]` | m | per soil layer | `SC-PERC-001` | diagnostic/direct projection |
| `q` | m | OFE-day | `SC-SUBHYD-001` | authoritative direct lateral operand |
| `Qdd` | m | OFE-day | `SC-SUBHYD-001` | authoritative direct drainage operand |
| `Qd` | m | OFE-day | `SC-SUBHYD-001` | authoritative direct downstream operand for R4B |
| `lateral_withdrawal[]` | m | per soil layer | `SC-SUBHYD-001` | diagnostic/direct projection |
| hourly carry arrays | m/hour | MOFE hourly lane | `SC-SUBHYD-001` | diagnostic/direct projection |

Static: `D`/`Pe` must not alias public `Dp`; `q`/`Qdd`/`Qd` must not alias
public `latqcc`, storage residual, root-zone ET, or handoff sentinels.
