# Review Agent B Final Pass — Hydrology And Ownership

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `848e60358cfc98d2de6a6dcdc6c45779a3255228`

Verdict: `HOLD / one material contextual E011 defect remains / no authority HOLD`.

This review read the complete version-5 `SC-SURFACELIQUID-001`, the package,
all retained Review-A and Review-B artifacts, the current surface-liquid owner,
ingress and independent-closure modules, the unified real-hydrology bridge,
the production infiltration transition, and the focused integration tests.

## Material finding

### B-FINAL-PASS-HIGH-001 — E011 still identifies a convenient first receiver instead of the actual offending receiver or owner

The final remediation closes the numeric and cardinality portions of the prior
receiver-envelope findings, but it does not close the canonical identity
payload for `SURFACELIQUID-E-011`.

`UnifiedLseFinalization::try_new()` combines duplicate LSE tiles, duplicate
thermal tiles, tile-set mismatch, duplicate thermal layers, duplicate rollback
identity, and changed rollback hash into one predicate
(`land_surface_energy_shadow/mod.rs:280-310`). On failure it always reports the
first LSE tile and the hydrology owner (`:311-318`). A duplicate or malformed
later tile, a thermal receiver defect, or a bad LSE/soil-thermal rollback row
therefore reports a valid first tile and the wrong owner even though the
offending row is available.

The later exact validators have the same loss of information. Receiver-set and
rollback validators return context-free `Identity` errors
(`land_surface_energy_shadow/mod.rs:1069-1202`), and
`apply_ingress_to_real_receivers()` maps either error to a closure that always
uses `configuration.records.first()` and the hydrology owner
(`:1006-1040`). Thus a defect in a second tile or in the independently named
LSE or soil-thermal owner is rejected, but its E011 payload identifies a
different receiver/owner.

This is the still-open part of `B-FINAL-HIGH-002` and
`A-FINAL-HIGH-001`'s contextual-error requirement. The contract requires the
OFE/tile/owner known at the guard site, reserving typed absence for genuinely
unavailable identity (`SC-SURFACELIQUID-001`, Branch And Guard Table). A
canonical error may not substitute a convenient valid row for the offending
one.

Required correction: have the receiver-set and rollback validators return the
first canonical offending row's typed context, and preserve it through E011.
For a rollback mismatch, report the offending owner kind/ID (or an explicitly
typed expected/actual owner context); for a tile or thermal-layer mismatch,
report that row's OFE/tile. Add at least one two-tile poison where the second
tile is wrong and one wrong LSE/soil-thermal rollback-owner poison that asserts
the exact E011 context, not only its code and rollback hashes.

No constitutive equation, tolerance, model identity, or new package is needed.

## Closed findings and confirmed behavior

- `B-FINAL-HIGH-001` is otherwise closed. `UnifiedReceiverExpectations` binds
  independent LSE and soil-thermal owner IDs/digests and exact ordered thermal
  layer vectors. Candidate validation rejects extra, reordered, missing, and
  nonfinite thermal state. The Child-3 bridge now requires exactly three
  ordered rollback rows—LSE, hydrology, and soil thermal—with exact independent
  owner IDs and beginning digests.
- The production-soil aggregate defect is closed. Frozen operands include
  `residual_theta`, layer depth, and frozen depth, and independent closure uses
  `theta_m + residual_theta * max(depth-frozen_depth,0)` for beginning and
  ending aggregates. A nonzero-residual focused vector distinguishes the old
  incorrect sum.
- E004 and E007 now locate the first offending production lane and preserve its
  owner, OFE, and configured surface identity. Snow and legacy depression
  custody remain rejected before candidate construction.
- Same-snapshot D/A/F is sealed and independently re-derived. Authorizations
  are proportional by exact surface source, and only finalized use is debited;
  `0 <= F <= A <= D` remains exact.
- Signed condensation is credited before the capacity check. Overflow is
  emitted as a timed mass/enthalpy parcel and cannot satisfy the same interval's
  authorization.
- Current raw precipitation, canopy release, runon, and condensation overflow
  enter only the post-resource ingress phase. One stateful production WB14
  continuation runs per OFE/1800-second interval, and exact tile/source
  retention plus routed/outlet receipts remain independently closed.
- Actual infiltration uses the shared production same-pass transition on a
  cloned exact production lane. LSE retained enthalpy and soil-thermal
  infiltration enthalpy have typed candidates and independent ending
  equations.
- The implementation remains default-off and clone-only. No runner selector,
  production dispatch/default, publication, activation, or production-state
  mutation path was added.

## Commands run at the exact reviewed commit

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
25 passed / 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
30 passed / 507 skipped

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets -- -D warnings
PASS
```

These passing gates establish the retained custody, closure, and rollback
behavior. They do not contain a multi-tile exact-offender E011 assertion and
therefore do not close the finding above.

## Approval statement

`NO-GO`: exact commit `848e60358` is not yet acceptable for dependency-package
closure. Correct the bounded E011 context defect, rerun the invalidated focused
tests, and obtain a fresh exact-byte hydrology/ownership review.
