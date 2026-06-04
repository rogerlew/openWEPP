# Unit Governance Gap Analysis

Status: completed
Evidence mode: static

Static: HPHYS0278 closes the gap where output writers could publish local
`units` metadata that drifted from canonical unit authority.

Closed:

- Hillslope WAT unit metadata now validates against `OutputUnitRegistry`.
- Watershed interchange output schema metadata now validates against
  `OutputUnitRegistry`.
- Boundary-backed rows cross-check boundary registry units.
- Publication-only rows require rationale and contract/invariant authority.
- Dynamic `key/value/units` outputs declare `unit_source = "units"` metadata
  and resolve the dynamic `value` column through output-registry rows.

Residual:

- Publication-only sediment, pollutant, ash, routing, and loss-summary columns
  remain outside runtime boundary-symbol authority by design; they are now
  explicit registry rows rather than silent local writer metadata.
- Full workspace validation remains blocked by unrelated SIMIMPL18/PL14S
  `HKERNEL-WB11-ET-E-003`.

Ran: not applicable for this artifact; validation is recorded in
`gate-results.md`.
