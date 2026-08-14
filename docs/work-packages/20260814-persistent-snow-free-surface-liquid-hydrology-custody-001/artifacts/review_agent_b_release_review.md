# Review Agent B Release Review — Hydrology And Ownership

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `9ab4b1f1786d2f559bbcf54677dfe59b83c94610`

Verdict: `HOLD / one material unsupported-domain guard defect remains / no authority HOLD`.

This fresh release review read the complete version-5
`SC-SURFACELIQUID-001`, the current `SC-LANDSURFACEENERGY-001` executable
domain, the dependency package and all retained Review-A and Review-B history,
the persistent owner, ingress, independent closure, shared WB14 transition,
unified real-hydrology bridge, production frame/state surfaces, and focused
integration tests. It inspected the complete custody path rather than only the
latest error-context diff.

## Material finding

### B-RELEASE-HIGH-001 — the public bridge admits frozen/thawing production state and one snow-liquid-only state

`validate_native_shadow_domain()` rejects only a lane for which
`lane.winter_column.snow.has_runtime_state()` is true or
`snow_runtime_carry` is present
(`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:820-835`).
It does not inspect:

- `lane.winter_column.frost.has_runtime_state()`;
- `lane.frost_runtime_carry`;
- production-layer `frozen_depth_m` or `frozen_water_m`; or
- `lane.winter_column.snow.liquid_water_retained_m` directly.

The omission is reachable. `DirectSnowLaneState::has_runtime_state()` checks
snow geometry, density, settling, albedo, and layers but omits
`liquid_water_retained_m`
(`crates/openwepp-hillslope-orchestrator/src/winter_column.rs:233-243`), while
the production state validator accepts any finite nonnegative retained liquid
(`direct_runtime/00_core_frames.rs:2238-2247`). A snow-liquid-only state can
therefore pass the bridge's E004 preflight. Production layers with positive
`frozen_depth_m` and `frozen_water_m` also pass this preflight; the existing
frozen test exercises only the lower-level water arbiter and expects a
zero/FrozenSource authorization
(`tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs:601-646`).
It does not call the public unified LSE bridge or prove rejection before the
fixed-cap callback.

This conflicts with both binding contracts:

- `SC-SURFACELIQUID-001` excludes frozen/thawing surface liquid and requires
  snow, terminal snow, frozen, or thawing branches to reject as
  `SURFACELIQUID-E-004` before candidate work (lines 42-44 and 461-480); and
- `SC-LANDSURFACEENERGY-001` requires liquid/unfrozen water and soil and says
  frozen or thawing material returns a typed unsupported error before
  calculation (lines 413-419 and 612).

Because the request batch binds only the production snapshot digest, it does
not prove that the potential problem used unfrozen production operands. A
caller can bind the exact digest of a frozen production frame while supplying
a structurally valid surface-liquid request and callback. Treating a frozen
soil layer as merely zero-authorized water therefore does not enforce the
selected LSE model domain.

Required correction: locate the first canonical offending lane before
authorization/callback work and return contextual `SURFACELIQUID-E-004` for
every admitted production representation of snow, retained snow liquid,
frozen material, or thawing/frost state. Preserve the exact hydrology owner,
OFE, tile, surface/source, transaction, and beginning hash. Add public-bridge
poisons for at least (1) positive production-layer frozen depth/water, (2) a
frost/thaw runtime or carry state, and (3) positive snow retained liquid with
all other snow fields zero; assert the callback is not invoked and the
production frame remains byte-identical.

This is a bounded in-scope implementation defect. It needs no new contract,
model identity, package, tolerance, or proxy behavior.

## Confirmed closures

- The latest pre-callback thermal-expectation correction reports the exact
  soil-thermal owner and the actual later OFE/tile mismatch before invoking the
  callback.
- Final LSE/soil-thermal candidates require exact tile topology, exact ordered
  thermal layers, finite carried thermal values, owner IDs, and beginning
  digests. Missing rows use the expected receiver identity; malformed and
  extra rows preserve the actual offender.
- Rollback validation requires exactly the three owners this bridge constructs:
  LSE, hydrology, and soil thermal. Missing, malformed, extra, reordered, and
  changed-digest rows preserve the first canonical expected or actual owner.
- Production receiver closure reconstructs every ordered layer and the
  production aggregate as
  `theta_m + residual_theta * max(depth_m-frozen_depth_m,0)`. The retained
  nonzero-residual vector distinguishes the prior incorrect `sum(theta_m)`
  reconstruction.
- Same-snapshot surface D/A/F remains sealed and independently re-derived.
  Authorization is proportional to immutable beginning supply, identity is
  preserved, `0 <= F <= A <= D`, and only finalized use is debited.
- Signed condensation is credited before capacity overflow. Overflow retains
  mass/enthalpy identity, enters only post-resource ingress, and cannot satisfy
  same-interval authorization.
- Persistent configuration/state bytes, transaction lineage, restart, and the
  48-step WB14 continuation remain strict and deterministic. Actual
  infiltration uses the shared production same-pass transition on a clone;
  LSE and soil-thermal receiver joins are independently reconstructed.
- Successful and rejected candidate construction remain clone-only. Production
  constructors keep `surface_liquid_shadow=None`, and no runner selector,
  production dispatch/default, publication, activation, or production-state
  mutation path was added.

## Commands run at the exact reviewed commit

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
27 passed / 0 skipped

cargo nextest run --profile quick \
  -p openwepp-hillslope-orchestrator -E 'test(/surface_liquid/)'
30 passed / 507 skipped

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets -- -D warnings
PASS
```

The focused gates retain the implemented custody, D/A/F, closure, restart,
receiver-context, rollback, and nonactivation behavior. They do not exercise
the public unified bridge with the unsupported frozen/thawing or
snow-liquid-only production states identified above.

## Approval statement

`NO-GO`: exact commit `9ab4b1f17` is not acceptable for dependency-package
closure. Correct `B-RELEASE-HIGH-001`, rerun the invalidated focused gates, and
obtain a fresh exact-byte hydrology/ownership release review.
