# Conservation / Output Lineage (D10B)

Status: executed
Evidence mode: Ran + Static

| Operand | Units | Basis | Source | Authority class |
|---|---|---|---|---|
| `v` rainfall-excess rate | m/s | per cell per step | `Forcing.rainfall_excess_m_s` (caller) | authoritative input |
| `q_up` upstream boundary flux | m^2/s | interval MEAN over each step when the integral closure is present (cascade), else point sample | upstream bin series (exact integral) x width ratio | authoritative input |
| `mass.inflow_m2` | m^2 (unit width) | `q_up dt` = the scheme's ACTUAL injection (prescribed-flux BC in both sweeps) | solver ledger | authoritative |
| `mass.outflow_m2` | m^2 | `0.5 (pred_out_face + q_pred[n-1]) dt` = the scheme's ACTUAL boundary discharge | solver ledger | authoritative |
| `mass.positivity_clamp_m2` | m^2 | half-weight stage clamps + full-weight committed clamps | solver ledger | authoritative diagnostic |
| `scheme_inflow/outflow_m2`, `tvd_boundary_leak_m2` | m^2 | booked-equals-actual identity surfaces | solver ledger | diagnostic (identity check) |
| `outlet_bin_outflow_m2[k]` | m^2 per bin | pro-rata apportioned ACTUAL outflow; sums EXACTLY to `outflow_m2` | solver | authoritative (handoff + volumes) |
| `hydrograph[k]` | m^2/s, m | bin-MEAN boundary flux + bin-mean outlet stage at bin midpoints | solver | shape/metrics surface |
| `per_ofe_outlet_m3` | m^3 | `outflow_m2 x width` (ledger), NOT sample-grid quadrature | cascade | authoritative |
| `per_ofe_received_upstream_m3` | m^3 | downstream ledger inflow = exact upstream bin integral x width ratio x width | cascade | authoritative |
| `CascadeMassBalance.outlet_m3` | m^3 | terminal ledger outflow x width | cascade | authoritative |

Rejected aliases (anti-tautology; see also `seam-conservation-ledger.md`):
sampled-trapezoid volumes as outflow authority (measurement quadrature, not
discharged mass); `total_source - total_outlet` as "loss" (conflates
storage); committed last-cell q as the outlet discharge surface (O(dx)
registration + boundary ripple); `residual + clamp ~ 0` self-consistency
without booked-equals-actual.

Independent reconstruction: the cascade residual decomposition identity
(residual = ofe_internal + seam_sampling + seam_injection +
terminal_quadrature) closes to <= 3.4e-14 m^3 with every term ZERO
post-correction, and the scheme identity (rain + in - out + tvd + clamp -
storage) closes to <= 2e-12 m^3 — two-sided, multi-operand closure, not a
restated producer formula.
