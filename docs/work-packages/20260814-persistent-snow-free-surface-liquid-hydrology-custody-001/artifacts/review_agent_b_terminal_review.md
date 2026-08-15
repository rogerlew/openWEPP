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

---

# Review Agent B Terminal Closure Review — Hydrology, Custody, And Science

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `6c786ca6d07881697d64205fe44b09f12369034b`

Verdict: `PASS / GO / no unresolved material hydrology, custody, science, or evidence finding`.

This fresh terminal review preserves the historical `4f50e494c` HOLD above.
It read the complete version-5 `SC-SURFACELIQUID-001`, the controlling LSE and
WATBAL ownership rules, the package and all retained finding/gate history, the
strict owner and restart implementation, ingress and independent closure, the
shared production WB14 transition, the unified LSE/real-hydrology bridge, the
production state and frozen/snow preflight surfaces, and the focused authority
and integration tests. It inspected the complete public custody path rather
than only the most recent test-governance diff.

No new material finding was identified. The exact executable custody source at
this commit is unchanged from the hydrology closure PASS at `ab703c83a`; the
subsequent executable-diff changes are confined to package-owned tests and the
historical Stage-0 source guard. This review nevertheless re-inspected the
current source and reran the focused custody suites at the exact reviewed
commit.

## Closure assessment

- Persistent custody is exact per `(run, OFE, tile, surface, class, source
  type, source ID)`. Configuration binds production lane/index, ordered soil
  layers, receiving thermal layer, area, tile fraction, capacity, ingress mode,
  and downstream routing. Strict canonical bytes, SHA-256 identity, complete
  key sets, state bounds, and transaction lineage make the state
  restart-representable without an executable default or scalar broadcast.
- One immutable beginning state supplies proportional same-store
  authorization. Complete request identity is retained, nonfinite aggregate
  arithmetic fails closed, the candidate independently re-derives the
  authorization, and exact `0 <= F <= A <= D` validation precedes debit. Only
  finalized use changes the candidate store; unused authorization remains.
- Accepted negative LSE vapor flux produces one positive, typed condensation
  credit. It is credited before the capacity test. Overflow preserves mass,
  temperature, enthalpy, transaction, OFE, tile, surface, and source mapping,
  enters only the post-resource ingress stage, and cannot satisfy the current
  interval's authorization.
- Open raw precipitation and accepted covered-canopy release are mutually
  exclusive through the digest-bound ingress mode. Throughfall, both drainage
  terms, stemflow, condensation overflow, and upstream runon retain typed
  timing and custody. Tile-to-OFE weighting and unequal-area OFE routing are
  each applied exactly once.
- Every OFE advances one persistent 1,800-second WB14 continuation per interval
  through the same production transition used by the unchanged daily wrapper.
  The candidate retains day/index/cumulative carry in restart bytes, replaces
  legacy depression retention with exact-zero legacy capacity in this shadow,
  and routes infiltration/excess without a copied proxy partition.
- The actual production soil candidate is built on a clone using the shared
  same-pass infiltration transition. Independent operands reconstruct every
  ordered layer, aggregate soil liquid including residual water over unfrozen
  depth, the named soil-thermal enthalpy credit, retained LSE tile enthalpy,
  store mass, parcel mass/enthalpy, and routed/outlet joins. No producer residual
  is accepted as closure evidence.
- The public bridge rejects snow runtime/carry, retained snow liquid,
  frost/thaw runtime/carry, and any positive production-layer frozen depth or
  frozen water before authorization or the fixed-cap callback. These are the
  exact typed unsupported branches required by the snow-free authority.
- Receiver and rollback validation carries the first actual offending
  owner/OFE/tile, or the exact expected missing identity. The earlier E011
  substitution and missing-row findings are covered by later-row and
  per-position poisons. The complete bridge owner set is exactly LSE,
  hydrology, and soil thermal.
- Success and every rejected path are candidate-only. The caller's production
  frame remains byte-identical, normal `DirectRunFrame` constructors set
  `surface_liquid_shadow=None`, and recursive source inspection finds no runner
  selector, production dispatch/default, publication, activation, or cutover
  consumer.

## Evidence checked

The retained exact executable-byte comparator rerun at `74d512f44` records:

```text
cargo clippy --workspace --all-targets --all-features -- -D warnings
PASS

TMPDIR=/tmp/ow-nextest-openwepp-20260814-4 \
  cargo nextest run --workspace --profile full
PASS: 2,783/2,783; 33 skipped

cargo test --doc --workspace
PASS

cargo deny check
PASS (one retained non-failing unmatched MIT-0 allowance warning)

cargo fmt --all -- --check
PASS

git diff --check
PASS
```

Commit `6c786ca6d` adds only the retained terminal gate/finding evidence after
the `74d512f44` executable bytes. Attempt 3's 2,782/2,783 failure and its exact
Stage-0 source-scan correction remain preserved alongside the successful
attempt 4 logs; no failed evidence was replaced.

Commands rerun at exact commit `6c786ca6d`:

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 27/27; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 32/32 selected; 507 skipped by filter

git diff --check
PASS before this appended review record
```

The current governed line counts are truthful: the largest new owner module is
2,990 lines and remains below the mandatory 3,000-line refactor threshold; all
2,000-line WARN files have an explicit package disposition.

## Residual boundaries

This verdict approves only the dependency package's persistent snow-free
surface/litter custody and default-off real-owner bridge. It does not authorize
production selection, production-state mutation, runtime activation,
publication, calibration, snow/frozen custody, Child-3 completion, real
consumer cutover, or campaign completion. Prompt archival, two terminal
verifiers, terminal lifecycle reconciliation, and later Child-3 resumption are
separate required steps.

## Approval statement

`GO`: exact commit `6c786ca6d07881697d64205fe44b09f12369034b` is acceptable from the
hydrology, custody, and science-closure perspective. All retained material
findings reviewed here are corrected, focused and full-workspace evidence is
present, and no material finding remains for this review role.
