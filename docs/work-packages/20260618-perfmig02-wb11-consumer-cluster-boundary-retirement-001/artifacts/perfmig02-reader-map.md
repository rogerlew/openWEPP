# PERFMIG02 Reader Map

Static: read the PERFMIG01 indexed writeback boundary, scheduler transfer logic, hydrology support helpers,
`kernel_phases_mod/`, and direct `state_surface` / `flux_surface` call sites with `rg`.

Ran: no executable gate in this artifact; focused executable gates are recorded in
`perfmig02-gate-results.md`.

## Scope

PERFMIG01 writes the WB11 warm-rain runoff success path as an indexed payload with 543 state updates plus
8 flux updates. PERFMIG02 mapped the downstream readers of that payload and separated them into:

- internal hot scalar readers that can read dense `SymbolId` values through `HillslopeKernelRequest`;
- internal duplicate/diagnostic symbols safe to keep indexed-only between phases;
- public/reporting/diagnostic boundaries that must remain logically materialized for now.

## Reader Classes

| Symbol family | Downstream readers | PERFMIG02 treatment |
|---|---|---|
| `wb12_infiltration` | WB17 infiltration/evap same-pass logic, WB18 plant/percolation lineage, runoff reconciliation helpers | Reads go through `optional_state_scalar_for_symbol` / `require_state_scalar_for_symbol`, which now prefer indexed hot state values when present. Logical materialization is retired and stale logical values are removed. |
| `wb12_runoff_carryover` | WB12 storage/erosion reconciliation carryover helpers | Reads go through `optional_flux_scalar_for_symbol` / `require_flux_scalar_for_symbol`, which now prefer indexed hot flux values when present. Logical materialization is retired and stale logical values are removed. |
| `wb12_runoff_reconciled` | No required production reader found after WB12/WB14 in the warm-rain indexed branch; it is an internal duplicate/status state | Logical materialization retired. Indexed authority remains updated. |
| `wb14_soil_conductivity_m_s`, `wb14_effective_conductivity_m_s`, `wb14_matric_potential_m` | No required production reader found after WB14 in the warm-rain indexed branch; these are internal conductivity/diagnostic states | Logical materialization retired. Indexed authority remains updated. |
| Public runoff/infiltration fluxes including `I`, `Irr`, `Q`, `S` | WB16 peak/runoff, WB12 storage reconciliation, WB13 publication, downstream reporting | Dense-first reads are available through hot-symbol helpers, but logical materialization is retained because publication/reporting boundaries still consume named logical surfaces. |
| Snow/frost/irrigation diagnostic families | Snow/frost coupling helpers, publication diagnostics, dormant/non-warm-rain branches | Dense-first helper access is available when the symbol is in the hot table and an indexed surface exists. Logical materialization remains because these are outside the retired WB11 warm-rain internal set. |
| MOFE transfer arrays and EROD transfer surfaces | Scheduler transfer extraction and erosion coupling read array/transfer symbols directly | Not part of the PERFMIG01 543+8 internal materialization retirement. Existing indexed transfer paths remain unchanged. |

## Direct Logical Reader Audit

The normal hydrology scalar helper path now centralizes dense-first reads in
`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`.
Direct `request.state_surface` / `request.flux_surface` sites that remain are outside the retired six-symbol
internal set or are fallback/publication/guard paths. The scheduler still validates consumer boundaries on
the logical surface for unmigrated public inputs, but the retired symbols are not required by that boundary.

## Boundary To Retire

The materialization boundary targeted by this rung is the compatibility step in
`apply_indexed_kernel_writeback`. PERFMIG02 introduced an explicit materialization policy so selected ids
update indexed authority while skipping logical insertion:

- state: `wb12_infiltration`, `wb12_runoff_reconciled`, `wb14_soil_conductivity_m_s`,
  `wb14_effective_conductivity_m_s`, `wb14_matric_potential_m`;
- flux: `wb12_runoff_carryover`.

This is deliberately narrower than "all 543+8" because publication/reporting readers still require named
logical values for several public outputs.
