# Review Agent B Terminal Closure 8 — Hydrology, Science, And Ownership

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `298acedbb47455d5ce54ec0bac2b7382955b11ee`

Verdict: `HOLD / independent replay is not joined to persistent ending state and continuation / no new authority package indicated`.

This fresh review preserves all historical reviews and findings. It traces the
current exact bytes from immutable surface-resource state and frozen ingress
through the independent chronological WB14 replay, exact child receipts,
routed descendants, receiving owners, and restart state.

## Material finding

### B-TERMINAL-CLOSURE8-CRITICAL-001 — independently reconstructed final store and WB14 continuation are discarded before the persistent-state join

The new expected-side projection is materially correct through the physical
partition. It starts from frozen beginning continuation and WB14 parameters,
replays the shared transition for each active chronological window, advances
`cumulative_supply_m` and `cumulative_infiltration_m`, independently derives
infiltration, retention and runoff, and advances a local `store_liquid` map.

Those authoritative ending values are not retained in
`ParcelArithmeticProjection` or compared with the persisted ending owner:

- `cumulative_supply_m` and `cumulative_infiltration_m` are initialized at
  `surface_liquid_closure.rs:1204-1205` and advanced at `:1300-1301`, but leave
  scope at the end of the OFE loop without comparison.
- `store_liquid` is independently advanced through every retained child at
  `:1634-1647`, but its final values are likewise discarded.
- `ParcelArithmeticProjection` returned at `:1687-1696` contains only receipt
  and enthalpy maps. It has no expected ending-store or continuation surface.

The separate store validator is not an independent substitute. Its
`retained_excess_kg_m2_ofe_ground` operand is captured by summing producer
receipts at `surface_liquid_closure.rs:727-755`; `validate_store_equations()`
then reconstructs W1 from that producer-derived retained scalar. It never joins
the final `store_liquid` calculated by the receipt-free replay. The closure
validator also receives no ending continuation operand and performs no
continuation comparison.

Public E009 reconstruction compares the complete producer ending state against
another execution of the same producer. That detects post-construction test
mutation, but it cannot independently detect an implementation defect that
updates the persisted store or continuation incorrectly while producing the
otherwise correct independently validated receipts.

This leaves two contract obligations unproved:

```text
W1 = independently replayed final tile store

ending cumulative supply/infiltration
    = independently replayed WB14 continuation
```

It also leaves the strict ending-state/restart digest dependent on same-producer
reconstruction rather than the independent replay required by Section 8 and
`INV-SURFACELIQUID-002,008`.

Required correction:

1. Extend the independent projection with exact expected ending store values
   and expected ending WB14 continuation values for every OFE.
2. Compare the replayed store directly with the actual ending owner record;
   do not use producer-captured retained receipts as the expected W1 operand.
3. Compare replayed cumulative supply and infiltration with the actual ending
   continuation, and independently derive/validate day index, next interval,
   accepted transaction identity, cardinality and OFE identity.
4. Recompute or validate the complete ending state digest only after those
   joins pass.
5. Add coordinated poisons for:
   - wrong retained ending store with correct receipts;
   - wrong cumulative supply;
   - wrong cumulative infiltration;
   - wrong day/interval rollover;
   - wrong continuation transaction identity;
   - missing, duplicate, reordered and wrong-OFE continuation rows;
   - a forged self-consistent store closure operand and ending store.

The existing `SC-SURFACELIQUID-001` equations and state schema are sufficient;
this is an in-package independent-owner validation omission.

## Closure7 findings confirmed corrected

- The expected projection has zero access to actual receipts. It independently
  replays WB14 from frozen sources, beginning continuation, parameters,
  pre-ingress stores and capacities.
- Expected infiltration, retention, routed runoff and outlet runoff are derived
  from the replay. Actual receipt mass, disposition or recipient cannot define
  its own expected row.
- `ParcelJoinKey` binds owner, source ID, origin store, current/recipient store,
  complete typed recipient, basis OFE, kind, exact support and disposition.
  Coordinated infiltration/retention owner swaps and cross-tile retargeting fail
  E010.
- Routed descendants are independently area-converted, use the destination
  current store, and become canonical `UpstreamRunon` before downstream sorting
  and mixing. Multi-hop support and source lineage remain exact.
- Frozen raw sources now require the exact checked
  `Q = mass * specific_liquid_enthalpy` relation in addition to the temperature/
  specific-enthalpy relation. The invalid raw-Q branch fails E003 before
  mixing.
- Source and receipt domain precedence remains contextual E003 for nonfinite,
  negative, reversed, or out-of-domain support, mass and temperature operands.

No new material child-receipt, canonical `h_mix,b`, multi-hop routing, or raw
enthalpy defect was found.

## Retained hydrology and receiving-owner behavior

- Exact typed `0 <= F <= A <= D` remains enforced from one immutable beginning
  snapshot. Finalized use alone debits persistent surface water and unused
  authorization remains.
- Signed condensation remains a pre-ingress credit with exact capacity overflow
  and enthalpy identity.
- Open precipitation and covered-canopy release remain mutually exclusive;
  timed ingress, retention, runoff and unequal-area routed runon preserve
  source/current/destination custody.
- Production still uses the shared stateful WB14 transition and one canonical
  `h_mix,b` for every child in each active window. This remediation changes
  validation and frozen operands, not accepted constitutive physics.
- Soil-liquid, named soil-thermal and retained-LSE receipts retain their exact
  production lane, layer, tile, mass and enthalpy identities. Coordinated wrong
  recipients fail the complete independent receipt comparison.
- Candidate construction remains clone-only, and focused failure vectors retain
  beginning/attempted hashes and byte-identical rollback. The finding concerns
  independent proof of the successful ending/restart state, not evidence of a
  partial mutation.
- Snow, terminal snow, frozen and thawing branches remain typed unsupported in
  this declared snow-free package domain.

## Production exclusion

The reviewed diff changes the independent closure, frozen partition operands,
tests and package evidence. It introduces no runner selector, production
scheduler reachability, default dispatch, output publication, runtime
activation, calibration value or consumer cutover. Normal production behavior
remains unchanged and the shadow still requires explicit invocation.

## Commands run at the exact reviewed commit

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 52/52 selected; 507 skipped by filter

cargo nextest run -p openwepp-hillslope-orchestrator --profile quick
RAN: 559 tests started; the three retained long routing-oracle tests exceeded
the 60-second slow threshold; the command completed without a reported test
failure in the captured session.

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this review artifact was added
```

The focused gates prove receipt-free partitioning and exact child identities.
They contain no poison against the discarded independently replayed ending
store or continuation.

## Approval statement

`NO-GO`: exact commit `298acedbb` closes all accepted closure7 receipt,
recipient, routing-kind, raw-Q and domain findings. Dependency closure remains
blocked because the receipt-free replay's final surface store and WB14
continuation are not compared with the persistent ending owner or its restart
digest. Join those replayed values to the ending state and rerun focused
restart, rollback, receiver and exact-byte terminal gates before closure.
