# Review Agent B Terminal Closure 11 — Hydrology, Science, And Ownership

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `f375349e38c134eb6eb105d03a87d85841ba8c26`

Verdict: `PASS / canonical identity and bit-frozen mixed-route evidence complete / no unresolved material hydrology, science, or ownership finding`.

This fresh review preserves all previous findings and failed reviews as
historical evidence. It re-audits the exact clean commit above from immutable
surface-resource state through D/A/F, condensation, chronological WB14
partitioning, receiving-owner joins, persistent ending state and restart.

## Closure 10 finding

`B-TERMINAL-CLOSURE10-MEDIUM-001` is completely corrected.

### Shared identity and ordering

`canonical_surface_liquid_source_id()` is now the single constructor for local
and condensation source identities. Production parcel construction, frozen
closure capture and frozen-source identity reconstruction all call that helper.
No second source-ID format string remains in those paths.

`canonical_parcel_order()` accepts a typed `CanonicalParcelOrderKey` containing
the exact canonical fields:

```text
start support
end support
origin store key
parcel kind
source parcel ID
```

Production `TimedParcel`, frozen-source identity and independent projected
segments adapt their distinct DTOs to that one ordering key. Production and
expected contribution sequences therefore cannot drift through separate field
lists or precedence.

The sharing is limited to identity and order. Production allocation remains in
`surface_liquid_ingress.rs`; the expected side still independently constructs
chronological windows, invokes the shared admitted WB14 transition, allocates
infiltration, retention and runoff, routes descendants and reconstructs ending
stores and continuations in `surface_liquid_closure.rs`. It has no expected-side
access to actual receipts or producer residuals.

### Bit-frozen mixed-route vector

The mixed-kind fixture is now a complete fixed regression rather than a coarse
order-invariance check. It combines:

- canopy throughfall and initial drainage with unequal amounts and
  temperatures;
- unequal upstream and downstream OFE areas;
- downstream local precipitation with partial support overlap;
- nonzero infiltration, surface retention, routed runoff and outlet runoff;
  and
- two chronological downstream mixture windows.

The fixture freezes the exact receipt sequence and, for every row, source ID,
basis OFE, canonical kind, disposition, complete typed recipient, support bits,
mass bits, mixture-temperature bits and enthalpy/Q bits. It therefore binds the
canonical per-window `h_mix,b`, proportional attribution, final-remainder
ownership and once-only unequal-area routing rather than merely proving that
two temperatures differ.

It also freezes both ending store masses, accepted transaction lineage and each
OFE's day, next interval, cumulative supply and cumulative infiltration bits.
Reversing caller ingress order produces byte-identical receipts, ending state
and closure operands.

## Ending-state context correction

`A-TERMINAL-CLOSURE9-HIGH-001` is corrected.

Owner, configuration and digest-wide failures use aggregate context with no
invented OFE, tile, surface or source identity. Store and continuation
membership/order checks use the shared membership-aware policy: deletions name
the expected missing key, additions/replacements retain an available offending
key, and reorders retain the actual first mismatching row. Value mismatches
retain the exact configured store or OFE.

The poison population covers missing, extra, duplicated, reordered and replaced
store/continuation rows, wrong values, cadence, lineage, aggregate owner and
digest failures. Canonical code, phase and applicable context are asserted.
Public candidate failure completion continues to attach beginning and attempted
rollback hashes; the independent validator itself remains read-only and cannot
mutate either owner state.

## Persistent endpoint and precedence

All closure8 endpoint and taxonomy corrections remain sound:

- Receipt-free replay returns final store values and final WB14 cumulative
  values for every key/OFE.
- The replayed endpoint joins directly to persistent ending records and
  continuations before digest reconstruction and strict state validation.
- A coordinated forged producer retained/ending operand cannot define the
  expected W1.
- Numeric/domain `E003` preflight runs exhaustively before structural `E009`
  producer reconstruction and independent `E010` comparison.
- Beginning cumulative infiltration may exceed neither cumulative supply nor
  infiltration-storage capacity, including zero-supply paths.
- Partition membership/order uses exact membership-aware `E009` context.
- Water density, liquid heat capacity and reference temperature are shared
  named constants; no closure-local physical literal can drift.

## Complete custody and historical-finding re-audit

No new material defect was found:

- Strict per-OFE/tile/surface/source persistent state, deterministic canonical
  restart bytes, digests and predecessor lineage remain enforced.
- One immutable beginning snapshot supplies typed requests and proportional
  maximum authorizations. Exact `0 <= F <= A <= D` remains independently
  reconstructed, finalized use alone debits storage, and unused authorization
  remains.
- Signed condensation credits the exact store before ingress and routes
  capacity overflow with exact mass, temperature and enthalpy identity.
- Open raw precipitation and covered canopy releases remain mutually
  exclusive. Each OFE executes one admitted stateful WB14 continuation per
  interval.
- Expected infiltration, retention, routed runoff and outlet runoff have zero
  access to actual receipts. Complete owner, source, origin/current store,
  recipient, basis, kind, support and disposition identity enters the join.
- Routed descendants become canonical `UpstreamRunon`, preserve source/origin
  lineage, use destination OFE/store identity, and apply unequal-area mass and
  energy conversion once. Existing multi-hop vectors remain passing.
- Raw `Q = mass * specific enthalpy`, canonical chronological `h_mix,b`,
  per-source mass/enthalpy, OFE aggregates, soil-liquid, soil-thermal and
  retained-LSE receipts remain independently reconstructed with checked
  arithmetic and contextual error precedence.
- Candidate construction is clone-only. Failure vectors retain canonical
  beginning/attempted hashes and byte-identical rollback.
- Snow, terminal snow, frozen and thawing states remain typed unsupported in
  the declared snow-free bridge.

## Production and campaign boundaries

The reviewed increment changes canonical internal identity helpers, independent
validation, tests and evidence. It does not alter the admitted WB14 equations,
accepted partition arithmetic, authorization rules, model identity or custody
state schema.

No runner selector, production scheduler reachability, default dispatch,
production output publication, runtime activation, calibration value or
consumer cutover was added. Production execution remains unchanged and the
bridge remains explicitly default-off. This PASS is for the custody
dependency-lift package; it is not a completion claim for held LSE Child 3 or
the parent campaign.

Line-count governance is truthful: the 2,759-line ingress test module is now
WARN with cohesion rationale and a specific fixture/vector split, while the
2,657-line closure and 2,014-line ingress modules retain explicit future split
intent. No affected file reaches the mandatory 3,000-line threshold.

## Commands run at the exact reviewed commit

```text
git rev-parse HEAD
PASS: f375349e38c134eb6eb105d03a87d85841ba8c26

git status --short --branch
PASS: clean main; 73 commits ahead of origin/main

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

`GO`: exact commit `f375349e38c134eb6eb105d03a87d85841ba8c26`
fully corrects the closure10 bounded finding and retains every historical
custody, WB14, D/A/F, condensation, routing, receiver, restart, rollback and
production-exclusion correction. No unresolved material hydrology, science,
ownership or evidence finding remains in this review scope. The package may
proceed to its separately required terminal verification and truthful
dependency-lift disposition before the held LSE runtime child resumes.
