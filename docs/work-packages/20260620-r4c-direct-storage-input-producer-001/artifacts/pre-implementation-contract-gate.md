# R4C Pre-Implementation Contract Gate

Status: complete.
Evidence mode: Static.

Gate completed before Rust edits:

| Check | Result | Evidence |
|---|---|---|
| WB12 storage-input authority | PASS | `SC-WATBAL-001#WB12 Reconciliation Authority Addendum` requires `wb12_storage_initial` and `wb12_precip_input` as storage reconciliation inputs. |
| Storage initial source | PASS | Direct `DirectWaterState::soil_water_m` is the current direct storage state before R4B mutation. R4C records it as `storage_initial_m`. |
| Precipitation source | PASS | R3A direct input accounting records direct precipitation separately from transfer and total accounted input. R4C consumes that field as `precip_input_m`. |
| Contract amendment required | PASS | No amendment required for this narrow direct-runtime producer slice. |
| Publication boundary | PASS | R4C does not edit WB13/WAT/HBP/PASS/loss schemas, metadata, or publication paths. |
| Compatibility boundary | PASS | R4C does not require compatibility storage/request/writeback/symbol lookup access inside direct runtime. |
| Default activation excluded | PASS | R4C remains opt-in through existing direct-runtime selection only. |
