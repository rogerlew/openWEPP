# Review Agent B Terminal Closure 10 — Hydrology, Science, And Ownership

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `fd8633865df289620aa5b9cf8c4e1bd206432f30`

Verdict: `HOLD / accepted ordering-definition and mixed-route fixture obligations remain incomplete / physical custody endpoint otherwise passes`.

This fresh review preserves every earlier finding and failed review as immutable
history. It re-audits the exact clean commit above from persistent beginning
state through D/A/F, signed condensation, chronological ingress, independent
partitioning, receiving-owner joins, persistent ending state and restart.

## Material finding

### B-TERMINAL-CLOSURE10-MEDIUM-001 — canonical parcel identity/order remains duplicated and the new mixed-route test does not freeze the required exact outputs

The accepted `A-TERMINAL-CLOSURE8-MEDIUM-003` required the non-arithmetic
canonical source identity and ordering definitions to be centralized while
retaining separate production and independent allocation arithmetic. The
remediation correctly shares water density, liquid heat capacity and reference
temperature, but the remaining required definitions are still transcribed
separately:

- production orders `TimedParcel` through `parcel_order()` at
  `surface_liquid_ingress.rs:1909-1915` and uses it at `:1189`, `:1292` and
  `:1588`;
- independent replay repeats the five-field comparator at
  `surface_liquid_closure.rs:1332-1340` and `:1506-1514`; and
- the canonical local source ID format is repeated in production at
  `surface_liquid_ingress.rs:1145-1148` and in closure capture/identity
  reconstruction at `surface_liquid_closure.rs:978-982` and `:1066-1070`.

These are identity and sequence definitions, not independently derived
physical arithmetic. A future edit can change both the contribution summation
order and the resulting `h_mix,b` bits, or change the receipt/source join key,
without one compiler-visible canonical definition governing both paths. The
package finding disposition says this accepted risk is remediated, but the
requested centralization is absent.

The new mixed-kind vector is nondegenerate: it combines canopy throughfall and
initial drainage of unequal temperatures and amounts, routes across unequal
OFE areas, overlaps downstream local rain, validates canonical
`UpstreamRunon`, and proves caller-order invariance. It does not, however,
freeze the exact expected lower-window `h_mix,b` bits, per-source proportional
infiltration/retention/runoff amounts, enthalpy attribution, or final-remainder
owner. Its only mixture-specific assertion is that at least two distinct lower
receipt temperatures exist. `candidate.validate()` supplies valuable dynamic
independent reconstruction, but it is not the explicit fixed expected vector
required by the accepted finding and cannot replace a regression value for the
shared canonical order seam.

Required correction:

1. Define one dependency-neutral canonical parcel identity/order key used by
   both production and closure DTOs. Keep production allocation and receipt-free
   expected allocation as separate implementations.
2. Use one canonical constructor for local and condensation source IDs; receipt
   IDs may remain separately derived from the canonical source ID and exact
   window/disposition identity.
3. Extend the mixed-kind unequal-area/downstream-overlap vector with fixed
   expected bits or exact admitted comparison rules for each chronological
   `h_mix,b`, every source's mass and enthalpy attribution, routed conversion,
   final remainder and receiving owner.
4. Retain the existing caller-reorder, routed-kind, multi-hop and rollback
   poisons and rerun the focused gates.

No authority or constitutive-model amendment is indicated. This is a bounded
in-package identity/evidence correction.

## Closure 8 critical endpoint finding

`B-TERMINAL-CLOSURE8-CRITICAL-001` remains completely corrected.

The receipt-free chronological projection retains final store liquid and final
WB14 cumulative supply/infiltration for every configured key/OFE. It directly
joins those values to persistent ending records and continuations with exact
record/cardinality/order/OFE identity and accepted transaction lineage. Only
after those comparisons pass does it recompute the complete state digest and
invoke strict state validation. A coordinated forged retained/ending producer
operand cannot define the expected endpoint.

The wrong-store, cumulative-supply, cumulative-infiltration, cadence,
transaction-lineage, missing/duplicate/reordered/wrong-OFE and digest poisons
remain present. Public candidate validation also reconstructs from immutable
configuration/resource/ingress input before the independent closure boundary.

## Closure 8 Rust taxonomy and arithmetic finding

`A-TERMINAL-CLOSURE8-HIGH-002` is corrected.

- Numeric/domain preflight exhaustively precedes producer and closure
  comparison and returns contextual `E003`.
- Both beginning-continuation cross-field bounds are explicit:
  cumulative infiltration may exceed neither cumulative supply nor the
  configured infiltration-storage capacity. The checks occur before the
  zero-supply replay branch.
- Partition membership/order is handled separately as `E009`. Missing,
  duplicate, replacement and reorder cases use membership-aware context rather
  than a shifted positional row.
- Combined poisons prove `E003 > E009 > E010` precedence and retain exact
  beginning/attempted rollback hashes.

The named water-density, heat-capacity and reference-temperature constants are
shared between production and closure. No raw duplicate of those physical
constants remains in the reviewed closure arithmetic.

## Complete custody re-audit

No additional material defect was found:

- Strict per-OFE/tile/surface/source persistent state, deterministic restart
  bytes, configuration/state digests and predecessor lineage remain enforced.
- One immutable snapshot supplies typed requests and proportional maximum
  authorizations. Exact `0 <= F <= A <= D` is independently reconstructed,
  finalized use alone debits storage, and unused authorization remains.
- Signed condensation credits the exact store before ingress and routes
  capacity overflow with exact mass, temperature and enthalpy identity.
- Open precipitation and covered canopy release remain mutually exclusive.
  Chronological support, canonical source order and one shared stateful WB14
  transition per OFE remain the accepted implementation.
- Expected infiltration, retention and runoff have zero access to actual
  receipts. Owner, source, origin/current store, typed recipient, basis OFE,
  kind, support and disposition all enter the join.
- Routed descendants become `UpstreamRunon`, retain source/origin lineage, use
  destination store and OFE identity, and apply unequal-area mass/energy
  conversion exactly once. The existing multi-hop test remains passing.
- Canonical per-window `h_mix,b`, raw mass/specific-enthalpy/Q identity,
  per-source and OFE aggregate closure, soil-liquid, soil-thermal and
  retained-LSE receivers remain independently reconstructed with checked
  arithmetic.
- Candidate construction is clone-only. Canonical rollback hashes and
  byte-identical beginning state remain covered across producer, closure,
  receiver and unsupported-domain failures.
- Snow, terminal snow, frozen and thawing states remain typed unsupported in
  this snow-free bridge.

## Production and campaign boundaries

The reviewed changes add no runner selector, production scheduler path,
default dispatch, output publication, runtime activation, calibration value or
consumer cutover. Production execution remains unchanged and the custody bridge
remains explicitly default-off. The physical dependency-lift endpoint is not a
claim that held LSE Child 3 or the parent campaign is complete.

`surface_liquid_closure.rs` is 2,532 lines and remains below the mandatory
3,000-line split threshold. The package retains an explicit future split for
the WARN-sized closure module.

## Commands run at the exact reviewed commit

```text
git rev-parse HEAD
PASS: fd8633865df289620aa5b9cf8c4e1bd206432f30

git status --short --branch
PASS: clean main; 69 commits ahead of origin/main

cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 55/55 selected; 507 skipped by the focused filter

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this review artifact was added
```

## Approval statement

`NO-GO`: commit `fd8633865df289620aa5b9cf8c4e1bd206432f30`
retains the complete persistent endpoint join and corrects the closure8
taxonomy, cumulative-bound and shared-physical-constant defects. The accepted
ordering/identity-definition and exact mixed-route fixture obligations remain
incomplete. Centralize those canonical non-arithmetic definitions and bind the
fixed mixed-route outputs before terminal verification or custody-lift
disposition.
