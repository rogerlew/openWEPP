# Conservation And Consumer Evidence

Evidence class: **Static** for lineage; **Ran** for named tests.

No output or conservation formula changed. This table records the unchanged
real handoff path and keeps the dormant point fallback out of consumer claims.

| Operand / surface | Units and basis | Authority / consumer |
| --- | --- | --- |
| `UpstreamHandoff::bins_m2` | m2 water per unit upstream width per bin | authoritative upstream solver boundary outflow |
| `bin_spans_s`, `bin_dt_s` | seconds | exact partial/final-bin time basis |
| `integrate_bin_series` result | m2 per unit upstream width over the requested interval | real downstream solver integral boundary consumer |
| `width_ratio = upstream.width_m / segment.width_m` | dimensionless upstream-to-current width conversion | preserves total discharge/volume across width changes |
| upstream outlet volume | solver outflow m2 times upstream width, m3 | independent upstream ledger operand |
| downstream received volume | downstream `MassBalance::upstream_inflow_m2` times current width, m3 | independent downstream ledger operand |

`handoff_injection_is_flux_integral_conservative` and
`partial_final_bin_handoff_is_exact` reconstruct interval injection from the
produced bins/spans. `width_change_scales_handoff_for_discharge_continuity` and
`runon_only_ofe_handoff_is_nonnegative_and_conservative` independently compare
upstream outlet m3 with downstream received m3 on distinct width bases.
`solver_ledger_books_scheme_actual_boundary_fluxes` checks the booked boundary
flux rather than restating the cascade aggregate. The point sampler is not the
current real consumer when upstream state exists; its extracted helper is
characterization coverage only.
