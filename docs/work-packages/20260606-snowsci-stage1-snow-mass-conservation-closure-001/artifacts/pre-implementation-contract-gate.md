# Pre-Implementation Contract Gate

Status: closed-with-follow-up-postreview

Evidence mode: Ran

Gate result:

- Contract authority amended before production correction.
- Contract-derived test added and run red before production correction.

Commands:

- `cargo test -p openwepp-hillslope-orchestrator snowsci_stage1_mixed_signed_melt_routes_authoritative_pack_loss -- --nocapture`

Observed red:

- Assertion failed on `redistribution.routed_melt_total_m`.
- Existing implementation routed signed net melt instead of the storage-loss
  scalar.
