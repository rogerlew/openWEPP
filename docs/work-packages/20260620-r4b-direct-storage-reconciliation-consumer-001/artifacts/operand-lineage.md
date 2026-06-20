# R4B Operand Lineage

Status: complete.
Evidence mode: Static.

R4B creates conservation-sensitive direct runtime state, but it does not publish
that state. Public output authority remains out of scope.

| Operand | Units | Sign / basis | Authority | R4B role |
|---|---|---|---|---|
| `storage_initial_m` | `m` | nonnegative storage depth before WB12 storage reconciliation | `SC-WATBAL-001#WB12 Reconciliation Authority Addendum` | authoritative direct input |
| `precip_input_m` | `m` | nonnegative precipitation/liquid storage input depth | `SC-WATBAL-001#WB12 Reconciliation Authority Addendum` | authoritative direct input |
| `snow_coupling_m` | `m` | signed `S`, positive melt / negative accumulation | `SC-WATBAL-001#CLIM05 Deterministic Coupling Rule` | authoritative direct input |
| `q_runoff_m` | `m` | nonnegative runoff loss depth | R4A direct `DirectRunoffDownstreamOperands::q_runoff_m`; `SC-WATBAL-001#WB12 Reconciliation Authority Addendum` | authoritative direct input |
| `evapotranspiration_m` | `m` | nonnegative ET loss depth | `SC-WATBAL-001#WB12 Reconciliation Authority Addendum` | authoritative direct input |
| `deep_seepage_m` | `m` | nonnegative deep seepage/percolation loss depth `D` | `SC-WATBAL-001#HPHYS0239 WB19->WB12->WB13 Ordering and Flux-Authority Handoff Addendum` | authoritative direct input |
| `subsurface_loss_m` | `m` | nonnegative subsurface loss depth `Qd` | `SC-SUBHYD-001#INV-SUBHYD-022`, `SC-SUBHYD-001#INV-SUBHYD-023` | authoritative direct input |
| `storage_reconciled_m` | `m` | nonnegative reconciled storage depth | `SC-WATBAL-001#WB12 Reconciliation Authority Addendum` | direct process state |
| `closure_residual_m` | `m` | signed residual; must be within direct tolerance | `SC-WATBAL-001#INV-WATBAL-016` | direct closure diagnostic |

Anti-alias obligations:

- Do not source `q_runoff_m` from publication fields, compatibility surfaces, or
  R3B diagnostic ledger values.
- Do not infer `S`, ET, `D`, or `Qd` from missing direct producers.
- Do not treat `storage_reconciled_m` as a public WB13/WAT storage value in R4B.

Pre-implementation conclusion:

The operand lineage is sufficient for the narrow R4B direct runtime slice. R4B
uses explicit direct operands for upstream producers that are not yet migrated,
and consumes only R4A direct runoff as an upstream direct process result.
