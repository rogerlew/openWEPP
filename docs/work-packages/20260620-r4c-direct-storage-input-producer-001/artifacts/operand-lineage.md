# R4C Operand Lineage

Status: complete.
Evidence mode: Static.

R4C creates direct input operands for R4B storage reconciliation. It does not
publish storage or precipitation outputs.

| Operand | Units | Sign / basis | Authority | R4C role |
|---|---|---|---|---|
| `soil_water_m` | `m` | nonnegative direct storage depth before R4B | `SC-WATBAL-001#WB12 Reconciliation Authority Addendum` | direct source for `storage_initial_m` |
| `precipitation_m` | `m` | nonnegative daily precipitation input | `SC-WATBAL-001#WB12 Reconciliation Authority Addendum` via R3A direct input accounting | direct source for `precip_input_m` |
| `storage_initial_m` | `m` | nonnegative storage depth before storage reconciliation | `SC-WATBAL-001#WB12 Reconciliation Authority Addendum` | direct downstream operand for R4B |
| `precip_input_m` | `m` | nonnegative precipitation storage-input depth | `SC-WATBAL-001#WB12 Reconciliation Authority Addendum` | direct downstream operand for R4B |

Anti-alias obligations:

- Do not source `precip_input_m` from R3A `transfer_input_m` or
  `total_accounted_input_m`.
- Do not source `precip_input_m` from R4A `liquid_input_m` or `runon_input_m`.
- Do not source `storage_initial_m` from publication storage or R3B diagnostic
  ledger fields.
- Do not treat the R4C shadow projection as public WB13/WAT/PASS authority.

Pre-implementation conclusion:

The operand lineage is sufficient for the narrow R4C direct runtime slice. R4C
uses direct storage state and R3A direct precipitation only; all other R4B
storage terms remain explicit direct operands until their own producer packages.
