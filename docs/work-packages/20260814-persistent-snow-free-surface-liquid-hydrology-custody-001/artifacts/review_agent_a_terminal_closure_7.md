# Review Agent A — Terminal Closure 7 Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `bf7210ea1238ac12adf4aef77416141d7717570e`

Verdict: `HOLD / NO-GO`.

## Findings

### Critical — Expected local disposition and routed residual still consume actual receipts

The closure-7 join key now retains source parcel, basis OFE, exact start/end
bits, and disposition
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs:407-413`).
Actual receipts are accumulated on that exact key at `:1051-1080`, while the
chronological projection independently constructs raw window mass, enthalpy,
and `h_mix,b` from frozen segments at `:1083-1207`. Those are material
improvements over closure 6.

The disposition projection remains circular. For every frozen source/window,
`project_parcel_arithmetic()` selects the **actual** infiltration and retained
receipts at `:1209-1227`, sums their actual masses at `:1228-1241`, and defines
runoff as the frozen segment mass minus that actual nonrouted mass at
`:1242-1260`. It then copies each selected receipt's actual mass and actual
disposition into the expected map at `:1268-1299`. Routed and outlet
expectations at `:1300-1399` are therefore independently timed and routed but
still inherit their upstream quantity from the producer's actual
infiltration/retention partition.

This admits a coherent owner-swap counterexample. Change an infiltration
receipt to `RetainedSurface`, replace its recipient with a structurally valid
`SurfaceStore`, and leave its source, basis, exact window, mass, and common
mixture enthalpy unchanged. The same changed receipt creates the matching
expected retained row. `validate_receipt_recipient()` accepts the coordinated
typed recipient, and all raw/post-mix and OFE transaction totals remain equal.
The reverse exchange is equally circular. A coordinated mass exchange between
nonrouted custody and routed runoff can likewise move through a downstream
topology while the residual calculation follows the changed actual partition.

The store operand is not an independent backstop. Candidate construction sums
actual retained receipts into
`retained_excess_kg_m2_ofe_ground` at `surface_liquid_closure.rs:734-762`; the
later store check does not independently join the exact retained children back
to a WB14-derived retention result. Immutable E009 reconstruction detects a
post-construction test mutation, but it reruns the same producer and cannot
detect the producer itself emitting the coordinated wrong partition. The new
disposition poison changes the enum without supplying a coherent matching
owner, so structural recipient validation catches it before it exercises this
seam.

Required correction:

1. Independently reconstruct each window's WB14 infiltration from immutable
   beginning continuation, parameters, source supply, and the shared
   authoritative transition.
2. Attribute infiltration to frozen sources using the canonical proportional
   and final-remainder rule, then derive per-source excess, exact-store
   retention, and runoff without reading actual receipts.
3. Construct all expected infiltration, retention, routed-runoff, and
   outlet-runoff rows solely from those results and topology. Actual receipts
   must occur only on the comparison side.
4. Join independently derived infiltration and retention totals to their exact
   soil/thermal receiver and store operands.
5. Add coherent disposition-plus-recipient swaps and equal-total mass exchanges
   across infiltration, retention, and routed runoff, including downstream
   continuation.

This violates the independent-ledger and exact-custody obligations in
`INV-SURFACELIQUID-007..008` and is closure-blocking.

### High — Exact current/recipient tile identity is absent from the independent join

`ParcelJoinKey` contains source, basis, support, and disposition but no current
or recipient store key (`surface_liquid_closure.rs:407-413`).
`RawParcelSegment` carries only the origin store (`:445-453`), so the
independent topology traversal never records the exact current tile on which a
local or routed child must be consumed.

Recipient validation does not close that omission. Infiltration validates OFE
lane metadata but not `receipt.recipient_store_key`; retention checks only that
the nested `SurfaceStore.store_key` equals that same receipt field; outlet
validates only the OFE; and routed runoff validates the next destination but
not the receipt's current store (`:1797-1855`). For context construction,
`projection_key_store()` can fall back to an actual receipt's recipient store
at `:543-546`, so even diagnostic context is not always derived solely from
frozen identity and topology.

In a multi-tile OFE, a producer can retarget a retained receipt's
`recipient_store_key` and nested `SurfaceStore` to another configured tile
while preserving source, origin, basis, window, disposition, mass, and
enthalpy. The independent join key is unchanged, structural recipient
validation succeeds, and the store operand remains producer-derived rather
than independently joined to that child. E009 detects a later candidate
mutation, but not this construction defect.

Required correction: carry the topology-derived current/recipient store key in
the independent segment identity or compare it as an equally exact identity
dimension; derive routed current-store identity at every hop; join retained
mass to the exact store operand; and remove actual-receipt fallback from
projection context. Add cross-tile poisons for infiltration, retention,
routing, and outlet receipts in a multi-tile OFE. This is required by
`INV-SURFACELIQUID-007`'s exact tile/source custody and no-cross-tile-
redistribution rule.

### High — Independent routed segments retain the pre-route kind and can reorder mixture arithmetic

Production converts every routed pending parcel to
`DirectSurfaceLiquidParcelKind::UpstreamRunon` at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs:1820-1829`.
Production then orders destination parcels by support, origin store, kind, and
parcel ID at `:1897-1904`.

The independent destination segment instead retains `segment.kind` at
`surface_liquid_closure.rs:1384-1398`, and the independent contribution sort
uses that kind before parcel ID at `:1140-1153`. If multiple canopy release
kinds from one origin reach the same destination window, production sees only
`UpstreamRunon` and falls through to parcel-ID order, while validation orders
by the pre-route kind. Checked floating summation, mixture bits, overflow
disposition, and final remainder ownership can consequently diverge. The new
multi-hop test routes only one raw source and cannot expose this condition.

The actual receipt kind is also absent from `ParcelJoinKey`; the closure does
not independently prove the canonical kind at each hop. Required correction:
set independently routed destination segments to `UpstreamRunon`, validate the
actual canonical kind for the receipt's basis, and centralize one canonical
timed-parcel ordering key for production and projection while keeping the
arithmetic reconstruction independent. Add a multi-hop, unequal-area,
destination-overlap vector with several nonzero canopy release kinds,
different temperatures and amounts, and deliberately order-sensitive parcel
IDs.

This mirrored but divergent ordering logic is a high-severity duplication
because it can silently change science arithmetic.

### High — Frozen raw total enthalpy is not joined to mass and specific enthalpy

Closure operands now freeze source mass, temperature, specific liquid
enthalpy, and total enthalpy. The E003 preflight validates support, nonnegative
finite mass, finite total enthalpy, the `200..=350 K` domain, and exact
`4_218 * (T - 273.15)` specific enthalpy at
`surface_liquid_closure.rs:1555-1584`. It does not require:

```text
source_enthalpy_j_m2 = source_mass_kg_m2 * specific_liquid_enthalpy_j_kg
```

The chronological projection accepts the frozen total enthalpy directly at
`:1041-1049` and proves conservation only relative to that value.
`FrozenSourceIdentity` excludes mass, temperature, specific enthalpy, and total
enthalpy (`:416-435`), so immutable producer identity reconstruction does not
restore this missing thermodynamic join.

A direct poison can change frozen temperature and specific enthalpy
coherently, retain the original mass and total enthalpy, and pass preflight,
producer identity, and the projection because the raw Q authority is
unchanged. More generally, a producer construction defect can publish an
internally inconsistent raw source Q that becomes the validator's authority.

Required correction: add a checked mass-times-specific-enthalpy equality to
the E003 arithmetic/domain preflight and the independent source join. Cover
positive, signed, zero-mass, finite-overflow, finite-underflow, and coordinated
raw-Q/output-Q poisons with precedence and rollback assertions. Use the
canonical named enthalpy function and constants rather than duplicating the
`4_218.0` and `273.15` literals. The duplicated formulation is a science-drift
risk and should be centralized.

### Medium — The newly 2,003-line closure module lacks required follow-on split intent

`surface_liquid_closure.rs` is now 2,003 lines. That is below the mandatory
3,000-line stop but newly crosses the repository's 2,000-line WARN threshold.
The package's `artifacts/line-count-governance.md` records only
"chronological window-keyed raw/post-mix and independently routed projection;
below the mandatory threshold." It provides no explicit follow-on split
intent, although `crates/AGENTS.md` requires both decomposition rationale and
follow-on split intent at or above 2,000 lines.

Record an owner and follow-on to split the monolithic
`project_parcel_arithmetic()` into typed frozen-input expected projection and
actual-receipt projection components. That boundary would also make the
critical circular dependency structurally harder to reintroduce. No reviewed
file reaches 3,000 lines.

## Closed Prior Findings And Retained Correctness

- The exact join key now retains source, basis OFE, start/end bits, and
  disposition. The aggregate-preserving cross-window temperature/enthalpy swap
  tested in closure 7 fails.
- Routed expected support, mass, enthalpy, and area conversion are no longer
  copied directly from a routed receipt. Topology-ordered reconstruction
  supports multiple hops and applies the checked source/destination area ratio
  once. Simple routed mass/Q and support drift are detected.
- The E003 preflight now exhaustively checks frozen-source and receipt support
  for finiteness, ordering, and `[0,1800]` bounds; mass for finiteness and
  nonnegativity; enthalpy for finiteness; and temperature for finiteness and
  the admitted `200..=350 K` domain before E009/E010 comparison. The new
  malformed-domain tests return contextual E003 and retain rollback hashes.
  The missing raw mass-times-specific-Q equation is the separate finding
  above.
- Local raw contribution ordering matches the production field comparator.
  Only the routed-kind transition remains divergent.
- D/A/F authority, signed condensation, stateful WB14 continuation, exact-zero
  participation, destination area conversion, store projection, receiver
  reconstruction, restart lineage, rollback behavior, snow/frost exclusion,
  and default-off selection are unchanged by closure 7.
- The reviewed code delta does not change production `h_mix,b` arithmetic.
  `surface_liquid_ingress.rs` is unchanged in the closure-7 implementation
  commit; production still computes one checked mixture per active window and
  applies it to every child.
- All earlier failed reviews and HOLD artifacts remain preserved. This review
  adds only the named Agent A closure-7 artifact and does not modify the
  concurrent Agent B artifact.

Current affected line counts are 2,347 for `surface_liquid_owner.rs`, 876 for
`surface_liquid_owner_tests.rs`, 1,941 for `surface_liquid_ingress.rs`, 1,858
for `surface_liquid_ingress_tests.rs`, 2,003 for
`surface_liquid_closure.rs`, 2,881 for `land_surface_energy_shadow/mod.rs`,
2,852 for `direct_runtime/runoff.rs`, 303 for `surface_liquid_wb14.rs`, 2,783
for `00_core_frames.rs`, and 2,157 for
`vegetation_real_hydrology_shadow.rs`. The closure-module WARN is governed by
the finding above; no mandatory line-count stop is reached.

## Exact-Commit Validation

Ran against exact commit
`bf7210ea1238ac12adf4aef77416141d7717570e`; the tracked checkout was clean
before the two terminal review artifacts appeared:

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator --profile quick
PASS: 558/558; 0 skipped; three known slow routing-oracle tests completed

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this Agent A artifact was added

git diff --check bce6fa830..bf7210ea
PASS
```

The passing tests prove exact cross-window matching, simple disposition drift,
one-source multi-hop routing, malformed-domain E003 precedence, and rollback
for the exercised mutations. They do not prove independent WB14 disposition
partitioning, exact current-tile custody, routed-kind canonical order, or the
raw source mass/specific/total enthalpy equation.

The last full-workspace, doctest, dependency-policy, AUTH11/anti-evasion, and
science-admission campaign evidence predates this comparator-sensitive
increment. It was not repeated after material static defects were established;
passing campaign gates could not make these reviewed bytes releasable.

## Residual Risk And Missing Tests

- Add independently enumerated expected rows for every source/window/
  disposition/current-store tuple, including zero results and exact final
  remainder ownership; do not derive test expectations from candidate
  receipts.
- Add coherent infiltration/retention recipient swaps, equal-total owner
  exchanges, and coordinated nonrouted/runoff mass shifts through an
  unequal-area downstream hop.
- Add multi-tile cross-store poisons for every disposition and verify exact
  store, OFE, lane, route, and outlet context.
- Add several canopy release kinds from the same origin to a multi-hop
  destination-local overlap, with temperatures and IDs selected to distinguish
  kind order from parcel-ID order.
- Add raw mass/specific/total enthalpy consistency vectors, including signed
  enthalpy, exact zero, checked overflow/underflow, later E009/E010 faults, and
  rollback hashes.
- After correction, rerun exact-head full-workspace Nextest, strict workspace
  Clippy, doctests, dependency policy, AUTH11/anti-evasion, science admission,
  formatting, diff hygiene, and package Markdown lint.

## Approval Statement

`NO-GO`: commit `bf7210ea1` closes the closure-6 exact-window, direct routed-
receipt, and E003-domain findings for the exercised paths while leaving
production `h_mix,b` unchanged. Dependency closure remains blocked because
actual nonrouted receipts still choose their own expected disposition and mass,
exact current/recipient tile identity is not independently bound, routed
segments retain the wrong pre-route kind/order, and frozen raw total enthalpy
is not joined to its own mass and specific enthalpy. Existing
`SC-SURFACELIQUID-001` authority is sufficient to correct these defects; no
new package or model identity is indicated.
