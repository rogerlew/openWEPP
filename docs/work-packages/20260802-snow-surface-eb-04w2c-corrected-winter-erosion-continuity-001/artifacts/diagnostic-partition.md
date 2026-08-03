# Corrected-Winter EROD16 Diagnostic Partition

Evidence mode: **Ran + independently reconstructed from retained logs**.

The intake hard `5e-3` trapezoid-versus-RK4 diagnostic was reapplied to
unbounded diagnostic solves to characterize the old failure; no acceptance
threshold was changed in the production result.

| Transition | Count |
|---|---:|
| `absent_to_clean` | 10 |
| `absent_to_refused` | 5 |
| `clean_to_absent` | 9 |
| `clean_to_clean` | 150 |
| `clean_to_refused` | 31 |
| `refused_to_absent` | 2 |
| `refused_to_clean` | 10 |
| `refused_to_refused` | 25 |

Prior population: `227` storms, `37` refusals.
Corrected population: `231` storms, `61` refusals.

| Refusal cohort | Median runoff (m) | Median peak runoff (m s^-1) | Median diagnostic ratio |
|---|---:|---:|---:|
| prior | 0.0150182 | 8.80804e-07 | 0.00756574 |
| corrected | 0.0176999 | 6.66308e-07 | 0.00857458 |

The exact day-level operands and classifications are retained in
`storm-partition.csv`.

The transitions show that the reported increase was not simply 24 new storm
days: only five newly qualifying days refused, while 31 formerly clean days
became refusals and ten formerly refused days became clean. The diagnostic
therefore responds to the changed forcing-dependent curvature of the accepted
solution. Every inspected accepted solve still satisfied exact telescoping
mass closure. This partition rejected a snow-physics rollback and supported
correction of the lower-order erosion diagnostic instead.
