# Review Agent B Terminal Closure 9 — Hydrology, Science, And Ownership

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `1242e4d382684dbeea1dad23735f6e6055c9c610`

Verdict: `PASS / persistent custody endpoint independently reconstructed and joined / no unresolved material hydrology, science, or ownership finding`.

This fresh review preserves every earlier finding and failed review as historical
evidence. It re-audits the complete snow-free surface-liquid custody endpoint,
with particular attention to
`B-TERMINAL-CLOSURE8-CRITICAL-001`, against the exact clean commit above.

## Closure 8 critical finding

`B-TERMINAL-CLOSURE8-CRITICAL-001` is completely corrected.

The receipt-free projection now retains, for every configured store and OFE:

- the ending tile-local liquid mass produced by the independent chronological
  replay;
- ending cumulative WB14 supply and infiltration;
- the independently calculated ending day and next-interval identity; and
- the accepted transaction identity.

`validate_projected_ending_state()` joins those values directly to the actual
persistent `DirectSurfaceLiquidOwnedState`. It requires exact configuration
record order and cardinality, exact store keys, bitwise-equal ending liquid,
exact OFE continuation order and cardinality, bitwise-equal cumulative values,
and exact accepted transaction lineage. It then recomputes the complete state
digest and invokes strict state validation. Digest or general state validation
cannot substitute for the endpoint joins because both occur only after every
projected store and continuation comparison passes.

The expected ending store is the local `store_liquid` map advanced by the
receipt-free replay. It does not consume producer receipts or the
producer-captured retained/ending fields. The coordinated poison that changes
both producer store-closure operands and the persistent ending store therefore
fails `E010` rather than becoming self-consistent evidence.

The expected continuation is likewise derived while replaying the frozen WB14
inputs. Beginning cumulative values come from the immutable beginning owner,
including the interval-48 reset rule; subsequent cumulative values come from
the independently executed chronological transitions. The ending cadence is
calculated from the immutable ingress day and interval during closure-operand
capture, separately from the production ending-state assignment. Public
candidate validation additionally reconstructs the producer candidate from the
original configuration, resource and ingress input before independent closure,
so coordinated mutation of a retained closure operand cannot bypass the
producer `E009` boundary.

The focused poison population rejects:

- wrong ending store with otherwise correct receipts;
- wrong cumulative supply or infiltration;
- wrong ending day or interval;
- stale continuation transaction identity;
- missing, duplicate, reordered or wrong-OFE continuation rows;
- coordinated forged retained/ending store operands; and
- a forged ending-state digest.

All validation is read-only over the candidate and beginning resource. Existing
failure tests retain canonical beginning and attempted hashes and prove
byte-identical rollback. No fallible mutation or partial owner commit was added.

## Full custody endpoint re-audit

No new material defect was found in the prior closure surfaces:

- Strict persistent per-OFE/tile/surface/source state, canonical restart bytes,
  configuration/state digests and predecessor lineage remain enforced.
- One immutable beginning snapshot supplies typed requests and proportional
  maximum authorizations. Exact `0 <= F <= A <= D` remains independently
  reconstructed, finalized use alone debits storage, and unused authorization
  remains available.
- Signed condensation credits the exact store before ingress, preserves mass
  and enthalpy identity, and routes capacity overflow as an ingress parcel.
- Open raw precipitation and covered accepted canopy releases remain mutually
  exclusive. Chronological source support, canonical source order and one
  shared stateful WB14 transition per OFE remain explicit.
- Expected infiltration, retention and runoff have zero access to actual
  receipts. Each source/window join retains owner, origin and current store,
  typed recipient, basis OFE, kind, support and disposition.
- Multi-hop runoff becomes canonical `UpstreamRunon`, uses the destination OFE
  and store, and applies the unequal-area mass/energy conversion exactly once.
- Raw and mixed enthalpy identities, canonical per-window `h_mix`, soil-liquid,
  soil-thermal and retained-LSE receivers, and OFE aggregate joins remain
  independently reconstructed with checked arithmetic and contextual `E003`
  precedence.
- Producer reconstruction remains ordered after exhaustive arithmetic/domain
  preflight and before independent `E010` closure, preserving canonical `E009`
  attribution without hiding later-record arithmetic failures.
- Snow, terminal-snow, frozen and thawing states remain typed unsupported for
  this declared snow-free bridge.

The remediation changes validation, frozen endpoint operands and tests only.
It does not change the shared WB14 constitutive transition, infiltration/runoff
physics, authorization arithmetic, accepted state equations or model identity.

## Production and campaign boundaries

The reviewed bytes add no runner selector, default dispatch, production
scheduler reachability, production output publication, runtime activation,
calibration value or consumer cutover. Production execution remains unchanged;
the custody bridge remains explicitly default-off. This review establishes the
dependency-lift endpoint only. It does not claim that held LSE Child 3 or the
parent integration campaign is complete.

`surface_liquid_closure.rs` is 2,441 lines. It remains below the mandatory
3,000-line split threshold, and the package records the specific future split
between frozen-operand projection and comparison/diagnostics required by WARN
governance.

## Commands run at the exact reviewed commit

```text
git rev-parse HEAD
PASS: 1242e4d382684dbeea1dad23735f6e6055c9c610

git status --short --branch
PASS: clean main; origin/main ahead count was 65

cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 52/52 selected; 507 skipped by the focused filter

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this review artifact was added
```

The exact checkpoint already retains the package-recorded complete 560/560
orchestrator run, 19/19 focused integration run, 9/9 authority run and earlier
exact-head full-workspace evidence. This review did not borrow those runs as a
replacement for its own focused endpoint and ownership checks.

## Approval statement

`GO`: exact commit `1242e4d382684dbeea1dad23735f6e6055c9c610`
completely closes `B-TERMINAL-CLOSURE8-CRITICAL-001` and retains all prior
custody, receiver, chronology, identity, arithmetic-precedence, rollback and
production-exclusion corrections. No unresolved material hydrology, science,
ownership or evidence finding remains in this review scope. The package may
proceed to the separately required terminal verification and truthful
dependency-lift disposition before the held LSE runtime child resumes.
