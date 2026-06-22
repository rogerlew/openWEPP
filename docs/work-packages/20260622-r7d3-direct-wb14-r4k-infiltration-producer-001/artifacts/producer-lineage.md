# Producer Lineage

Status: executed-held.

## Operand Lineage

| Operand | Units | Direct authority after R7D3 | Consumer |
|---|---:|---|---|
| R4I `liquid_input_m` | m | `wb12_rainfall_input` from lane-indexed direct seed surface | R4A/R4K |
| R4C `precip_input_m` | m | `wb12_rainfall_input` storage-input override | R4B scalar closure |
| R4K hyetograph | s, m s^-1 | `timem_####`/`intsty_####` interval series | R4K infiltration producer |
| R4K `cumulative_infiltration_m` | m | direct WB14 producer from hyetograph, effective conductivity, matric potential, and top-two-layer storage capacity | R4A, WB18, R4N |
| R4K `depression_storage_delta_m` | m | direct WB14 producer rainfall-excess/depression cap | R4A |
| R4L `surface_saturation_runoff_m` | m | sum of R4O `hourly_saturation_carry_m` when R4O has run; constructor handoff retained only for non-production/unit paths | R4A and R4B via `Q` |

## Direct Runtime Wiring

- R4K now mutates:
  `runoff_partition_inputs.cumulative_infiltration_m`,
  `runoff_partition_inputs.depression_storage_delta_m`,
  `percolation_inputs.same_pass_infiltration_m`,
  `percolation_inputs.same_pass_infiltration_lineage`, and
  `evapotranspiration_compute_inputs.same_pass_infiltration_m`.
- Direct day execution order now runs R4K before R4M/R4N consumers.
- R4C can consume a direct storage-input override so scalar storage closure and
  direct layer ingress use the same liquid supply.
- R4L now consumes R4O hourly saturation carry when present and rejects a
  conflicting nonzero constructor handoff.
- Remaining gap: no executor-level mutation transfers current-lane R4O/R4L
  carry arrays into the downstream lane before downstream R3A/R4J execution.

## Anti-Tautology Review

- Focused tests prove nonzero direct infiltration makes `Q < liquid_input`
  without compatibility `wb12_infiltration`.
- Malformed hyetograph and conflicting saturation handoff tests exercise
  independent fail-closed conditions rather than restating producer formulas.
- H2637 residual evidence is not accepted as parity; it is used only to name the
  next missing producer boundary.
