# Default Rollback Evidence

Status: complete
Evidence mode: Static/Ran

## Default Selector

Static: absent `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL` selects
`SnowPhasePartitionModel::LegacyRst`.

Static: empty `OPENWEPP_SNOWDENSITY1035_PHASE_MODEL` also selects
`SnowPhasePartitionModel::LegacyRst`.

Static: the package does not add parser, runfile, or user CLI activation for the
candidate partition model.

## Legacy Branch Identity

Ran: `snowdensity1035b_legacy_partition_keeps_threshold_identity` confirms the
explicit `LegacyRst` selector preserves the pre-existing `RST` threshold branch
for active precipitation.

Static/Ran: the initial full `cargo test --workspace` run failed
`hphys0299_static_openwepp_sources_publish_depth_and_water_equiv_separately`
because the generalized opt-in implementation obscured the exact legacy
`rain_m / wntdur * 10.0` source expression required by the snowfall-depth
provenance guard. The fix restored explicit legacy source evidence through
`simimpl28_legacy_stmtim_snowfall_depth_m`; the targeted guard and the final full
workspace test both passed.

## Opt-In Boundary

Ran: `snowdensity1035b_harder_pomeroy_hourly_is_fractional_and_conservative`
confirms opt-in near-freezing precipitation can split into rain and snow while
`hrrain + hrsnow / 10` reconstructs active precipitation within `1e-12`.

Ran: `snowdensity1035b_supersaturated_dewpoint_normalizes_to_exact_saturation`
confirms supersaturated dewpoint-derived RH normalizes only to exact saturation
for the opt-in path.
