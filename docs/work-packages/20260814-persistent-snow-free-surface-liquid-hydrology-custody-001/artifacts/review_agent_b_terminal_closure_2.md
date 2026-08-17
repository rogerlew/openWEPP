# Review Agent B Terminal Closure 2 — Hydrology, Custody, And Science

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `1b76bd12ec5bb98b031c9fbeed3cd35d93afd597`

Verdict: `HOLD / one material global-E003 preflight omission / no authority HOLD`.

This fresh exact-byte review preserves all prior artifacts. It rechecks the
multi-record precedence remediation, exact later-record E009 attribution,
final E010 disposition, and the retained persistent hydrology custody, D/A/F,
ingress, receiver, WB14, rollback, restart, and production-exclusion surfaces.

## Material finding

### B-TERMINAL-CLOSURE2-HIGH-001 — the arithmetic preflight accumulates OFE enthalpy but omits its checked comparison

The new dedicated preflight correctly scans all store equations and most
parcel arithmetic without stopping at a finite mismatch. It also independently
accumulates `expected_ofe_enthalpy` and `actual_ofe_enthalpy`
(`direct_runtime/surface_liquid_closure.rs:785-821`). However, those maps are
never joined with `checked_surface_liquid_close()` before the preflight returns
at line 858.

The final independent validator does perform that missing comparison through
`require_close_enthalpy(actual_ofe_enthalpy, expected_ofe_enthalpy, ...)` at
lines 1121-1129. Consequently the claimed arithmetic-only preflight is not an
exhaustive projection of the arithmetic surfaces evaluated later.

This gap is reachable with finite operands. Two or more parcel components for
one OFE can each have actual and expected enthalpy small enough that their
individual comparison scales remain finite while their accumulated actual and
expected OFE totals are each large enough that the final comparison scale
overflows. The preflight accepts the finite accumulations and never tests that
scale. A simultaneous producer-field mismatch then returns E009 before the
later OFE comparison can return precedence-required E003. Likewise, an earlier
finite store/parcel E010 can still hide that later aggregate E003 in the final
short-circuiting validator.

The new combined poison covers an earlier store E010 plus a later store-equation
E003. It does not cover the omitted OFE aggregate comparison. The gate record's
claim that every OFE aggregate arithmetic surface is scanned is therefore
broader than the executable evidence.

Required correction:

1. After constructing the per-OFE actual and expected enthalpy maps, run the
   same unit-aware checked comparison for every relevant OFE. `None` must emit
   contextual E003 naming that OFE/store; `Some(true)` and `Some(false)` are
   ignored by the arithmetic-only preflight.
2. Prefer factoring shared arithmetic projections so the preflight and final
   validator cannot silently diverge. The current 300-line parallel
   transcription already omitted one final checked operation.
3. Add a nondegenerate multi-parcel same-OFE poison in which individual joins
   have finite comparison scales but the accumulated OFE comparison scale does
   not. Combine it separately with an E009 producer mismatch and an earlier
   finite E010, asserting global E003, exact OFE identity, and both rollback
   hashes.

This is a bounded implementation and test defect. It requires no contract
amendment, new model identity, changed tolerance, clamp, fallback, or new
package.

## Correctly closed portions of the remediation

- The new later-store combined poison proves that an earlier finite store
  mismatch no longer hides a later store-equation E003.
- Producer mismatch localization is now structural rather than a first-record
  fallback. Second ending-state, ledger, and WB14-call poisons report E009 with
  the actual later OFE and tile where applicable, plus exact rollback hashes.
- The wrong infiltration recipient remains E009 in `IngressCandidate` with
  exact receipt identity.
- When no arithmetic indeterminacy exists and producer fields reconstruct,
  finite independent mismatches remain the final E010 disposition.
- Checked comparison callers still distinguish `None` from `Some(false)`;
  receiver comparisons retain E003 versus E011 correctly.

## Retained hydrology and custody correctness

- One immutable beginning snapshot supplies one authorization batch. Exact
  transaction/requester/OFE/tile/source identities and normal
  `0 <= F <= A <= D` bounds remain intact; only finalized use debits water and
  unused authorization remains.
- Signed condensation remains a resource-phase credit before capacity
  overflow. Rain, canopy release, runon, and overflow remain post-authorization
  ingress and cannot satisfy the same authorization.
- Ingress preserves parcel source, recipient, temperature, enthalpy, area, and
  route identity. Tile/OFE and unequal-area route conversions remain checked
  and occur exactly once.
- Receiver depth and enthalpy division/aggregation remain checked. Production
  soil, soil thermal, and retained LSE candidates remain clone-only and are
  independently reconstructed from explicit operands.
- The bridge continues to use the single shared complete WB14 continuation;
  no copied infiltration/runoff transition was introduced.
- Persistent configuration/state digests, accepted transaction lineage,
  interval continuation, deterministic restart, and exact three-owner rollback
  remain intact.
- Snow, snow-retained liquid, frost, frozen-layer, and thawing entry remains
  contextual E004 before authorization or callback.
- The reviewed production change remains confined to closure/preflight and
  package evidence. Normal production constructors still disable the shadow;
  no runner selector, default dispatch, production scheduler, publication,
  runtime activation, or cutover was introduced.

## Commands run at the exact reviewed commit

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 39/39 selected; 507 skipped by filter

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this review artifact was added
```

These passing gates confirm the current later-store and later-record cases.
They do not exercise the missing per-OFE aggregate checked comparison.

## Approval statement

`NO-GO`: exact commit `1b76bd12e` correctly closes later-record E009
attribution and the demonstrated later-store E003 case, while retaining D/A/F,
ingress, checked receivers, WB14, rollback, restart, and production exclusion.
Dependency-package closure remains blocked because the arithmetic preflight
omits a checked comparison that the final validator evaluates, so global E003
precedence is not yet established across all multi-parcel/OFE operands.
