# Operand Lineage

Status: `prospectively frozen`.

Evidence class: `Static`.

| Operand | Canonical meaning | Units/basis | Reduction | Rejected alias |
| --- | --- | --- | --- | --- |
| external complete carrier | absorbed shortwave + net longwave + sensible + latent + precipitation-advected heat | J m^-2 per evaluated substep | ordered substep sum -> day -> WY | CoE melt energy; daily aggregate without support proof |
| active internal conduction | energy transferred into/out of the active layer | J m^-2 per evaluated substep | included once in active-layer energy; paired lower-layer transfer must cancel internally | external carrier; lower-interface flux counted twice |
| complete active energy | external carrier + active internal conduction | J m^-2 per evaluated substep | ordered substep sum -> day -> WY | surface carrier alone |
| cold-content response | positive energy consumed warming resolved ice to the fusion boundary | J m^-2 | state-transition reconstruction | latent fusion energy |
| fusion response | positive energy consumed by ice melt after cold content | J m^-2 | state-transition reconstruction | CoE melt or snowfall loss |
| terminal unallocated energy | residual only after resolved ice is exhausted | J m^-2 | explicit diagnostic sum | unused-positive-energy legacy aggregate |
| support | evaluated sequential substep with resolved pre-state | boolean/count | exact ordered membership and duration | day-level `stage3_evaluated` alone |

Every primitive is diagnostic/evaluation-only; CoE state remains authoritative.
Schema v4 E00/E01 custody uses only `schema`, `day_index`, `lane_index`,
`stage3_energy_enabled`, `stage3_shadow_hourly_complete_energy_j_m2`,
`stage3_shadow_complete_energy_j_m2`, and
`stage3_shadow_maximum_energy_closure_residual_j_m2`. Positive energy points
toward snow; values are `J m^-2`; the 24 hourly values must sum to the daily
complete value under the energy tolerance. Numeric zero is retained as the
historical aggregate value and cannot prove tuple support or N/A.

Schema v6 E10/E11 custody uses `stage3_operator_reconciliation.schema_version`,
`hourly_status`, and ordered `tuples`. The common aggregate operand is tuple
`legacy_sequential_complete_j_m2`, positive toward snow in `J m^-2`.
Independent primitive closure uses tuple `net_shortwave_w_m2`,
`net_longwave_w_m2`, `sensible_flux_w_m2`, `latent_flux_w_m2`,
`precipitation_advected_flux_w_m2`, `complete_external_flux_w_m2`,
`duration_seconds`, and `internal_active_lower_conduction_j_m2`. Support exists
only when tuple `applicable=true`, `applicability_reason=evaluated`, and the
matching hourly status is evaluated. Mass/cold endpoint fields are exactly
those bound by INV-SNOWFREEZE-096. `null` is N/A; numeric zero cannot replace
absent support.

Both adapters use exact dates derived from `day_index`, WY1990--2024 windows,
and Python `statistics.median` order. Cross-schema effects are aggregate
contrasts only. Exact hashes, tolerances, and rejected aliases are frozen in
`protocol-freeze.json` and tested adversarially.
