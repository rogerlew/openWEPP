# R4B Pre-Implementation Contract Gate

Status: complete.
Evidence mode: Static.

Gate completed before Rust edits:

| Check | Result | Evidence |
|---|---|---|
| WB12 storage equation authority | PASS | `SC-WATBAL-001#WB12 Reconciliation Authority Addendum` defines `wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S - Q - ET - D - Qd`. |
| `Q` authority | PASS | R4A exports direct `DirectRunoffDownstreamOperands::q_runoff_m`; R4B consumes that value as direct `Q`. |
| `S`, ET, `D`, `Qd` authority | PASS | `SC-WATBAL-001` defines the required storage inputs; `SC-SUBHYD-001#WB12 Reconciliation Coupling Addendum` defines `Qd` as the required subsurface-loss term. R4B keeps these as explicit direct operands because their producers are not migrated in this slice. |
| Contract amendment required | PASS | No amendment required for this narrow direct-runtime consumer slice. |
| Publication boundary | PASS | R4B does not edit WB13/WAT/HBP/PASS/loss schemas, metadata, or publication paths. |
| Compatibility boundary | PASS | R4B does not require compatibility storage/request/writeback/symbol lookup access inside direct runtime. |
| Default activation excluded | PASS | R4B remains opt-in through existing direct-runtime selection only. |
