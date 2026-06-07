# Contract-Test Implementation Evidence

Status: closed-with-follow-up-postreview

Evidence mode: Ran

Test added:

- `snowsci_stage1_mixed_signed_melt_routes_authoritative_pack_loss`
  in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`

Red gate:

- Ran `cargo test -p openwepp-hillslope-orchestrator snowsci_stage1_mixed_signed_melt_routes_authoritative_pack_loss -- --nocapture`
- Pre-fix result: failed because `routed_melt_total_m` was the signed net
  `0.001204946614 m` instead of the authoritative positive pack loss
  `0.007376104224 m`.

Green gate:

- Same command passed after production correction.

Regression assertion:

- Mixed positive/negative raw melt routes the positive pack loss.
- `snowpack_state_loss_m` equals `routed_melt_total_m`.
- Routed hourly melt values are non-negative and sum to the same scalar.
