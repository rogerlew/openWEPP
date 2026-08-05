# Wet-Compaction Operand Lineage

Status: complete / correction authorized

Evidence mode: Static

All mass values below are `m water equivalent`; the density boundary converts
the selected daily value to `kg m^-2` with `1000 kg m^-3`. The wet-compaction
operator itself normalizes that liquid mass by current snow mass.

Required deliberately non-aliasing candidates:

| Candidate | Formula | Stage / result |
| --- | --- | --- |
| Current | `snowpack_state_loss + routed_melt` | Rejected later-stage duplicate. Compact closure makes it `2 * state_loss + rain_released`. |
| Routed-only | `routed_melt` | Rejected export-stage alias; omits generated melt and rain retained in-pack. |
| Loss plus all rain | `snowpack_state_loss + rain_retained + rain_released` | Rejected bounded-storage alias; generated positive melt may exceed state loss when liquid is retained. |
| Generated liquid | `sum(max(hourly melt_raw_m, 0)) + rain_retained + rain_released` | Authoritative pre-runoff H2O addition for hours whose pack was active at interval start. |
| Store | retained-liquid level or delta | Rejected state alias; a level/change is not the day's melt-plus-rain addition. |

`melt_raw_m` here is the exact capped/applied CoE hourly generation surface
also published as `coe_melt_applied_m`; it is not uncapped empirical melt, the
signed daily sum, redistributed melt, bounded SWE loss, or routed melt.
`rain_retained + rain_released` is snow-contact rain, not raw daily rain. A
mixed event that first creates snow from zero start-of-hour cover is excluded,
matching PySnobal's start-of-timestep `snowcover` gate.

The anti-alias test vector freezes generated melt `0.011`, retained rain
`0.003`, released rain `0.005`, state loss `0.006`, routed handoff `0.011`, and
retained-store change `0.002`. The authoritative result is `0.019`; every
rejected formula is distinct.

`SC-SNOWFREEZE-001` v125 now binds this decision through
`INV-SNOWFREEZE-092`, `OBL-SNOWFREEZE-P-065`, and `TOL-SNOWFREEZE-017`.
