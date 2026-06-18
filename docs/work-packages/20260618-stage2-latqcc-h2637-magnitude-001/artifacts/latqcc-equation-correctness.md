# H2637 `latqcc` Equation Correctness

Package:
`20260618-stage2-latqcc-h2637-magnitude-001`

## Evidence

Static:

- `SC-SUBHYD-001` `INV-SUBHYD-003` and `INV-SUBHYD-012` require lateral `q`
  to follow the WB19 lateral-flow equation from layer-aware conductivity,
  geometry, and drainable depth.
- The H2637 traced lane has `solwpv=9002`, `lane_substeps=24`, anisotropy
  `1.0`, and no `solwpv < 2006` legacy saturation multiplier.

Ran:

- The expanded trace row count is 114 (`6` selected simulation days x `19`
  OFEs).
- Residual summary was computed from:
  `/tmp/stage2_latqcc/diag3/h2637_trace_selected_days.jsonl` and
  `/tmp/stage2_latqcc/diag3/owepp_output/H2637.wat.parquet`.

## Recomputed Equation

For each OFE-day and each of the 24 substeps, the diagnostic recomputation used:

```text
Ke = (86400 / lane_substeps) *
     conductivity_depth_sum / saturated_depth_sum

q_potential =
  fcdep_before * anisotropy * Ke * sin(atan(avgslp)) / slplen

q_target = min(q_potential, available_pool, lateral_capacity_tdv)
```

Then:

```text
latqcc_mm = sum(q_substep_m) * 1000
q_m       = sum(q_substep_m)
q_m       = sum(layer_withdrawal_m)
Qd        = q + Qdd
```

## Residuals

Maximum absolute residuals over all 114 traced OFE-days:

| Check | Max absolute residual |
| --- | ---: |
| WAT `latqcc_mm - q_m * 1000` | `0.0 mm` |
| `q_m - sum(q_substeps)` | `4.163336342344337e-17 m` |
| `potential_m - sum(potential_substeps)` | `4.163336342344337e-17 m` |
| `target_m - sum(target_substeps)` | `4.163336342344337e-17 m` |
| `capacity_tdv_m - sum(capacity_substeps)` | `7.105427357601002e-15 m` |
| `q_m - sum(layer_withdrawals)` | `1.3877787807814457e-17 m` |
| `potential_m - recomputed Eq [6.2.4] potential` | `4.163336342344337e-17 m` |
| `target_m - recomputed min(potential, pool, capacity)` | `4.163336342344337e-17 m` |
| `q_m - target_m` | `0.0 m` |
| `Qd - (q + Qdd)` | `0.0 m` |

## Peak-Row Check

The global traced maximum is OFE 7 on simulation day 5507:

| Field | Value |
| --- | ---: |
| WAT `latqcc` | `71.62409876710504 mm` |
| trace `q` | `0.07162409876710504 m` |
| trace potential | `0.07162409876710504 m` |
| trace target | `0.07162409876710504 m` |
| `sum(q_substeps)` | `0.07162409876710506 m` |
| recomputed Eq potential | `0.07162409876710506 m` |
| WB19 capacity total | `12.584565702061628 m` |
| minimum available pool | `0.49008592113612026 m` |
| `watyld` range | `0.1706334131690198..0.339395182626328` |

The small differences are floating-point summation residuals.

## Equation Verdict

`CORRECT` for the traced H2637 rows. Emitted `latqcc` equals WB19 lateral `q`,
and WB19 lateral `q` equals the Eq [6.2.4] / Dun 3a-3c potential and target at
machine precision. No `INV-SUBHYD-003` or `INV-SUBHYD-012` kernel equation
violation was found.
