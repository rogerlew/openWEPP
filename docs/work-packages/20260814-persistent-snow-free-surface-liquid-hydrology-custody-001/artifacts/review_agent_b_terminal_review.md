# Review Agent B Terminal Re-review — Hydrology And Ownership

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `4f50e494cf7757309c94cd1b5cd62bb7cd9c0782`

Verdict: `HOLD / one material E011 expectation-context defect remains / no authority HOLD`.

This review read the complete version-5 `SC-SURFACELIQUID-001`, the package,
all retained Review-A and Review-B artifacts (including both
`review_agent_a_final_pass.md` and `review_agent_b_final_pass.md`), the current
surface-liquid owner, ingress and closure modules, the unified real-hydrology
bridge, the production infiltration transition, and the focused integration
tests.

## Material finding

### B-TERMINAL-HIGH-001 — malformed independent thermal expectations still report the first LSE receiver

The `75ba70681` correction materially fixes the reported final-candidate and
rollback paths, but the earlier independent-expectation guard still combines
four distinct failures into one predicate. In
`validate_receiver_expectations()`
(`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:893-935`),
a wrong second or later entry in `ordered_thermal_layers` makes
`thermal_tiles != expected_tiles`, after which the failure always uses:

- `expectations.lse_owner_id` rather than the soil-thermal owner; and
- `configuration.records.first()` rather than the actual first mismatching
  expected/actual OFE and tile.

`UnifiedReceiverExpectations::try_new()` admits any nonempty, unique typed
OFE/tile vector, so this path is reachable before fixed-cap callback execution.
For a malformed second thermal expectation, the resulting
`SURFACELIQUID-E-011` payload therefore names a valid first LSE receiver instead
of the actual offending soil-thermal expectation. That is the same prohibited
substitution identified by `B-FINAL-PASS-HIGH-001`, and it conflicts with the
version-5 guard rule requiring available owner/OFE/tile identity in the
canonical public failure.

The new two-row poison at
`tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs:1458-1525`
exercises a malformed finalization candidate after the callback. It does not
exercise a malformed second row in the independent
`UnifiedReceiverExpectations` supplied to the public bridge, so the remaining
path is not rejected by an exact-context assertion.

Required correction: split `validate_receiver_expectations()` by failure
family. For an expectation topology mismatch, select the first ordered
mismatch, report `soil_thermal_owner_id`, preserve the actual row when present
or the exact expected row when missing, and retain typed OFE/tile context in
E011. Add a two-row public-bridge poison for a wrong second independent thermal
expectation and assert exact owner/OFE/tile. This is a bounded implementation
defect; it needs no contract amendment, model identity, tolerance, or new
package.

## Confirmed closures

- The final-candidate E011 path no longer substitutes the first configured
  receiver. Duplicate/mismatched later thermal candidates report the actual
  soil-thermal owner and actual OFE/tile; missing final thermal rows use the
  expected typed receiver identity.
- Rollback validation requires exactly three ordered rows:
  `LandSurfaceEnergy`, `Hydrology`, and `SoilThermal`. Wrong or reordered rows
  report the first actual typed owner, a missing terminal row reports the exact
  expected owner, and an extra row reports the actual extra owner. OFE/tile are
  correctly absent because these rollback rows are owner-wide, not tile-wide.
- Independent receiver expectations bind exact LSE and soil-thermal owner IDs,
  beginning digests, tile order, and complete ordered thermal-layer vectors.
  Candidate validation rejects missing, extra, reordered, duplicate, and
  nonfinite thermal state.
- Production-soil closure reconstructs every layer and the production aggregate
  using `theta_m + residual_theta * max(depth_m-frozen_depth_m,0)`. The
  nonzero-residual vector distinguishes the prior incorrect `sum(theta_m)`
  implementation.
- Same-snapshot D/A/F remains sealed and independently re-derived. Exact
  source identity is preserved, authorization is proportional to immutable
  beginning supply, `0 <= F <= A <= D`, and only finalized use is debited.
- Signed condensation is credited before capacity overflow. Overflow retains
  mass/enthalpy identity, enters only post-resource ingress, and cannot satisfy
  same-interval authorization.
- Configuration/state persistence is strict, deterministic, digest-bound, and
  restart-representable. Continuation lineage and one 1800-second WB14 call per
  OFE/interval remain enforced.
- Actual infiltration uses the shared production same-pass transition on a
  cloned lane. Ordered soil-liquid deltas, soil-thermal enthalpy and retained
  LSE enthalpy are independently reconstructed.
- Candidate work remains clone-only and rollback poisons preserve the beginning
  production frame. The bridge uses the exact three owners it actually
  constructs; no Vegetation/BGC placeholder rollback rows remain.
- The state remains default-off (`DirectRunFrame` constructors set
  `surface_liquid_shadow=None`). No runner selector, production dispatch,
  default, publication, activation, or production-state mutation path was
  added.

## Commands run at the exact reviewed commit

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
26 passed / 0 skipped

cargo nextest run --profile quick \
  -p openwepp-hillslope-orchestrator -E 'test(/surface_liquid/)'
30 passed / 507 skipped

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets -- -D warnings
PASS
```

These gates retain the custody, protocol, closure, restart, rollback, and
non-activation behavior. They do not cover the malformed-later independent
expectation path above.

## Approval statement

`NO-GO`: exact commit `4f50e494c` is not acceptable for dependency-package
closure. All constitutive custody, D/A/F, signed-credit, ingress/restart,
three-owner rollback, receiver reconstruction, and production-exclusion claims
reviewed here are materially closed. The one remaining E011 exact-offender
attribution path is in scope and must be corrected and independently re-reviewed
at new exact bytes.
