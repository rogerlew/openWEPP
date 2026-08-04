# Operand Lineage

Status: `complete / production edits authorized after contract RED gate`

Evidence mode: `Static: exact producer and consumer lineage`

## Compact Ledger Operands

| Ledger | Operand | Units | Time/sign basis | Exact producer | Authority role | Downstream consumer | Rejected aliases |
|---|---|---|---|---|---|---|---|
| solid-to-liquid | `raw_signed_melt_m` | `m` SWE | daily, signed | sum of exact hourly `SnowHourlyState.melt_raw_m` | diagnostic context in owned compact ledger | trace / audit | applied loss or positive sum |
| solid-to-liquid | `redistributed_positive_melt_m` | `m` SWE | daily, nonnegative | `SnowMeltRedistributionOutcome.routed_melt_total_m` | exact redistribution outcome | trace / audit | signed raw melt |
| solid-to-liquid | `snowpack_swe_loss_m` | `m` SWE | daily, nonnegative | bounded authoritative state loss | authoritative mass transition | runtime, storage guard, trace | raw signed melt |
| solid-to-liquid | `rain_released_m` | `m` water | daily, nonnegative | `ActiveSnowDailyTotals.rain_released_m` | authoritative released rain | runoff handoff / trace | retained rain |
| exact linked handoff | `liquid_handoff_m` | `m` water equivalent | daily, nonnegative | existing `resolve_snow_partition_terms` result | authoritative downstream liquid forcing | Stage-3 `incoming_liquid_m`, runner hyetograph | raw or redistributed melt alone |
| liquid disposition | `incoming_liquid_m` | `m` SWE | daily, nonnegative | exact `liquid_handoff_m` argument to Stage 3 | compact ledger | Stage-3 solve / trace | raw or redistributed CoE melt |
| liquid disposition | `routed_liquid_m` | `m` SWE | daily, nonnegative | `route_stage3_liquid_through_layers` tuple | compact ledger | trace / temperature outcome | upstream liquid handoff |
| liquid disposition | `retained_liquid_delta_m` | `m` SWE | daily, signed | Stage-3 layer-store delta tuple | compact ledger | trace / independent parser | CoE retained store or omission |
| liquid disposition | `refrozen_liquid_m` | `m` SWE | daily, nonnegative | Stage-3 layer-routing tuple | compact ledger | layer mass / trace | double refreeze |
| liquid disposition | `liquid_closure_residual_m` | `m` SWE | daily, signed | existing guarded Stage-3 residual | guard/diagnostic | independent parser | trusting producer residual |

## Accepted Identities

The upstream independent identity is:

    liquid_handoff_m - snowpack_swe_loss_m - rain_released_m = 0

The downstream independent identity is:

    incoming_liquid_m - routed_liquid_m
      - retained_liquid_delta_m - refrozen_liquid_m
      = liquid_closure_residual_m

When Stage 3 is enabled, `incoming_liquid_m` is the same exact argument value as
the upstream `liquid_handoff_m`; no projection or recomputation intervenes.
Stage-3-disabled ledger values remain zero by the existing disabled branch.

The frozen baseline separates aliases: upstream closure is within
`1.3878e-17 m`, Stage-3 closure within `1.2272e-17 m`, all `8615` enabled rows
link the handoff, and raw signed melt differs from authoritative pack loss on
`3844` rows. The predecessor independently rejects omitted retention,
top-level routed-melt substitution, CoE-store substitution, and doubled
refreeze. No compact operand is inferred from a neighboring output.
