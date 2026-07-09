# Protected Fallback Evidence

Status: `EXECUTED`
Evidence mode: `Ran` plus `Static`

Protected fallback:

- All-no-hourly contributor inlets remain on the `SC-ROUTE-001`
  Eq. [13.4.1]-[13.4.2] triangular fallback.
- Existing WS10/WS11 typed watershed tests with no hourly arrays continue to
  pass in `wshedw5_typed_watershed_runtime_contract`.

Fail-closed hourly authority:

- Mixed one-inlet contributor set: one hillslope with a complete hourly pair
  and one hillslope without hourly arrays fails with
  `WKERNEL-WS10-CHANNEL-E-003` before `routed_channels` is populated.
- Malformed one-inlet contributor set: hourly runoff length 24 and hourly
  sediment length 23 fails with the same channel domain guard before state
  write.
- Hourly hillslope contributors with dependency nodes lacking channel-hourly
  surfaces are rejected before daily fallback can carry an active-hourly claim.
- All-hourly multi-contributor inlets route successfully and superpose the
  hourly water/sediment pair at the channel inlet.

Ran:

- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract`:
  18 passed, including
  `mt3_all_hourly_contributors_superpose_at_channel_inlet`,
  `mt3_hourly_contributor_with_dependency_node_fails_closed`, and
  `mt3_mixed_or_malformed_hourly_pair_fails_closed_before_routing_state`.
- `cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity`:
  W7R release/test fixture remains protected and passes.
