# Operand Lineage

Status: frozen before result execution

Evidence mode: Static

| Operand | Units | Stage / source | Role |
| --- | --- | --- | --- |
| Fixture precipitation | `m d^-1` | checked-in CLIGEN climate | supplied all-phase input; not observed truth |
| Observed `WTEQ` | `m` | normalized SNOTEL point series | diagnostic storage correspondence |
| Observed `PREC` increment | `m d^-1` | guarded difference of cumulative SNOTEL series | dry-period guard; not bridged across gaps/resets |
| Hourly snowfall SWE | `m` | phase partition trace | modeled solid input |
| `rain_retained_m` / `rain_released_m` | `m` | snow-contact rain partition | modeled liquid contact, kept separate |
| `runtime_swe_before/after_m` | `m` | direct snow state | modeled storage |
| `snowpack_swe_loss_m` | `m` | upstream solid-to-liquid transition | authoritative pack-state loss |
| `raw_melt_m` | `m` | signed daily CoE melt | generated-melt diagnostic; not routed liquid |
| `amelt` | `m` water equivalent | empirical CoE radiation term | formula contribution, not measured shortwave energy |
| `bmelt` | `m` water equivalent | empirical temperature/cloud term | mixed formula contribution, not pure sensible heat |
| `cmelt` | `m` water equivalent | empirical wind/dewpoint/canopy/temperature term | mixed formula contribution, not pure turbulent heat |
| `dmelt` | `m` water equivalent | empirical rain/temperature term | formula contribution, not measured rain energy |
| Cap adjustment | `m` water equivalent | bounded CoE application | separates uncapped from applied melt |
| Stage-3 energy fields | `J m^-2` | downstream layered liquid/thermal solver | response/disposition, not upstream melt cause |
| Stage-3 refreeze/retain/route | `m` | downstream liquid ledger | post-generation disposition |
| Scaled Snowbird lane | same as canonical | precipitation-only development derivative | state/input sensitivity only |

Rejected aliases include state loss plus routed melt, routed melt alone,
empirical melt terms relabeled as measured flux shares, Stage-3 energy relabeled
as upstream CoE causation, and scaled Snowbird treated as canonical truth.
