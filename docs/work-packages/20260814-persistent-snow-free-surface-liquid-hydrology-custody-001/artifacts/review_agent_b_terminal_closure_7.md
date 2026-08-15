# Review Agent B Terminal Closure 7 — Hydrology, Custody, And Science

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `bf7210ea1238ac12adf4aef77416141d7717570e`

Verdict: `HOLD / independent disposition custody remains circular / no new authority package indicated`.

This exact-byte review preserves all earlier reviews and HOLDs. It confirms the
closure7 improvements for window identity, routed support, multi-hop expected
routing, and domain precedence, then audits whether each child disposition and
raw thermodynamic operand is independently authoritative.

## Material findings

### B-TERMINAL-CLOSURE7-CRITICAL-001 — actual nonrunoff receipts choose their own expected disposition and mass

The new `ParcelJoinKey` correctly retains source, basis OFE, exact support bits,
and disposition. That closes an aggregate-preserving cross-window enthalpy
swap. The routed runoff expectation is also no longer copied directly from the
actual routed receipt.

The infiltration-versus-retention partition remains self-referential. For each
independently reconstructed source/window segment,
`project_parcel_arithmetic()` selects the **actual** receipts whose disposition
is either `Infiltration` or `RetainedSurface`
(`surface_liquid_closure.rs:1209-1227`). It sums their actual masses at
`:1228-1241`, then iterates those same actual receipts and inserts each actual
mass and actual `receipt.disposition` into the expected map at `:1268-1298`.
Only the combined residual is independently classified as runoff.

Consequently, the independent validator does not determine how much of the
accepted WB14 infiltration belongs to `Infiltration` or how much post-WB14
excess belongs to `RetainedSurface`. A coordinated poison can:

1. change one infiltration receipt to `RetainedSurface`;
2. replace its recipient with the structurally valid exact surface store;
3. retain the same window, mass, common `h_mix,b`, and enthalpy.

The actual receipt then creates an identically keyed expected retained row.
`validate_receipt_recipient()` accepts the coordinated typed recipient. The
source/window, OFE, and raw-to-mixed totals are unchanged.

The store operands do not independently close this seam. At candidate
construction, `capture_operands()` derives
`retained_excess_kg_m2_ofe_ground` by summing the producer's retained receipts
(`surface_liquid_closure.rs:734-762`). Thus an implementation that emits the
coordinated wrong disposition supplies both the retained operand and the
receipt used to prove it. Soil and retained-LSE receiver candidates likewise
consume those producer dispositions; structural recipient validation is not
an independent proof of the WB14 infiltration/excess partition.

Immutable E009 reconstruction catches a test mutation made after construction,
but it reruns the same producer and cannot detect the producer itself assigning
a source child to the wrong owner. This violates the exact infiltration,
retention, receiver, and independent-ledger obligations in
`INV-SURFACELIQUID-007..008`.

Required correction:

1. Independently reconstruct the per-window total WB14 infiltration from the
   immutable beginning continuation, parameters, and source supply, or freeze
   an authoritative WB14 outcome that is independently joined to the shared
   production transition.
2. Attribute that infiltration to sources using the canonical proportional and
   final-remainder rule; derive post-infiltration source excess by subtraction.
3. Independently derive retained mass from beginning store/capacity and the
   exact tile/source excess; derive runoff as the remainder.
4. Build expected infiltration, retained, routed-runoff, and outlet-runoff rows
   from those values. Use actual receipt values only as the comparison side.
5. Independently join retained receipt totals to the store operand and
   infiltration receipt totals to the production-soil/thermal receiver
   operands.
6. Add coordinated disposition-plus-recipient, equal-total owner-swap, and
   infiltration/retention exchange poisons. A disposition-only poison with a
   mismatched recipient is not sufficient.

### B-TERMINAL-CLOSURE7-HIGH-002 — independently routed segments retain the pre-route kind and can use a different canonical sum order

Production converts every routed child to
`DirectSurfaceLiquidParcelKind::UpstreamRunon` before inserting it into the
destination OFE (`surface_liquid_ingress.rs:1820-1829`). Destination production
ordering is support, origin store, `UpstreamRunon`, then parcel ID
(`:1897-1904`).

The independent route reconstruction preserves `segment.kind` when it creates
the destination `RawParcelSegment`
(`surface_liquid_closure.rs:1384-1398`). Its destination-window sort uses that
kind before parcel ID (`:1140-1153`). These orders differ when several canopy
release kinds from one origin reach the same downstream window: production
sees all as `UpstreamRunon` and falls through to parcel-ID ordering, while the
independent projection orders them by the original throughfall/drainage/
stemflow enum variants.

Because checked floating summation and `h_mix,b` are order-sensitive, the two
paths can produce different mixture bits or different E003 overflow/cancellation
disposition. The new multi-hop vector routes one raw-precipitation source, so it
cannot expose this case.

Required correction: set the independently constructed destination segment's
kind to `UpstreamRunon`, and preferably share one canonical timed-parcel order
key between producer and validator while retaining independent arithmetic.
Add a multi-hop covered-canopy vector with several nonzero release kinds,
unequal temperatures/amounts, unequal OFE areas, and a destination-local
overlap. Include an order-sensitive poison.

### B-TERMINAL-CLOSURE7-HIGH-003 — raw source total enthalpy is not joined to its frozen temperature and specific enthalpy

The new E003 preflight validates source support, mass/Q finiteness, temperature
domain, and `specific_liquid_enthalpy == c_w*(T-T_ref)`
(`surface_liquid_closure.rs:1552-1583`). It does not validate:

```text
source_enthalpy_j_m2 = source_mass_kg_m2 * specific_liquid_enthalpy_j_kg
```

The chronological projection accepts `parcel.enthalpy_j_m2_basis_ofe_ground`
as raw `q_p,b`, calculates `h_mix,b` from it, and proves only that the resulting
output conserves that supplied Q. A producer construction defect that gives a
source Q inconsistent with its otherwise valid temperature/specific-enthalpy
fields can therefore become the independent raw authority and pass every
mixed-output check. Receipt temperature/Q joins do not reconstruct the missing
raw-source equation.

Required correction: add the checked raw mass-times-specific-enthalpy join to
the domain/arithmetic preflight and final independent source validation, with
normal, signed, zero-mass, finite-overflow, finite-underflow, and coordinated
raw-Q/output-Q poisons. Use the canonical named enthalpy function/constants
rather than a second literal formulation.

## Improvements confirmed

- Exact window and disposition identity now prevents aggregate-preserving
  temperature/Q swaps between chronological windows and between distinct
  disposition keys.
- Independent runoff routing derives support, mass, enthalpy, area conversion,
  and downstream segments from the upstream reconstructed window rather than
  treating the actual routed receipt as its own expectation.
- Topology-ordered extension supports multiple hops and preserves destination
  basis identity. Routed mass/Q drift and simple disposition/recipient drift
  fail E010.
- Source and receipt domain preflight now rejects nonfinite, reversed,
  out-of-range support, negative/nonfinite mass, and out-of-range temperature as
  contextual E003 before E009/E010. Rollback hashes remain populated.
- Production `h_mix,b` physics is unchanged: one checked common mixture is
  still used for every source child in an active window. No source-specific or
  whole-OFE temperature regression was introduced.

## Retained hydrology and ownership behavior

- Surface-resource identity and exact `0 <= F <= A <= D` remain intact. Only
  finalized use debits the persistent store; unused authorization remains.
- Signed condensation remains a pre-ingress credit with capacity overflow.
- The shared stateful WB14 continuation, strict 1800-second/48-step cadence,
  zero legacy depression capacity, OFE topology, and restart lineage are
  unchanged.
- Configuration/state digests, candidate isolation, typed failures, complete
  rollback hashes, deterministic restart, and production-byte invariance remain
  present.
- No runner selector, production scheduler path, default dispatch, output
  publication, activation, calibration, or cutover was introduced.

These retained properties do not close the independent disposition and raw-Q
findings above.

## Commands run at the exact reviewed commit

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 51/51 selected; 507 skipped by filter

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this review artifact was added
```

The passing vectors prove cross-window h_mix enforcement, a one-source
multi-hop route, and domain precedence. They do not include a coordinated
infiltration/retention owner swap, multiple routed source kinds with
order-sensitive arithmetic, or a raw source Q inconsistent with its own
mass/temperature fields.

## Approval statement

`NO-GO`: exact commit `bf7210ea1` closes the closure6 window-key, routed-receipt
self-reference, and E003 domain-precedence findings for the exercised cases.
Dependency closure remains blocked because actual nonrunoff receipts still
select their own expected owner disposition and mass, independently routed
segments retain the wrong destination kind/order, and raw source Q is not
joined to its frozen thermodynamic fields. Existing
`SC-SURFACELIQUID-001` authority is sufficient to correct all three defects;
no new package or model identity is indicated.
