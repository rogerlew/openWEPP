# Review Agent B Terminal Closure 6 — Hydrology, Custody, And Science

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `bce6fa8300321a76ad4f2db36164eabbc57f5cc2`

Verdict: `PASS / GO / no unresolved material hydrology, custody, or science finding`.

This exact-byte review preserves every earlier failed review and HOLD artifact.
It rechecks the closure5 findings against the complete public candidate
validation path, then traces the unchanged resource, WB14, receiver, restart,
rollback, and production-exclusion boundaries.

## Chronological mixed-enthalpy closure

The remediation now reproduces the admitted chronological rule rather than a
whole-OFE average:

- Frozen local source operands retain the validated input's exact
  `start_s/end_s` support and are sorted by complete `FrozenSourceIdentity`.
  Caller tile-ingress order therefore cannot change source order, floating
  remainder ownership, candidate bytes, or results.
- For every topology OFE, the independent projection collects local sources
  and basis-rekeyed routed inputs, forms the union of their exact support
  boundaries, and processes each window separately.
- Each window independently reconstructs source fractions, `x_p,b`, `q_p,b`,
  `X_b`, and `Q_b`. The checked zero-mass branch requires exact zero enthalpy;
  the positive branch computes exactly one `h_mix,b=Q_b/X_b`.
- Every source's expected post-mix enthalpy is reconstructed as its attributed
  window mass times that one `h_mix,b`, then accumulated by exact
  `(source_parcel_id,basis_ofe_id)` identity.
- Raw window enthalpy is accumulated separately from post-mix attribution.
  Final comparison closes each source's mass and enthalpy, each OFE's
  attributed enthalpy, and raw-to-post-mix OFE enthalpy under the admitted
  dimensional closure rule.

The algorithm handles all three load-bearing support cases:

1. **Disjoint supports:** each nonempty window has only its active source, and
   the empty gap takes the exact zero branch without inventing a receipt or
   enthalpy.
2. **Partly overlapping supports:** pre-overlap, overlap, and post-overlap
   windows receive distinct mixtures; no whole-interval temperature is reused.
3. **Routed supports:** an upstream routed receipt is area-converted once,
   re-keyed to the destination basis OFE with its exact support retained, and
   participates in the destination window mixture alongside local sources.

The new partial-overlap vector exercises unequal source temperatures across
three active windows. Reversed caller ingress order produces identical ending
state, receipts, ledgers, and closure operands. Static tracing confirms that
the same boundary algorithm includes disjoint and routed segments; neither
case has a separate fallback or aggregate-only branch.

## Source identity and validation separation

The complete public validator now has two distinct checks:

- Producer reconstruction regenerates the candidate from immutable
  configuration, resource, and ingress input. Its source-identity comparison
  includes source ID, kind, origin store, basis OFE, and exact support bits, so
  a mutated support fails E009 with the actual tile/source context.
- Independent closure consumes those frozen source operands and reconstructs
  the chronological mass/enthalpy equations without accepting a producer
  residual. Mass and enthalpy changes remain independently detectable even
  when source identity is unchanged.

Exact-zero configured sources remain members of the frozen identity set.
Deletion, duplicate, re-key, kind swap, support mutation, and caller reorder
controls therefore do not alias an absent source to a numerical zero. Aggregate
OFE failures retain owner/OFE identity and typed absence for tile, surface, and
source when no unique finer identity exists.

## Canonical production arithmetic

The remediation does not change the accepted constitutive ingress calculation.
Production continues to:

- partition each OFE at every source boundary;
- call the shared checked WB14 continuation on each exact chronological
  subinterval while retaining one interval-level continuation lineage;
- compute the common `h_mix,b` once from summed subinterval mass and enthalpy;
- apply that same mixture to all attributed infiltration, retained-water,
  runoff, and downstream-runon parcels;
- allocate mass by canonical source order with the final mass remainder; and
- retain source and destination custody through routing.

Repeated floating operations are judged only by the contract's explicit mass
and enthalpy closure envelopes. They do not change identity, branch selection,
water bounds, or source ordering, and no tolerance is used to repair a missing
or duplicate operand.

## Retained resource and receiver custody

- One immutable beginning snapshot supplies the surface-water authorization.
  Transaction, requester, OFE, tile, surface, source, basis, and interval
  identities remain typed and exact.
- `0 <= finalized use <= authorization <= demand` remains exact. Only finalized
  use debits the persistent store; unused authorization remains.
- Signed condensation is credited before capacity overflow. Precipitation,
  canopy release, routed runon, infiltration, retention, and runoff remain
  separately typed; denied canopy demand is not donated to ground demand.
- WB14 remains the extracted production transition with exact zero legacy
  depression capacity. Persistent cumulative supply/infiltration, day/interval
  cadence, topology order, and restart lineage remain part of strict state and
  digest validation.
- Soil-liquid, soil-thermal, and retained-LSE candidates independently receive
  their exact mass/enthalpy operands. Infiltration binds the production lane
  and named thermal layer; retained energy returns to the exact tile; routed
  runoff preserves source, current basis, destination, area conversion, and
  support.
- Candidate construction remains clone-only. Complete owner snapshots,
  attempted hashes, receiver rows, continuations, parcel records, and
  diagnostic state retain byte-identical rollback on every typed failure.
  Restart and deterministic serialization surfaces are unchanged by this
  remediation.

No unresolved constitutive, conservation, owner-custody, independent-ledger,
rollback, or restart omission was found in this package's supported snow-free
surface-liquid domain.

## Production exclusion

The exact diff adds chronological independent-closure reconstruction, exact
source-support comparison, tests, and package evidence. It introduces no
runner selector, production scheduler reachability, default dispatch, output
publication, runtime activation, calibration value, or consumer cutover.
Normal production constructors continue to exclude the shadow path. Snow,
terminal snow, frozen, and thawing branches remain typed unsupported at the
declared boundary.

## Commands run at the exact reviewed commit

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 49/49 selected; 507 skipped by filter

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this review artifact was added
```

## Approval statement

`GO`: exact commit `bce6fa830` closes the closure5 chronological-mixing and
source-support defects without changing the admitted production physics.
Chronological raw and post-mix enthalpy, exact source identity, routed basis and
support, persistent D/A/F custody, WB14 continuation, independent receivers,
rollback/restart, and production exclusion are coherent on the reviewed bytes.
This review approves truthful closure of the surface-liquid hydrology-custody
dependency package within its default-off, snow-free, non-activated scope.
