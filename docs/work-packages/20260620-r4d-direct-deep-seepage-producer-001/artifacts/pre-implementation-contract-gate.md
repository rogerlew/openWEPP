# R4D Pre-Implementation Contract Gate

Status: complete.
Evidence mode: Static.

Gate completed before Rust edits:

| Check | Result | Evidence |
|---|---|---|
| Deep-seepage authority | PASS | `SC-PERC-001#REF-PERC-CH5-BAL` declares daily water-balance closure with below-root-zone `D`; `SC-PERC-001` required outputs include `D` and `Pe`. |
| WB12 consumer authority | PASS | `SC-WATBAL-001` WB12 storage-reconciliation authority consumes percolation/deep-seepage loss in the storage budget. |
| Handoff-only boundary | PASS | R4D only promotes a direct handoff operand into R4B input state; full WB18 equations remain out of scope. |
| Publication boundary | PASS | `SC-WATBAL-001` maps public `Dp` as downstream `D -> Dp`; R4D does not edit public output writers, schemas, or manifests. |
| Compatibility boundary | PASS | The direct runtime can consume a typed direct handoff field and does not require compatibility storage/request/writeback/symbol lookup access. |
| Default activation excluded | PASS | R4D remains opt-in through existing direct-runtime selection only. |
| Contract amendment required | PASS | No canonical contract amendment is required for this narrow handoff slice. |
