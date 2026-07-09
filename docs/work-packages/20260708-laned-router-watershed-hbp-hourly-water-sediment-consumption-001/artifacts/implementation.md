# Implementation

Status: `EXECUTED`
Evidence mode: `Static`

Production changes:

- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`
  adds `direct_hillslope_hourly_authority`.
- The WS10 inlet path now classifies contributor hourly authority before
  assembling peaks:
  - `(0, 0)` hourly arrays on every hillslope: use authorized daily triangular
    fallback.
  - `(24, 24)` on every hillslope and no dependency nodes: consume hourly
    water/sediment arrays.
  - partial cardinality, mixed hourly/no-hourly contributors, or hourly
    hillslopes with dependency nodes lacking channel-hourly surfaces: hard fail
    with the channel domain guard.
- Existing hourly consumers remain the production path:
  - water: `superposed_hourly_limb(&summed_hourly_volume_m3)`;
  - sediment mass: `sum(hourly_sediment_mass_kg)`;
  - sediment time base: active-hour span of superposed `S_h`.

Test/docs changes:

- `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` adds M-T3
  timing sensitivity, all-hourly multi-contributor, dependency-node
  fail-closed, and malformed/mixed fail-closed vectors.
- `crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs` adds a
  production CLI proof that schema-1.1 HBP hourly pairs populate the watershed
  contribution and change channel output timing.
- `crates/openwepp-watershed-orchestrator/src/lib.rs` updates an old helper
  comment to point at the stricter production guard.
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` rev 49
  authorizes the tightened all-hourly/no-hourly rule; rev 50 adds the
  profile-only Binding Exposure Index.

No changes:

- No hillslope producer math, active mesh policy, default activation, baseflow
  physics, or HBP binary layout changed.
