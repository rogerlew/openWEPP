# review_agent_a

Status: complete

Evidence mode: Static

## Findings

Chandrasekhar reviewed the first implementation pass as a Rust correctness
gate and identified four blocker findings:

1. The initial heat-flow update was effectively one-sided surface loss and did
   not separately model/publish the lower-front unfrozen-soil heat input
   (`Quf`) required by `SC-SNOWFREEZE-001#INV-SNOWFREEZE-006`.
2. Frozen-water exchange could create or delete storage by clamping liquid
   `wb11_soil_water` instead of failing closed on overdraw.
3. Adding WAT `frdp` changed the interchange schema without advancing or
   documenting the dataset version.
4. Runner WAT publication validated `frdp` as nonnegative but did not reject
   depth beyond physical profile depth.

## Finding Disposition

| # | Finding | Disposition (accepted/rejected/deferred/follow-up) | Rationale |
|---|---------|-----------------------------------------------------|-----------|
| 1 | Missing separate `Qsrf`/`Quf` heat-flow model and diagnostics. | accepted | `coupling.rs` now computes surface heat loss through snow/residue/frozen path separately from lower-front unfrozen-soil heat input; the signed balance drives latent depth increments. `fdhp01_contract_heat_flow_publishes_separate_surface_and_unfrozen_fluxes` passes. |
| 2 | Frozen-water storage exchange could silently mask liquid overdraw. | accepted | `coupling.rs` now hard-fails if newly frozen storage exceeds available liquid `wb11_soil_water`, and credits thawed frozen storage back to liquid water. `fdhp01_contract_frozen_water_exchange_hard_fails_on_liquid_overdraw` and `fdhp01_contract_warm_heat_flow_thaws_prior_deep_frost` pass. |
| 3 | WAT schema/versioning drift. | accepted | WAT dataset version is now `1.4`; `SC-WATBAL-001` and the runner CLI spec document the additive `frdp` extension; schema metadata tests pass. |
| 4 | WAT `frdp` not bounded by profile depth. | accepted | Runner publication rejects `frost.runtime_frdp_m > wb13_profile_depth_mm / 1000`; `fdhp01_wb13_publication_rejects_frdp_beyond_profile_depth` passes. |
