# Operand Lineage

Status: complete.
Evidence mode: Static.

R4A creates conservation-sensitive direct runtime state, but it does not publish
output surfaces. This table binds the direct operands before production edits.

| Operand | Units | Normalization / Basis | Source Authority | Status |
|---|---|---|---|---|
| `liquid_input_m` | `m` | direct runoff-partition boundary depth after interception/liquid supply assembly | `SC-RUNOFFPART-001#INV-RUNOFFPART-001`, `INV-RUNOFFPART-016` | authoritative within R4A direct span |
| `runon_input_m` | `m` | direct same-boundary upstream/runon depth | `SC-RUNOFFPART-001#INV-RUNOFFPART-001`, `INV-RUNOFFPART-009` | authoritative within R4A direct span |
| `cumulative_infiltration_m` | `m` | cumulative infiltration consumed by the direct partition closure | `SC-RUNOFFPART-001#INV-RUNOFFPART-001`, `INV-RUNOFFPART-027` | authoritative within R4A direct span |
| `depression_storage_delta_m` | `m` | retained depression-storage reduction term in the partition closure | `SC-RUNOFFPART-001#INV-RUNOFFPART-001`, `INV-RUNOFFPART-002` | authoritative within R4A direct span |
| `surface_saturation_runoff_m` | `m` | current-OFE saturation addback included in final `Q` | `SC-RUNOFFPART-001#INV-RUNOFFPART-014` | authoritative within R4A direct span |
| `partition_runoff_m` | `m` | computed residual before saturation addback | `SC-RUNOFFPART-001#INV-RUNOFFPART-001` | authoritative direct output, not public output |
| `q_runoff_m` | `m` | computed runoff depth after saturation addback | `SC-RUNOFFPART-001#INV-RUNOFFPART-009`, `INV-RUNOFFPART-014` | authoritative direct output, not public output |
| `closure_residual_m` | `m` | independent direct residual check for accepted operands | `SC-RUNOFFPART-001#INV-RUNOFFPART-001` | diagnostic proof surface |

Anti-aliasing requirement:

Focused tests must choose values where accepted `q_runoff_m` differs from
precipitation-only, no-depression-storage, no-saturation-addback, and
infiltration-as-runoff candidates.
