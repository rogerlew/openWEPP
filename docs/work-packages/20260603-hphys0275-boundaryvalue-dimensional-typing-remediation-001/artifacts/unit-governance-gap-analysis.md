# Unit Governance Gap Analysis

Status: completed
Evidence mode: static

Static: HPHYS0275 closes the first runtime producer wave for hillslope daily
climate and SIMIMPL28 hourly forcing. Remaining gaps are intentional follow-up,
not silent omissions.

## Closed in HPHYS0275

- `BoundaryValue` variants exist for meters, elapsed seconds, hour-of-day,
  `m s^-1`, `Ly d^-1`, `MJ m^-2 d^-1`, `MJ m^-2 h^-1`, Celsius, density,
  and unit-interval fractions.
- Hillslope daily climate producer emits typed values for selected dimensional
  non-direction fields.
- SIMIMPL28 hourly producer emits typed values for selected radiation,
  temperature, cloud fraction, rain, and snowfall fields.
- Registry posture marks migrated aliases `TypedRequired`.

## Follow-Up Gaps

- Wind direction remains scalar/follow-up until a direction-specific wrapper
  and domain contract are added.
- Watershed-prefixed climate aliases remain scalar/follow-up.
- Snow runtime state and retained snow trace families remain follow-up.
- Output publication rows remain follow-up under output metadata/typing work.
- Soil/WB13 runtime geometry and storage rows remain follow-up.
- Exact per-symbol dynamic error labels for `timem_####`/`intsty_####` remain
  diagnostic ergonomics follow-up.

Ran: not-run; static gap analysis.
